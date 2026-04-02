# Rethinking Transformations

The redesign addresses the limitations of the current design described in [The Current State](#the-current-state) by:

1. **Making backwards a static method** — `fn backward(artifact: &Artifact, solution: Solution)`
   forces all backwards state into the artifact

2. **Separating analysis and backwards state** — `AnalysisManager` for analyses,
   `Artifact` trait for backwards state

3. **Using a backwards registry** — Enables backwards execution without pass instances

4. **Type-safe analysis store** — `AnalysisKey<T>` provides compile-time type safety

5. **Open extension** — Any type can be an artifact, any pass can be implemented without
   modifying core types

6. **Forcing Python state externalization** — Python artifacts are pickled objects with
   their `backward()` method preserved

7. **CompilationRecord as first-class** — Serializable record of the transformation path,
   independent of PassManager

8. **Unified pass types** — IfElsePass is just a `ReversiblePass`, not a special case

This redesign maintains all the strengths while fixing the structural issues.

---

## Core Philosophy: Passes as Pure Transformations

Inspired by LLVM's pass infrastructure and MLIR's operation-centric design:

1. **Passes are stateless transformations** — all state needed for backwards is externalized
2. **Analysis and transformation are separate** — like LLVM's analysis/transform split
3. **Artifacts are the "backwards IR"** — they encode the inverse transformation
4. **Analysis preservation is explicit** — LLVM-style invalidation
5. **Extensibility through registration** — MLIR-style dialects/plugins

---

## 1. The Core Abstraction: Reversible Transformation

```rust
/// A reversible transformation pass.
/// 
/// The forward pass transforms a model and produces an artifact.
/// The artifact encodes everything needed to invert the transformation.
pub trait ReversiblePass: Send + Sync {
    /// The artifact type this pass produces.
    /// This is the "backwards IR" — it encodes the inverse transformation.
    type Artifact: Artifact;
    
    /// Unique identifier for this pass (e.g., "normalize_bounds")
    fn name(&self) -> &str;
    
    /// Forward transformation: Model → TransformedModel + Artifact
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> Result<Self::Artifact>;
    
    /// Inverse transformation: Solution + Artifact → InverseSolution
    /// 
    /// This is a static method — it doesn't need `&self`.
    /// All configuration is encoded in the Artifact itself.
    fn backward(artifact: &Self::Artifact, solution: Solution) -> Result<Solution>;
}
```

**Key insight:** `backward` is a **static method**. The artifact contains everything needed.
This is what makes serialization work: `Artifact` is data, `Pass` is code.

---

## 2. Analysis Pass Infrastructure (LLVM-Inspired)

```rust
/// An analysis pass computes information without transforming the model.
pub trait AnalysisPass: Send + Sync {
    /// The type of analysis result this pass produces
    type Result: Send + Sync + 'static;
    
    /// Unique identifier for this analysis
    fn name(&self) -> &str;
    
    /// Compute the analysis result
    fn run(&self, model: &Model, ctx: &PassContext) -> Result<Self::Result>;
    
    /// Which analyses does this analysis depend on?
    fn required_analyses(&self) -> &[&'static str] {
        &[]
    }
    
    /// Is this analysis invalidated by the given pass?
    fn is_invalidated_by(&self, pass_name: &str) -> bool;
}
```

---

## 3. Type-Safe, Open Analysis Store

```rust
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Typed key for accessing analysis results.
/// The type parameter ensures compile-time type safety.
pub struct AnalysisKey<T: 'static> {
    name: &'static str,
    _marker: PhantomData<fn() -> T>,
}

impl<T: 'static> AnalysisKey<T> {
    pub const fn new(name: &'static str) -> Self {
        Self { name, _marker: PhantomData }
    }
}

/// Type-safe analysis storage (LLVM's AnalysisManager equivalent)
pub struct AnalysisManager {
    results: HashMap<&'static str, Box<dyn Any + Send + Sync>>,
}

impl AnalysisManager {
    /// Get an analysis result (returns None if not computed)
    pub fn get<T: Send + Sync + 'static>(&self, key: &AnalysisKey<T>) -> Option<&T> {
        self.results
            .get(key.name)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
    
    /// Get an analysis result (error if not available)
    pub fn require<T: Send + Sync + 'static>(&self, key: &AnalysisKey<T>) -> Result<&T> {
        self.get(key).ok_or_else(|| {
            Error::MissingAnalysis { name: key.name }
        })
    }
    
    /// Store an analysis result
    pub fn insert<T: Send + Sync + 'static>(&mut self, key: &AnalysisKey<T>, value: T) {
        self.results.insert(key.name, Box::new(value));
    }
    
    /// Invalidate analyses affected by a transformation pass.
    /// `invalidates_by_pass["simplify"] = {"sparsity", "row_norms"}`.
    pub fn invalidate(
        &mut self,
        pass_name: &str,
        invalidates_by_pass: &HashMap<&'static str, HashSet<&'static str>>,
    ) {
        if let Some(invalidated) = invalidates_by_pass.get(pass_name) {
            self.results.retain(|name, _| !invalidated.contains(name));
        }
    }
}
```

**Usage example:**
```rust
// Define typed keys as module-level constants
pub static MAX_WEIGHT: AnalysisKey<f64> = AnalysisKey::new("max_weight");
pub static SPARSITY: AnalysisKey<SparsityInfo> = AnalysisKey::new("sparsity");

// Type-safe access
let max = analysis_manager.require(&MAX_WEIGHT)?; // &f64
let sparsity = analysis_manager.get(&SPARSITY);   // Option<&SparsityInfo>
```

---

## 4. The Artifact Trait: Backwards IR

```rust
/// An artifact encodes the inverse transformation.
/// 
/// This is serialized alongside the transformed model to enable backwards execution.
pub trait Artifact: Send + Sync + 'static {
    /// Unique type identifier for deserialization
    fn type_tag(&self) -> &'static str;
    
    /// Serialize this artifact
    fn serialize(&self) -> Result<Vec<u8>>;
    
    /// Deserialize this artifact type
    fn deserialize(bytes: &[u8]) -> Result<Self> where Self: Sized;
}

/// Type-erased artifact for storage in CompilationRecord
pub struct ErasedArtifact {
    type_tag: String,
    data: Vec<u8>,
}

impl ErasedArtifact {
    pub fn new<A: Artifact>(artifact: &A) -> Result<Self> {
        Ok(Self {
            type_tag: artifact.type_tag().to_string(),
            data: artifact.serialize()?,
        })
    }
    
    pub fn restore<A: Artifact>(&self) -> Result<A> {
        A::deserialize(&self.data)
    }
}
```

**Concrete example:**
```rust
/// Artifact for a bounds normalization pass
#[derive(Serialize, Deserialize)]
pub struct NormalizeBoundsArtifact {
    /// Original bounds before normalization
    original_bounds: Vec<(VarId, f64, f64)>,
    /// Scaling factors applied
    scale_factors: HashMap<VarId, f64>,
}

impl Artifact for NormalizeBoundsArtifact {
    fn type_tag(&self) -> &'static str {
        "lunamodel::normalize_bounds"
    }
    
    fn serialize(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }
    
    fn deserialize(bytes: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(bytes)?)
    }
}
```

---

## 5. Compilation Record: The Backwards Execution Trace

```rust
/// A record of the forward compilation, structured for backwards execution.
/// 
/// This is the serializable artifact of a full pass pipeline run.
pub struct CompilationRecord {
    /// The sequence of transformations applied, in forward order
    entries: Vec<PassEntry>,
}

/// A single entry in the compilation record
pub enum PassEntry {
    /// A transformation pass with its artifact
    Transform {
        pass_name: String,
        artifact: ErasedArtifact,
    },
    
    /// An analysis pass (no artifact, not reversed)
    Analysis {
        pass_name: String,
    },
    
    /// A nested sub-pipeline
    Pipeline {
        name: String,
        record: Box<CompilationRecord>,
    },
}

impl CompilationRecord {
    /// Execute backwards transformation
    /// 
    /// This is a standalone function that doesn't need the original PassManager.
    /// All information is encoded in the artifacts.
    pub fn backward(&self, solution: Solution) -> Result<Solution> {
        let mut sol = solution;
        
        // Reverse order: last transformation first
        for entry in self.entries.iter().rev() {
            sol = match entry {
                PassEntry::Transform { pass_name, artifact } => {
                    // Look up the backwards function and apply it
                    backward_registry::apply(pass_name, artifact, sol)?
                }
                
                PassEntry::Analysis { .. } => {
                    // Analysis passes don't affect backwards
                    sol
                }
                
                PassEntry::Pipeline { record, .. } => {
                    // Recursively apply backwards through sub-pipeline
                    record.backward(sol)?
                }
            };
        }
        
        Ok(sol)
    }
    
    /// Serialize this compilation record
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(Into::into)
    }
    
    /// Deserialize a compilation record
    pub fn deserialize(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(Into::into)
    }
}
```

---

## 6. Backwards Registry: Decoupling Code from Data

The key to making backwards work after deserialization is the **backwards registry**.

```rust
/// Global registry of backwards functions.
/// 
/// Each pass type registers its backwards function at program startup.
pub struct BackwardRegistry {
    functions: HashMap<String, BackwardFn>,
}

type BackwardFn = fn(&ErasedArtifact, Solution) -> Result<Solution>;

impl BackwardRegistry {
    /// Register a backwards function for a pass type
    pub fn register<P: ReversiblePass>(&mut self, pass_name: &str) {
        self.functions.insert(
            pass_name.to_string(),
            |artifact, solution| {
                let typed_artifact = artifact.restore::<P::Artifact>()?;
                P::backward(&typed_artifact, solution)
            }
        );
    }
    
    /// Apply the backwards transformation for a pass
    pub fn apply(
        &self,
        pass_name: &str,
        artifact: &ErasedArtifact,
        solution: Solution
    ) -> Result<Solution> {
        let backward_fn = self.functions
            .get(pass_name)
            .ok_or_else(|| Error::UnregisteredPass { name: pass_name.to_string() })?;
        
        backward_fn(artifact, solution)
    }
}

/// Global singleton registry
static BACKWARD_REGISTRY: OnceLock<Mutex<BackwardRegistry>> = OnceLock::new();

pub fn register_backward<P: ReversiblePass>(pass_name: &str) {
    BACKWARD_REGISTRY
        .get_or_init(|| Mutex::new(BackwardRegistry::default()))
        .lock()
        .unwrap()
        .register::<P>(pass_name);
}
```

**Usage:**
```rust
// At program startup or module init
register_backward::<NormalizeBoundsPass>("normalize_bounds");
register_backward::<SimplifyConstraintsPass>("simplify_constraints");

// Later, after deserialization:
let record = CompilationRecord::deserialize(&bytes)?;
let original_solution = record.backward(solver_solution)?;
// ✅ Works! No PassManager needed.
```

---

## 7. PassManager: Orchestrating the Pipeline

```rust
/// Object-safe erased transform pass used by the pipeline runtime.
pub trait ErasedTransformPass: Send + Sync {
    fn name(&self) -> &'static str;
    fn forward_erased(&self, model: &mut Model, ctx: &PassContext) -> Result<ErasedArtifact>;
}

/// Typed pass can be wrapped into ErasedTransformPass.
impl<P> ErasedTransformPass for P
where
    P: ReversiblePass + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        self.name()
    }

    fn forward_erased(&self, model: &mut Model, ctx: &PassContext) -> Result<ErasedArtifact> {
        let artifact = self.forward(model, ctx)?;
        ErasedArtifact::new(&artifact)
    }
}

pub trait ErasedAnalysisPass: Send + Sync {
    fn name(&self) -> &'static str;
    fn run_erased(&self, model: &Model, ctx: &PassContext, analyses: &mut AnalysisManager)
        -> Result<()>;
}

pub enum PipelineStep {
    Transform(Arc<dyn ErasedTransformPass>),
    Analysis(Arc<dyn ErasedAnalysisPass>),
    Pipeline { name: String, passes: Vec<PipelineStep> },
}

// Note: PipelineStep is intentionally Arc-backed so `from_steps(steps.clone())`
// is cheap and does not require cloning non-cloneable closures or trait objects.

pub struct PassManager {
    passes: Vec<PipelineStep>,
    analysis_manager: AnalysisManager,
    invalidates_by_pass: HashMap<&'static str, HashSet<&'static str>>,
}

impl PassManager {
    pub fn run(&mut self, model: &mut Model) -> Result<CompilationRecord> {
        let mut entries = Vec::new();
        for step in &self.passes {
            match step {
                PipelineStep::Transform(pass) => {
                    let ctx = PassContext::new(&self.analysis_manager);
                    let artifact = pass.forward_erased(model, &ctx)?;
                    entries.push(PassEntry::Transform {
                        pass_name: pass.name().to_string(),
                        artifact,
                    });
                    self.analysis_manager
                        .invalidate(pass.name(), &self.invalidates_by_pass);
                }
                PipelineStep::Analysis(pass) => {
                    let ctx = PassContext::new(&self.analysis_manager);
                    pass.run_erased(model, &ctx, &mut self.analysis_manager)?;
                    entries.push(PassEntry::Analysis {
                        pass_name: pass.name().to_string(),
                    });
                }
                PipelineStep::Pipeline { name, passes } => {
                    let mut sub_manager = PassManager {
                        passes: passes.clone(),
                        analysis_manager: self.analysis_manager.clone(),
                        invalidates_by_pass: self.invalidates_by_pass.clone(),
                    };
                    let sub_record = sub_manager.run(model)?;
                    entries.push(PassEntry::Pipeline {
                        name: name.clone(),
                        record: Box::new(sub_record),
                    });
                }
            }
        }
        Ok(CompilationRecord { entries })
    }
}
```

---

## 8. PassContext: Dependency Injection for Passes

```rust
/// Context provided to passes during execution.
/// 
/// This is how passes access analyses and other infrastructure.
pub struct PassContext<'a> {
    analysis_manager: &'a AnalysisManager,
}

impl<'a> PassContext<'a> {
    pub fn new(analysis_manager: &'a AnalysisManager) -> Self {
        Self { analysis_manager }
    }
    
    /// Get an analysis result
    pub fn get_analysis<T: 'static>(&self, key: &AnalysisKey<T>) -> Option<&T> {
        self.analysis_manager.get(key)
    }
    
    /// Require an analysis result (error if missing)
    pub fn require_analysis<T: 'static>(&self, key: &AnalysisKey<T>) -> Result<&T> {
        self.analysis_manager.require(key)
    }
}
```

---

## 9. Python Integration: Clean Adapter Pattern

```rust
/// Adapter for Python transformation passes
pub struct PyReversiblePassAdapter {
    inner: Py<PyAny>,
    name: String,
}

impl ReversiblePass for PyReversiblePassAdapter {
    type Artifact = PyArtifact;
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> Result<Self::Artifact> {
        Python::with_gil(|py| {
            let py_model = model.to_python(py)?;
            let py_ctx = ctx.to_python(py)?;
            
            // Call Python forward method
            let result = self.inner.call_method1(py, "forward", (py_model, py_ctx))?;
            
            // Extract artifact
            let artifact: &PyArtifact = result.extract(py)?;
            Ok(artifact.clone())
        })
    }
    
    fn backward(artifact: &PyArtifact, solution: Solution) -> Result<Solution> {
        // This is a static method, so we need the backward function stored in the artifact
        artifact.apply_backward(solution)
    }
}

/// Python artifact wrapper
#[pyclass]
pub struct PyArtifact {
    /// Pickled Python object
    data: Vec<u8>,
}

impl Artifact for PyArtifact {
    fn type_tag(&self) -> &'static str {
        "python_artifact"
    }
    
    fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(Into::into)
    }
    
    fn deserialize(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(Into::into)
    }
}

impl PyArtifact {
    fn apply_backward(&self, solution: Solution) -> Result<Solution> {
        Python::with_gil(|py| {
            // Unpickle the artifact
            let pickle = py.import("pickle")?;
            let artifact_obj = pickle
                .call_method1("loads", (PyBytes::new(py, &self.data),))?;
            
            // Call its backward method
            let py_solution = solution.to_python(py)?;
            let result = artifact_obj.call_method1("backward", (py_solution,))?;
            
            Solution::from_python(result)
        })
    }
}
```

> Security note: `pickle.loads(...)` must only be used with trusted artifacts. For untrusted
> inputs, prefer a restricted serialization format (e.g., msgpack/json schema + explicit
> artifact class registry).

**Python protocol:**
```python
from abc import ABC, abstractmethod
import pickle

class ReversiblePass(ABC):
    """Base class for Python reversible passes."""
    
    @abstractmethod
    def forward(self, model, ctx) -> 'Artifact':
        """Apply the forward transformation."""
        pass

class Artifact(ABC):
    """Base class for Python artifacts."""
    
    @abstractmethod
    def backward(self, solution):
        """Apply the backward transformation."""
        pass
    
    def __getstate__(self):
        """Called during pickling."""
        return self.__dict__
    
    def __setstate__(self, state):
        """Called during unpickling."""
        self.__dict__.update(state)

# Example usage:
class NormalizeBoundsPass(ReversiblePass):
    def forward(self, model, ctx):
        # Transform model and create artifact
        original_bounds = [(v.id, v.lower, v.upper) for v in model.variables]
        
        for var in model.variables:
            var.lower = 0.0
            var.upper = 1.0
        
        return NormalizeBoundsArtifact(original_bounds)

class NormalizeBoundsArtifact(Artifact):
    def __init__(self, original_bounds):
        self.original_bounds = original_bounds
    
    def backward(self, solution):
        # Map solution values back to original bounds
        for (var_id, lower, upper), value in zip(self.original_bounds, solution.values):
            solution.values[var_id] = lower + value * (upper - lower)
        return solution
```

---

## 10. Complete Usage Example

```rust
use lunamodel_transform::*;

fn main() -> Result<()> {
    // Register backward functions at startup
    register_backward::<NormalizeBoundsPass>("normalize_bounds");
    register_backward::<SimplifyConstraintsPass>("simplify_constraints");
    
    // Build the pass pipeline
    let mut pipeline = PassManager::new()
        .add_analysis(ComputeSparsityPass)
        .add_transform(NormalizeBoundsPass::new())
        .add_transform(SimplifyConstraintsPass::new())
        .add_analysis(ValidateModelPass);
    
    // Load model
    let mut model = Model::load("input.lm")?;
    
    // Run forward pass
    let record = pipeline.run(&mut model)?;
    
    // Save transformed model and record
    model.save("transformed.lm")?;
    let record_bytes = record.serialize()?;
    std::fs::write("compilation.record", &record_bytes)?;
    
    // ... solve the transformed model ...
    let solver_solution = solve(&model)?;
    
    // === Later, in a different process ===
    
    // Load the compilation record
    let record_bytes = std::fs::read("compilation.record")?;
    let record = CompilationRecord::deserialize(&record_bytes)?;
    
    // Execute backwards (no PassManager needed!)
    let original_solution = record.backward(solver_solution)?;
    
    Ok(())
}
```

---

## 11. Key Design Decisions

### Static `backward()` Method

The backwards transformation is a **static method** on the pass trait. This enforces that
all state needed for backwards is in the artifact, not in the pass configuration.

```rust
// ✅ Good: static method forces externalization
fn backward(artifact: &Artifact, solution: Solution) -> Solution;

// ❌ Bad: instance method allows hidden state
fn backward(&self, artifact: &Artifact, solution: Solution) -> Solution;
```

### Artifact is Serializable, Pass is Not

The `PassManager` is code — you write it in Rust or build it dynamically in Python. It's
not serialized. Only the `CompilationRecord` (containing artifacts) is serialized.

This separation is what makes the architecture clean: **data** (CompilationRecord) is
separate from **code** (PassManager).

### Backwards Registry for Late Binding

The backwards registry allows the `CompilationRecord` to invoke backwards functions
without having the original `Pass` instances. This is essential for deserialization.

```rust
// At startup:
register_backward::<NormalizeBoundsPass>("normalize_bounds");

// Later, after deserialization:
record.backward(solution)?;  // ✅ Works via registry lookup
```

### Type-Safe Analysis Store

The `AnalysisKey<T>` pattern gives compile-time type safety for analysis access while
remaining open for extension. No closed enum required.

```rust
// Define typed keys
static MAX_WEIGHT: AnalysisKey<f64> = AnalysisKey::new("max_weight");

// Type-safe access
let max: &f64 = ctx.require_analysis(&MAX_WEIGHT)?;  // ✅ Type checked
```

### Python Artifacts Store Backward Function

Python artifacts contain both data and the backward logic (via pickling the whole object).
This works because Python objects can be serialized with their methods intact.

```python
class MyArtifact(Artifact):
    def __init__(self, data):
        self.data = data
    
    def backward(self, solution):
        # This method is preserved through pickle!
        return transform(solution, self.data)
```

---

## 12. What This Design Enables

### ✅ Serialization of Backwards Path

```rust
let record = pipeline.run(&mut model)?;
let bytes = record.serialize()?;

// Later...
let record = CompilationRecord::deserialize(&bytes)?;
record.backward(solution)?;  // ✅ Works!
```

### ✅ Type-Safe Extension

```rust
// Define your own artifact type
#[derive(Serialize, Deserialize)]
struct MyArtifact { /* ... */ }

impl Artifact for MyArtifact { /* ... */ }

// Define your own pass
struct MyPass;

impl ReversiblePass for MyPass {
    type Artifact = MyArtifact;
    // ...
}

// Register and use
register_backward::<MyPass>("my_pass");
```

### ✅ Clean Python Integration

```python
class MyPass(ReversiblePass):
    def forward(self, model, ctx):
        # Transform and return artifact
        return MyArtifact(...)

class MyArtifact(Artifact):
    def backward(self, solution):
        # Inverse transformation
        return transformed_solution
```

### ✅ Analysis Preservation (LLVM-style)

```rust
impl AnalysisPass for SparsityAnalysis {
    fn is_invalidated_by(&self, pass_name: &str) -> bool {
        match pass_name {
            "add_constraint" | "remove_variable" => true,
            _ => false,
        }
    }
}
```

### ✅ Composable Pipelines

```rust
let pipeline = PassManager::new()
    .add_transform(Pass1)
    .add_pipeline("optimization", vec![
        PipelineStep::Transform(Box::new(Pass2)),
        PipelineStep::Transform(Box::new(Pass3)),
    ])
    .add_transform(Pass4);
```

---

## 13. Crate Structure

```
crates/transform/
├── src/
│   ├── lib.rs                      — Public API exports
│   ├── pass.rs                     — ReversiblePass and AnalysisPass traits
│   ├── artifact.rs                 — Artifact trait and ErasedArtifact
│   ├── analysis.rs                 — AnalysisManager and AnalysisKey
│   ├── record.rs                   — CompilationRecord and PassEntry
│   ├── registry.rs                 — BackwardRegistry
│   ├── manager.rs                  — PassManager
│   ├── context.rs                  — PassContext
│   │
│   ├── passes/
│   │   ├── mod.rs
│   │   ├── normalize_bounds.rs     — Example transformation pass
│   │   ├── simplify.rs             — Example transformation pass
│   │   └── sparsity.rs             — Example analysis pass
│   │
│   └── python/
│       ├── mod.rs
│       ├── adapter.rs              — PyReversiblePassAdapter
│       ├── artifact.rs             — PyArtifact
│       └── bindings.rs             — PyO3 bindings
│
└── Cargo.toml
```

---

## 14. Migration Strategy

1. **Create new crate** with clean-slate design
2. **Implement core infrastructure** (pass traits, artifacts, registry, manager)
3. **Port existing passes** one by one to new traits
4. **Add Python bindings** with clean adapter pattern
5. **Update consumers** to use new API
6. **Delete old crate** once migration is complete

The new API is simpler:
```rust
// Old API
let mut ir = IR::new();
manager.run(&model, &mut ir)?;
let solution = solve(&ir.model)?;
manager.backwards(&solution, &ir)?;

// New API
let record = manager.run(&mut model)?;
let solution = solve(&model)?;
record.backward(solution)?;
```

---

## 15. Summary: Core Principles

1. **Passes are pure transformations** — `backward()` is static
2. **Artifacts are first-class IR** — serializable, typed, explicit
3. **Analysis is separate from transformation** — LLVM-style preservation
4. **Type safety everywhere** — `AnalysisKey<T>`, associated types
5. **Backwards registry for late binding** — enables deserialization
6. **Python artifacts are pickled objects** — methods preserved
7. **PassManager is code, CompilationRecord is data** — clear separation

This design follows compiler engineering best practices from LLVM and MLIR while being
idiomatic Rust and supporting clean Python integration.

---

## 16. Advanced Pass Types: IfElsePass and Beyond

### The IfElsePass Pattern

The `IfElsePass` is a **conditional branching pass** that evaluates a predicate and takes
one of two transformation paths. This is essential for optimization pipelines.

**Key insight:** IfElsePass is just a `ReversiblePass` whose artifact records which branch was taken!

```rust
/// A conditional pass that branches based on a predicate.
pub struct IfElsePass<P: Fn(&Model, &PassContext) -> Result<bool>> {
    name: String,
    predicate: P,
    then_branch: Vec<PipelineStep>,
    else_branch: Vec<PipelineStep>,
}

/// Artifact records which branch was taken and its compilation record
#[derive(Serialize, Deserialize)]
pub struct IfElseArtifact {
    /// Which branch was executed
    branch_taken: Branch,
    /// The compilation record from that branch
    branch_record: CompilationRecord,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
enum Branch {
    Then,
    Else,
}

impl<P> ReversiblePass for IfElsePass<P>
where
    P: Fn(&Model, &PassContext) -> Result<bool> + Send + Sync,
{
    type Artifact = IfElseArtifact;
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> Result<Self::Artifact> {
        // Evaluate predicate
        let condition = (self.predicate)(model, ctx)?;
        
        // Execute the appropriate branch
        let (branch_taken, branch_passes) = if condition {
            (Branch::Then, &self.then_branch)
        } else {
            (Branch::Else, &self.else_branch)
        };
        
        // Run the branch as a sub-pipeline
        let mut branch_manager = PassManager::from_steps(branch_passes.clone());
        let branch_record = branch_manager.run(model)?;
        
        Ok(IfElseArtifact {
            branch_taken,
            branch_record,
        })
    }
    
    fn backward(artifact: &Self::Artifact, solution: Solution) -> Result<Solution> {
        // Backwards just delegates to the branch that was taken
        artifact.branch_record.backward(solution)
    }
}

impl Artifact for IfElseArtifact {
    fn type_tag(&self) -> &'static str {
        "lunamodel::if_else"
    }
    
    fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(Into::into)
    }
    
    fn deserialize(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes).map_err(Into::into)
    }
}
```

**Usage example:**
```rust
let pipeline = PassManager::new()
    .add_transform(NormalizeBoundsPass)
    .add_if_else(
        "optimize_if_sparse",
        |model, ctx| {
            let sparsity = ctx.require_analysis(&SPARSITY_RATIO)?;
            Ok(*sparsity > 0.7)  // If more than 70% sparse
        },
        // Then branch: use sparse-optimized passes
        vec![
            PipelineStep::Transform(Box::new(SparseSimplifyPass)),
            PipelineStep::Transform(Box::new(SparseNormalizePass)),
        ],
        // Else branch: use dense passes
        vec![
            PipelineStep::Transform(Box::new(DenseSimplifyPass)),
            PipelineStep::Transform(Box::new(DenseNormalizePass)),
        ],
    )
    .add_transform(FinalizePass);

let record = pipeline.run(&mut model)?;

// Backwards automatically follows the branch that was taken!
let original_solution = record.backward(solver_solution)?;
```

### Why This Works Perfectly

1. **The artifact records the decision**: `branch_taken: Branch`
2. **The artifact contains the sub-pipeline's record**: `branch_record: CompilationRecord`
3. **Backwards just delegates**: No special logic needed
4. **Serialization is automatic**: Everything is already serializable
5. **Type-safe**: Still uses the same `ReversiblePass` trait

---

### Other Advanced Pass Types

The design naturally supports many advanced patterns:

#### 1. **WhilePass** — Iterate Until Convergence

```rust
pub struct WhilePass<P: Fn(&Model, &PassContext) -> Result<bool>> {
    name: String,
    condition: P,
    max_iterations: usize,
    body: Vec<PipelineStep>,
}

#[derive(Serialize, Deserialize)]
pub struct WhileArtifact {
    /// Records from each iteration
    iteration_records: Vec<CompilationRecord>,
}

impl<P> ReversiblePass for WhilePass<P>
where
    P: Fn(&Model, &PassContext) -> Result<bool> + Send + Sync,
{
    type Artifact = WhileArtifact;
    
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> Result<Self::Artifact> {
        let mut iteration_records = Vec::new();
        
        for _ in 0..self.max_iterations {
            if !(self.condition)(model, ctx)? {
                break;
            }
            
            let mut iteration_manager = PassManager::from_steps(self.body.clone());
            let record = iteration_manager.run(model)?;
            iteration_records.push(record);
        }
        
        Ok(WhileArtifact { iteration_records })
    }
    
    fn backward(artifact: &Self::Artifact, solution: Solution) -> Result<Solution> {
        // Backwards through iterations in reverse order
        let mut sol = solution;
        for record in artifact.iteration_records.iter().rev() {
            sol = record.backward(sol)?;
        }
        Ok(sol)
    }
}
```

**Usage:**
```rust
.add_while(
    "simplify_until_stable",
    |model, ctx| {
        let prev_size = ctx.get_analysis(&MODEL_SIZE)?;
        let curr_size = model.constraint_count();
        Ok(prev_size.map_or(true, |&prev| curr_size < prev))
    },
    5,  // max iterations
    vec![
        PipelineStep::Analysis(Box::new(ComputeModelSizePass)),
        PipelineStep::Transform(Box::new(SimplifyPass)),
    ],
)
```

#### 2. **SwitchPass** — Multi-Way Branch

```rust
pub struct SwitchPass<K: Eq + Hash + Serialize + DeserializeOwned> {
    name: String,
    selector: Box<dyn Fn(&Model, &PassContext) -> Result<K>>,
    cases: HashMap<K, Vec<PipelineStep>>,
    default: Vec<PipelineStep>,
}

#[derive(Serialize, Deserialize)]
pub struct SwitchArtifact<K> {
    case_selected: K,
    case_record: CompilationRecord,
}

impl<K> ReversiblePass for SwitchPass<K>
where
    K: Eq + Hash + Clone + Serialize + DeserializeOwned + Send + Sync + 'static,
{
    type Artifact = SwitchArtifact<K>;
    
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> Result<Self::Artifact> {
        let case_selected = (self.selector)(model, ctx)?;
        
        let steps = self.cases
            .get(&case_selected)
            .unwrap_or(&self.default);
        
        let mut manager = PassManager::from_steps(steps.clone());
        let case_record = manager.run(model)?;
        
        Ok(SwitchArtifact {
            case_selected,
            case_record,
        })
    }
    
    fn backward(artifact: &Self::Artifact, solution: Solution) -> Result<Solution> {
        artifact.case_record.backward(solution)
    }
}
```

**Usage:**
```rust
.add_switch(
    "optimize_by_problem_type",
    |model, ctx| {
        Ok(match model.objective_type() {
            ObjectiveType::Linear => ProblemClass::LP,
            ObjectiveType::Quadratic => ProblemClass::QP,
            ObjectiveType::Nonlinear => ProblemClass::NLP,
        })
    },
    [
        (ProblemClass::LP, vec![/* LP-specific passes */]),
        (ProblemClass::QP, vec![/* QP-specific passes */]),
        (ProblemClass::NLP, vec![/* NLP-specific passes */]),
    ].into(),
    vec![/* default passes */],
)
```

#### 3. **TryCatchPass** — Error Recovery

```rust
pub struct TryCatchPass {
    name: String,
    try_branch: Vec<PipelineStep>,
    catch_branch: Vec<PipelineStep>,
}

#[derive(Serialize, Deserialize)]
pub enum TryCatchArtifact {
    Success(CompilationRecord),
    Recovered(CompilationRecord),
}

impl ReversiblePass for TryCatchPass {
    type Artifact = TryCatchArtifact;
    
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> Result<Self::Artifact> {
        let mut model_backup = model.clone();
        let mut try_manager = PassManager::from_steps(self.try_branch.clone());
        
        match try_manager.run(model) {
            Ok(record) => Ok(TryCatchArtifact::Success(record)),
            Err(_) => {
                // Restore model and try catch branch
                *model = model_backup;
                let mut catch_manager = PassManager::from_steps(self.catch_branch.clone());
                let record = catch_manager.run(model)?;
                Ok(TryCatchArtifact::Recovered(record))
            }
        }
    }
    
    fn backward(artifact: &Self::Artifact, solution: Solution) -> Result<Solution> {
        match artifact {
            TryCatchArtifact::Success(record) => record.backward(solution),
            TryCatchArtifact::Recovered(record) => record.backward(solution),
        }
    }
}
```

#### 4. **ParallelPass** — Fork-Join Transformations

```rust
pub struct ParallelPass {
    name: String,
    branches: Vec<Vec<PipelineStep>>,
    join: Box<dyn Fn(Vec<Model>) -> Result<Model>>,
}

#[derive(Serialize, Deserialize)]
pub struct ParallelArtifact {
    branch_records: Vec<CompilationRecord>,
    // Metadata about how branches were joined
    join_info: Vec<u8>,
}

// This is more complex — would need to handle solution splitting/merging
```

---

### Adding New Pass Types: The Extension Pattern

**The beauty of this design:** Any new pass type is just a struct implementing `ReversiblePass`!

```rust
// Want a new pass type? Just implement the trait!
pub struct MyCustomPass {
    config: MyConfig,
}

#[derive(Serialize, Deserialize)]
pub struct MyCustomArtifact {
    // Whatever backwards needs
}

impl ReversiblePass for MyCustomPass {
    type Artifact = MyCustomArtifact;
    
    fn forward(&self, model: &mut Model, ctx: &PassContext) -> Result<Self::Artifact> {
        // Do something interesting
        // Return artifact with backwards state
    }
    
    fn backward(artifact: &Self::Artifact, solution: Solution) -> Result<Solution> {
        // Invert the transformation
    }
}

// Register at startup
register_backward::<MyCustomPass>("my_custom_pass");

// Use it!
pipeline.add_transform(MyCustomPass { config });
```

**No core crate modifications needed!** The trait system handles everything.

---

### Python Custom Passes

Python users can also create custom pass types:

```python
class IfElsePass(ReversiblePass):
    """Python version of IfElsePass"""
    
    def __init__(self, name, predicate, then_branch, else_branch):
        self.name = name
        self.predicate = predicate
        self.then_branch = then_branch
        self.else_branch = else_branch
    
    def forward(self, model, ctx):
        condition = self.predicate(model, ctx)
        
        if condition:
            branch = self.then_branch
            branch_name = "then"
        else:
            branch = self.else_branch
            branch_name = "else"
        
        # Run the branch
        sub_manager = PassManager(branch)
        record = sub_manager.run(model)
        
        return IfElseArtifact(branch_name, record)

class IfElseArtifact(Artifact):
    def __init__(self, branch_taken, branch_record):
        self.branch_taken = branch_taken
        self.branch_record = branch_record
    
    def backward(self, solution):
        return self.branch_record.backward(solution)
```

---

### Design Principles for Special Passes

When adding a new pass type, follow these principles:

1. **Artifact captures decisions**: Record which path was taken
2. **Artifact contains sub-records**: For branching/looping, store the `CompilationRecord`s
3. **Backward delegates**: Most special passes just delegate to sub-records
4. **Configuration in the pass**: Predicates, selectors, etc. are in the pass struct
5. **State in the artifact**: What happened is in the artifact

**Example structure:**
```rust
// Pass = Code (configuration, predicates, branches)
pub struct SpecialPass {
    predicate: Box<dyn Fn(&Model) -> bool>,  // ← Configuration
    branches: Vec<Vec<PipelineStep>>,         // ← Code structure
}

// Artifact = Data (what happened, sub-records)
pub struct SpecialArtifact {
    which_path: PathId,           // ← What happened
    path_record: CompilationRecord,  // ← How to reverse it
}
```

---

### Comparison to LLVM PassManager

| Feature | LLVM | This Design |
|---------|------|-------------|
| Pass types | Function, Module, Loop, Region | Reversible, Analysis, IfElse, While, etc. |
| Conditional execution | Manual `if` in pass code | First-class `IfElsePass` |
| Iteration | `LoopPass` base class | `WhilePass` with artifact tracking |
| Analysis caching | `AnalysisManager` | `AnalysisManager` with typed keys |
| Serialization | Not supported | **Full support via artifacts** ✨ |

The key advantage: **we borrow LLVM/MLIR pass-management principles while adding serializable backwards execution for this domain.**

---

### Summary: Extensibility

✅ **IfElsePass** — Yes, it's just a `ReversiblePass`  
✅ **WhilePass** — Yes, artifact stores iteration history  
✅ **SwitchPass** — Yes, artifact stores selected case  
✅ **TryCatchPass** — Yes, artifact stores success/recovery path  
✅ **Custom passes** — Yes, implement `ReversiblePass` trait  
✅ **Python custom passes** — Yes, same trait-based pattern  

**The trait-based design is fully open for extension without modification of core types.**

---

## The Current State

## 1. High-Level Architecture

The transformation pipeline transforms a `Model` into a different representation and tracks enough information to invert the transformation when a `Solution` is available.

```
┌─────────────┐
│ PassManager │  Orchestrates pass execution
└──────┬──────┘
       │ .run(model)
       ▼
┌─────────────┐
│     IR      │  Intermediate Representation (output of forward pass)
│  ┌───────┐  │
│  │ Model │  │  Transformed model
│  ├───────┤  │
│  │ Cache │  │  Analysis results + backwards state
│  ├───────┤  │
│  │  Log  │  │  Execution trace
│  └───────┘  │
└──────┬──────┘
       │ .backwards(solution, ir)
       ▼
┌─────────────┐
│  Solution   │  Original solution (for input model)
└─────────────┘
```

**Forward Pass:**
```rust
let pass_manager = PassManager::new(passes);
let ir = pass_manager.run(model)?;
// ir.model is the transformed model
```

**Backward Pass:**
```rust
let solver_solution = solve(&ir.model)?;
let original_solution = pass_manager.backwards(solver_solution, &ir)?;
```

---

## 2. core components

### 2.1 the `ir` struct

```rust
pub struct ir {
    pub model: model,              // transformed model
    pub cache: analysiscache,      // analysis results + backwards state
    pub execution_log: executionlog, // execution trace
    pub input_model: option<model>, // original input (for validation)
}
```

the `ir` is the output of the forward pass. it contains:
- **transformed model** — the model to be solved
- **analysis cache** — stores analysis results and backwards state
- **execution log** — records what each pass did (for backwards navigation)
- **input model** — deep clone of original (for solution validation)

**problem:** the `ir` conflates three concerns: the output model, analysis results, and
backwards state. these should be separate.

---

### 2.2 The `AnalysisCache`

```rust
pub struct AnalysisCache {
    cache: IndexMap<String, AnalysisCacheElement>,
}
```

A string-keyed map of analysis results. The keys are pass names.

**The Cache Element Enum:**
```rust
pub enum AnalysisCacheElement {
    IfElseInfoAnalysis(IfElseInfo),
    MaxBiasAnalysis(MaxBias),
    BinarySpinInfoAnalysis(BinarySpinInfo),
    MinValueInConstraintAnalysis(MinConstraintValues),
    SpecsAnalysis(Specs),
    General(Vec<String>),
    IntegerToBinaryInfoAnalysis(IntegerToBinaryInfo),
    #[cfg(feature = "py")]
    PyAnalysis(Py<PyAny>),
}
```

**Problems:**

1. **Closed Enum** — Adding a new pass type requires modifying this core enum.
   Every new pass must add a variant here, coupling all passes to the core crate.

2. **Mixed Purposes** — The enum stores both analysis results AND backwards state.
   For example:
   - `MaxBiasAnalysis(MaxBias)` — pure analysis result
   - `BinarySpinInfoAnalysis(BinarySpinInfo)` — backwards state for `BinarySpinPass`
   - `IfElseInfoAnalysis(IfElseInfo)` — records which branch was taken (backwards state)

3. **Untyped Access** — Passes access the cache by string keys:
   ```rust
   cache.get("binary-spin")  // Returns Option<&AnalysisCacheElement>
   ```
   Then they must match on the enum variant to extract the typed value. No compile-time
   type safety.

4. **Python GIL Dependency** — The `PyAnalysis(Py<PyAny>)` variant breaks normal `Clone`:
   ```rust
   impl Clone for AnalysisCacheElement {
       fn clone(&self) -> Self {
           match self {
               // ... normal clones ...
               #[cfg(feature = "py")]
               AnalysisCacheElement::PyAnalysis(v) => {
                   Python::attach(|py| AnalysisCacheElement::PyAnalysis(v.clone_ref(py)))
               }
           }
       }
   }
   ```
   A special `clone_py(&self, py: Python)` method is needed in Python contexts, which
   leaks GIL-awareness throughout the codebase.

---

### 2.3 The `ExecutionLog`

```rust
pub struct ExecutionLog {
    log: Vec<LogElement>,
}

pub struct LogElement {
    pub pass: String,            // Pass name
    pub timing: Timing,          // Execution time
    pub kind: ActionType,        // What the pass did
    pub components: Option<ExecutionLog>, // Nested log (for sub-pipelines)
}

pub enum ActionType {
    DidNothing,
    DidTransform,
    DidAnalysis,
    DidAnalysisTransform,
    DidIfElse,
    DidPipeline,
}
```

The log records what each pass did during forward execution. It's used during backwards
to navigate which passes need to be reversed.

**Problems:**

1. **Dual Purpose** — The log serves both as an execution record AND as the guide for
   backwards execution. It's a flat list with an `ActionType` tag, making pattern matching
   verbose.

2. **Implicit Structure** — The `components: Option<ExecutionLog>` field is only set for
   certain pass types (`IfElse`, `Pipeline`). This structure is implicit — you have to
   know which `ActionType` values have components.

3. **Backwards Navigation Logic** — The backwards function walks the log in reverse:
   ```rust
   for (pass, log_elem) in passes.iter().zip(log.iter()).rev() {
       match (pass, &log_elem.kind) {
           (Pass::Transformation(_), ActionType::DidTransform) => { /* ... */ }
           (Pass::IfElse(_), ActionType::DidIfElse) => { /* ... */ }
           // ...
       }
   }
   ```
   The pass type and action type must align. If they don't, backwards silently skips
   the pass. This is fragile.

---

### 2.4 The Pass Traits

There are several pass traits:

**BasePass** — Common interface for all passes:
```rust
pub trait BasePass {
    fn name(&self) -> String;
    fn requires(&self) -> Vec<String>;  // Dependency names
}
```

**TransformationPass** — Reversible transformations:
```rust
pub trait TransformationPass: BasePass + DynClone {
    fn invalidates(&self) -> Vec<String>;  // Analyses invalidated by this pass
    
    fn run(&self, model: Model, cache: &AnalysisCache) -> TransformationPassResult;
    
    fn backwards(&self, solution: Solution, cache: &AnalysisCache) -> LunaModelResult<Solution>;
}

pub struct TransformationOutcome {
    pub model: Model,                         // Transformed model
    pub analysis: Option<AnalysisCacheElement>, // Optional analysis/backwards state
    pub action: ActionType,                   // What happened
}
```

**Key observation:** `backwards(&self, ...)` is an **instance method**. It takes `&self`,
meaning it has access to the pass's configuration. However, the configuration alone is
often NOT enough to invert the transformation — you also need backwards state from the
`cache`.

**Example from BinarySpinPass:**
```rust
impl TransformationPass for BinarySpinPass {
    fn run(&self, mut model: Model, _cache: &AnalysisCache) -> TransformationPassResult {
        let mut info = BinarySpinInfo::try_new(self.vtype)?;
        
        // ... transform model, populate info.map ...
        
        Ok(TransformationOutcome::new(
            model,
            Some(AnalysisCacheElement::BinarySpinInfoAnalysis(info)),  // ← Store backwards state
            ActionType::DidTransform,
        ))
    }
    
    fn backwards(&self, mut solution: Solution, cache: &AnalysisCache) -> LunaModelResult<Solution> {
        // Retrieve backwards state from cache
        match cache.get("binary-spin") {
            Some(AnalysisCacheElement::BinarySpinInfoAnalysis(info)) => {
                // ... use info.map to invert the transformation ...
            }
            _ => Err(/* ... */)
        }
    }
}
```

The pass stores `BinarySpinInfo` in the cache during `run()`, then retrieves it during
`backwards()`. The cache acts as the communication channel between forward and backward.

**Problems:**

1. **Pass Configuration vs. Backwards State** — The `backwards(&self, ...)` signature
   suggests the pass configuration (in `self`) is enough, but it's not. The real backwards
   state is in the cache, accessed by string key.

2. **Instance Method Requires PassManager** — Since `backwards` is an instance method,
   you need the original `PassManager` (with its pass instances) to execute backwards.
   This means the `PassManager` cannot be serialized and reconstructed later.

3. **Invisible State for Python Passes** — Python passes can store state in `self`:
   ```python
   class MyPass:
       def run(self, model, cache):
           self.backwards_info = compute_info()  # ← Stored in Python self
           return model
       
       def backwards(self, solution, cache):
           use(self.backwards_info)  # ← Accessed from self
   ```
   This state is invisible to the framework. It's not in the cache, not serializable,
   and the framework can't ensure it's preserved.

---

**AnalysisPass** — Non-reversible analyses:
```rust
pub trait AnalysisPass: BasePass + DynClone {
    fn run(&self, model: &Model, cache: &AnalysisCache) -> AnalysisPassResult;
}

pub type AnalysisPassResult = LunaModelResult<Option<AnalysisCacheElement>>;
```

Analysis passes compute information and optionally store it in the cache. They don't
transform the model and don't participate in backwards execution.

---

**MetaAnalysisPass** — Analyses of the pass pipeline itself:
```rust
pub trait MetaAnalysisPass: BasePass + DynClone {
    fn run(&self, passes: &[Pass], cache: &mut AnalysisCache) -> AnalysisPassResult;
}
```

These passes analyze the pipeline structure (e.g., "how many transformation passes are
there?"). They're used rarely but add complexity.

**Problem:** This is a separate trait for a special case. In a well-designed system,
this functionality should be handled through regular analysis passes with appropriate
context.

---

### 2.5 The Pass Enum

```rust
pub enum Pass {
    Transformation(Box<dyn TransformationPass>),
    Analysis(Box<dyn AnalysisPass>),
    IfElse(IfElsePass),
    Pipeline(Box<dyn AbstractPipeline>),
    MetaAnalysis(Box<dyn MetaAnalysisPass>),
}
```

A closed enum of pass types. Special passes like `IfElse` and `Pipeline` are enum variants,
not trait implementations.

**Problems:**

1. **Closed Enum** — Adding a new pass type (e.g., `WhilePass`, `SwitchPass`) requires
   modifying this enum and adding branches to all match statements.

2. **Inconsistent Design** — `IfElsePass` is a concrete struct, not a trait implementation.
   It's special-cased in the execution logic.

---

### 2.6 The IfElsePass Implementation

```rust
pub struct IfElsePass {
    requires: Vec<String>,
    condition: Box<dyn Condition>,
    then: Box<dyn AbstractPipeline>,
    otherwise: Box<dyn AbstractPipeline>,
    name: String,
}

pub trait Condition {
    fn call(&self, cache: &AnalysisCache) -> LunaModelResult<bool>;
}

impl IfElsePass {
    pub fn run(&self, model: Model, cache: &AnalysisCache, executor: &PassManager) -> IfElsePassResult {
        let is_condition = self.condition.call(cache)?;
        let ir = if is_condition {
            self.then.run(model, &cache, executor)?
        } else {
            self.otherwise.run(model, &cache, executor)?
        };
        
        let analysis = AnalysisCacheElement::IfElseInfoAnalysis(IfElseInfo {
            fulfilled_condition: is_condition,
        });
        
        Ok(IfElseOutcome { ir, analysis })
    }
    
    pub fn backwards(&self, solution: Solution, ir: &IR, log: &ExecutionLog) -> LunaModelResult<Solution> {
        // Backwards through the sub-pipeline that was executed
        // ...
    }
}
```

**Key Design:**
- The pass evaluates a condition at runtime
- It executes one of two sub-pipelines based on the condition
- It records which branch was taken in `IfElseInfo`
- Backwards delegates to the sub-pipeline

**Problems:**

1. **Special-Cased** — `IfElsePass` is not a `TransformationPass`. It's a separate type
   with its own backwards signature:
   ```rust
   fn backwards(&self, solution: Solution, ir: &IR, log: &ExecutionLog)
   ```
   vs. the normal:
   ```rust
   fn backwards(&self, solution: Solution, cache: &AnalysisCache)
   ```
   This inconsistency complicates the execution logic.

2. **Requires PassManager** — The `run` method takes `executor: &PassManager` to run
   sub-pipelines. This tightly couples `IfElsePass` to the execution infrastructure.

3. **Backwards State Implicit** — The `IfElseInfo` only records `fulfilled_condition: bool`.
   The actual backwards logic is in the pass instance itself (`self.then` and `self.otherwise`).
   This means the pass instance must be available during backwards.

---

### 2.7 The PassManager

```rust
pub struct PassManager {
    passes: Vec<Pass>,
}

impl PassManager {
    pub fn run(&self, model: Model) -> LunaModelResult<IR> {
        let input_model = model.deep_clone();
        check_dependencies(&self.passes)?;
        let mut ir = run_passes(&self.passes, model, AnalysisCache::new(), self)?;
        ir.input_model = Some(input_model);
        Ok(ir)
    }
    
    pub fn backwards(&self, solution: Solution, ir: &IR) -> LunaModelResult<Solution> {
        let sol = backwards(&self.passes, solution, ir, None)?;
        // Evaluate solution against input model for validation
        if let Some(input) = &ir.input_model {
            Ok(input.evaluate_solution(&sol)?)
        } else {
            Ok(sol)
        }
    }
}
```

**Key Design:**
- `run` produces an `IR` containing the transformed model, cache, and log
- `backwards` takes the `PassManager` instance (`&self`), the solution, and the `IR`
- It delegates to the `backwards` function which walks the passes and log in reverse

**Problems:**

1. **PassManager Required for Backwards** — The `backwards` method is on the `PassManager`.
   This means you must have the original `PassManager` instance to execute backwards.
   You can't just serialize the `IR` and later call `ir.backwards(solution)`.

2. **No Serialization** — The `PassManager` contains `Vec<Pass>`, which contains trait
   objects (`Box<dyn TransformationPass>`). Trait objects are not serializable. Even if
   they were, closures (used in conditions, predicates) are not serializable.

3. **Deep Clone of Input Model** — The input model is deep-cloned at the start of `run()`.
   For large models, this is expensive and unnecessary — we only need it for solution
   validation, not for backwards execution.

---

### 2.8 Backwards Execution Logic

```rust
pub fn backwards(
    passes: &Vec<Pass>,
    mut solution: Solution,
    ir: &IR,
    log: Option<&ExecutionLog>,
) -> LunaModelResult<Solution> {
    for (pass, log_elem) in passes.iter().zip(log.unwrap_or(&ir.execution_log).iter()).rev() {
        match (pass, &log_elem.kind) {
            (Pass::Transformation(pass), ActionType::DidTransform | ActionType::DidAnalysisTransform) => {
                solution = pass.backwards(solution, &ir.cache)?;
            }
            (Pass::IfElse(pass), ActionType::DidIfElse) => {
                if let Some(inner_log) = &log_elem.components {
                    solution = pass.backwards(solution, &ir, inner_log)?
                }
            }
            (Pass::Pipeline(pass), ActionType::DidPipeline) => {
                if let Some(inner_log) = &log_elem.components {
                    solution = pass.backwards(solution, &ir, inner_log)?
                }
            }
            _ => {}
        }
    }
    Ok(solution)
}
```

**How It Works:**
1. Iterate through passes in **reverse order**
2. For each pass, check the corresponding log element
3. If the pass transformed the model (`DidTransform`), call its `backwards` method
4. If the pass was a special pass (`IfElse`, `Pipeline`), recursively call `backwards`
   on the sub-pipeline using the nested log

**Problems:**

1. **Pass and Log Must Align** — The code zips passes with log elements. If the lengths
   don't match or the order is wrong, backwards will fail silently (skipping passes).

2. **Fragile Pattern Matching** — The match statement relies on the pass type and action
   type being consistent. If a transformation pass returns `ActionType::DidNothing`, it's
   skipped during backwards (correct), but this logic is implicit.

3. **Cache Passed to All Backwards Calls** — The entire `ir.cache` is passed to every
   `backwards` call. Passes retrieve their specific backwards state by string key. There's
   no isolation — a pass could accidentally access another pass's state.

4. **Requires Pass Instances** — The `&self` reference in `backwards(&self, ...)` means
   we need the original pass instances. This prevents backwards execution after deserialization.

---

## 3. Python Integration

### 3.1 Python Pass Adapter

Python transformation passes are adapted to Rust via `PyTransformationPassAdapter`:

```rust
pub struct PyTransformationPassAdapter {
    inner: Py<PyAny>,  // Python pass instance
    name: String,
    // ...
}

impl TransformationPass for PyTransformationPassAdapter {
    fn run(&self, model: Model, cache: &AnalysisCache) -> TransformationPassResult {
        Python::with_gil(|py| {
            let py_model = model.to_python(py)?;
            let py_cache = cache.to_python(py)?;
            
            // Call Python run() method
            let result = self.inner.call_method1(py, "run", (py_model, py_cache))?;
            
            // Extract TransformationOutcome
            // ...
        })
    }
    
    fn backwards(&self, solution: Solution, cache: &AnalysisCache) -> LunaModelResult<Solution> {
        Python::with_gil(|py| {
            let py_solution = solution.to_python(py)?;
            let py_cache = cache.to_python(py)?;
            
            // Call Python backwards() method
            let result = self.inner.call_method1(py, "backwards", (py_solution, py_cache))?;
            
            // Extract Solution
            // ...
        })
    }
}
```

**Python Protocol:**
```python
class TransformationPass:
    def run(self, model, cache):
        # Transform model
        # Return (transformed_model, optional_analysis, action_type)
        pass
    
    def backwards(self, solution, cache):
        # Invert transformation
        # Return original solution
        pass
```

**Problems:**

1. **State in Python `self`** — Python passes can store backwards state in `self`:
   ```python
   class MyPass:
       def run(self, model, cache):
           self.mapping = create_mapping()  # ← Invisible to framework
           return transform(model)
       
       def backwards(self, solution, cache):
           return invert(solution, self.mapping)  # ← Uses invisible state
   ```
   This state is not in the cache. The framework doesn't know about it. If the Python
   pass instance is garbage collected, the state is lost.

2. **No Enforcement of Externalization** — There's no mechanism to force Python passes
   to store backwards state in the cache. It's a convention, not a requirement.

3. **GIL Dependency in Cache** — The `PyAnalysis(Py<PyAny>)` variant requires GIL access
   for cloning, which leaks throughout the Rust codebase.

---

## 4. Fundamental Limitations

### 4.1 Backwards Requires PassManager Instance

**Problem:** `PassManager::backwards(&self, solution, ir)` requires `&self`.

**Why It Matters:** You cannot serialize the transformation pipeline to disk, load it
later, and execute backwards. The `PassManager` must be reconstructed in code.

**Real-World Impact:**
- Can't save a compiled model pipeline to disk for later use
- Can't distribute pre-compiled pipelines
- Can't execute backwards in a different process from forward

**Root Cause:** The `backwards(&self, ...)` signature on transformation passes allows
them to access pass configuration. But configuration alone isn't enough — backwards state
is in the cache. This mixed design prevents serialization.

---

### 4.2 Backwards State Not Explicitly Typed

**Problem:** Backwards state is stored in the `AnalysisCacheElement` enum, accessed by
string keys, with no compile-time type safety.

**Example:**
```rust
// Pass stores backwards state during run()
Some(AnalysisCacheElement::BinarySpinInfoAnalysis(info))

// Pass retrieves it during backwards()
match cache.get("binary-spin") {
    Some(AnalysisCacheElement::BinarySpinInfoAnalysis(info)) => { /* ... */ }
    _ => Err(/* ... */)
}
```

**Why It Matters:**
- No compile-time guarantee that backwards state is present
- Easy to misspell the cache key ("binary-spin" vs "binary_spin")
- Must pattern match to extract typed value
- The connection between the pass and its backwards state is implicit

---

### 4.3 AnalysisCacheElement is a Closed Enum

**Problem:** Every new pass that needs backwards state must add a variant to
`AnalysisCacheElement`.

**Why It Matters:**
- Tight coupling between passes and the core crate
- Can't define pass types in external crates without modifying core
- Violates the Open-Closed Principle (open for extension, closed for modification)

**Example:** To add a new pass:
```rust
// 1. Define backwards state type
pub struct MyPassInfo { /* ... */ }

// 2. Add variant to AnalysisCacheElement (modifying core!)
pub enum AnalysisCacheElement {
    // ... existing variants ...
    MyPassInfoAnalysis(MyPassInfo),  // ← Core modification
}

// 3. Update Clone impl
impl Clone for AnalysisCacheElement {
    fn clone(&self) -> Self {
        match self {
            // ... existing cases ...
            AnalysisCacheElement::MyPassInfoAnalysis(v) => { /* ... */ }
        }
    }
}

// 4. Implement the pass
pub struct MyPass;
impl TransformationPass for MyPass { /* ... */ }
```

This is heavyweight for adding a simple pass.

---

### 4.4 Python State is Invisible

**Problem:** Python passes can store backwards state in `self`, invisible to the framework.

**Example:**
```python
class BadPass:
    def run(self, model, cache):
        self.secret_state = expensive_computation()  # ← Not in cache!
        return model
    
    def backwards(self, solution, cache):
        return use_state(self.secret_state)  # ← Relies on invisible state
```

**Why It Matters:**
- The framework can't serialize this state
- The framework can't ensure the state is preserved
- No way to enforce that Python passes externalize state

---

### 4.5 GIL-Dependent Clone

**Problem:** `AnalysisCacheElement::PyAnalysis(Py<PyAny>)` requires GIL access to clone.

```rust
AnalysisCacheElement::PyAnalysis(v) => {
    Python::attach(|py| AnalysisCacheElement::PyAnalysis(v.clone_ref(py)))
}
```

**Why It Matters:**
- Special `clone_py(&self, py: Python)` method needed in Python contexts
- GIL-awareness leaks throughout the codebase
- Can't use `AnalysisCache: Clone` in generic contexts without GIL

---

### 4.6 ExecutionLog as Backwards Guide

**Problem:** The `ExecutionLog` serves dual purposes: execution record and backwards guide.

**Why It Matters:**
- The log structure (flat list + action types) is optimized for logging, not navigation
- Backwards execution must zip passes with log elements and match on action types
- The structure is implicit (e.g., `components` is only set for certain action types)

**Example of fragility:**
```rust
for (pass, log_elem) in passes.iter().zip(log.iter()).rev() {
    match (pass, &log_elem.kind) {
        (Pass::Transformation(_), ActionType::DidTransform) => { /* ... */ }
        // If pass and log get out of sync, this silently skips passes
    }
}
```

---

### 4.7 Special Passes are Special-Cased

**Problem:** `IfElsePass` and `Pipeline` are not `TransformationPass` implementations.
They're separate types with special execution logic.

**Why It Matters:**
- Adding a new special pass type (e.g., `WhilePass`) requires modifying the execution
  infrastructure
- The `Pass` enum must be extended
- The `backwards` function must be updated with new match arms
- Violates the Open-Closed Principle

**Example:**
```rust
// In execution.rs
match (pass, &log_elem.kind) {
    // ... existing cases ...
    (Pass::IfElse(pass), ActionType::DidIfElse) => { /* special logic */ }
    (Pass::Pipeline(pass), ActionType::DidPipeline) => { /* special logic */ }
    // Adding WhilePass requires another case here
}
```

---

### 4.8 MetaAnalysisPass is a Separate Concept

**Problem:** `MetaAnalysisPass` is a trait for passes that analyze the pass list itself.

```rust
pub trait MetaAnalysisPass {
    fn run(&self, passes: &[Pass], cache: &mut AnalysisCache) -> AnalysisPassResult;
}
```

**Why It Matters:**
- This should be handled through regular analysis passes with appropriate context
- Creates an extra pass type to maintain
- The need for this trait suggests the analysis infrastructure is incomplete

---

## 5. Why These Limitations Exist

### Historical Context

The current design evolved incrementally:
1. Started with simple transformation passes
2. Added analysis passes for caching computed results
3. Reused the cache to store backwards state (convenient but conceptually wrong)
4. Added `IfElsePass` as a special case
5. Added Python support via trait objects and `PyAnalysis` enum variant
6. Added `MetaAnalysisPass` for pipeline-level analyses

Each addition made sense in isolation, but the cumulative design has structural issues.

### Design Tradeoffs

**Why instance method backwards?**
- Seemed natural — the pass knows how to invert itself
- Allows access to pass configuration
- Works fine for simple cases

**Why reuse AnalysisCache for backwards state?**
- Convenient — already had a cache mechanism
- Avoided creating a separate backwards state store
- Made sense when there were only a few passes

**Why closed enums?**
- Rust's enum pattern matching is powerful
- Seemed simpler than trait objects for everything
- Works well for small, fixed sets of variants

**Why allow Python state in self?**
- Python programmers expect to use `self`
- Difficult to enforce externalization in Python
- Adapters can't inspect Python object internals

### The Core Problem

**The design conflates analysis results and backwards state.** They're both stored in
`AnalysisCache` and both use `AnalysisCacheElement`. This seemed reasonable when:
- There were only a few passes
- Serialization wasn't a requirement
- The distinction between "analysis" and "backwards state" was unclear

But as the system grew, this conflation became problematic. Backwards state is not the
same as analysis results:
- **Analysis results** are optional information about the model (e.g., max bias, sparsity)
- **Backwards state** is mandatory information to invert transformations (e.g., variable mappings)

They have different lifetimes, purposes, and serialization requirements.

---

## 6. Summary of Limitations

| # | Limitation | Root Cause | Impact |
|---|------------|------------|--------|
| 1 | Backwards requires PassManager instance | `backwards(&self, ...)` signature | Can't serialize/deserialize pipeline |
| 2 | Backwards state not typed | String-keyed cache access | No compile-time safety |
| 3 | AnalysisCacheElement is closed | Enum-based design | Can't extend without modifying core |
| 4 | Python state invisible | No enforcement mechanism | State can be lost, not serializable |
| 5 | GIL-dependent clone | `PyAnalysis(Py<PyAny>)` variant | GIL-awareness leaks everywhere |
| 6 | ExecutionLog as backwards guide | Dual-purpose design | Fragile backwards navigation |
| 7 | Special passes are special-cased | Enum-based pass types | Hard to add new pass types |
| 8 | MetaAnalysisPass separate concept | Incomplete analysis infrastructure | Extra complexity |
| 9 | Analysis and backwards state conflated | Reused AnalysisCache for both | Conceptual confusion |
| 10 | Deep clone of input model | Needed for solution validation | Expensive for large models |

---

## 7. What Works Well

Despite the limitations, the current design has strengths:

✅ **Functional** — The pipeline works correctly for current use cases  
✅ **Performant** — No significant performance issues  
✅ **Flexible pass definitions** — Easy to implement new simple passes  
✅ **Python integration exists** — Python passes work (with caveats)  
✅ **Dependency checking** — Ensures required analyses are available  
✅ **Invalidation tracking** — Transformations can invalidate analyses  
✅ **IfElsePass** — Conditional branching works (though special-cased)  

The core ideas are sound. The issues are structural, not algorithmic.

---

## 8. Motivation for Redesign

The redesign (described below) addresses these limitations by:

1. **Making backwards a static method** — `fn backward(artifact: &Artifact, solution: Solution)`
   forces all backwards state into the artifact

2. **Separating analysis and backwards state** — `AnalysisManager` for analyses,
   `Artifact` trait for backwards state

3. **Using a backwards registry** — Enables backwards execution without pass instances

4. **Type-safe analysis store** — `AnalysisKey<T>` provides compile-time type safety

5. **Open extension** — Any type can be an artifact, any pass can be implemented without
   modifying core types

6. **Forcing Python state externalization** — Python artifacts are pickled objects with
   their `backward()` method preserved

7. **CompilationRecord as first-class** — Serializable record of the transformation path,
   independent of PassManager

8. **Unified pass types** — IfElsePass is just a `ReversiblePass`, not a special case

This redesign maintains all the strengths while fixing the structural issues.
