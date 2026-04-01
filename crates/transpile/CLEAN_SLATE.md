# Clean-Slate Transform Architecture
## Compiler Engineering Perspective (LLVM/MLIR Inspired)

**Design Constraints:**
- Forward: `Model → [Passes] → TransformedModel`
- Backward: `Solution(TransformedModel) → [Inverse Passes] → Solution(Model)`
- Backward must work after serialization/deserialization
- Type-safe, extensible, no closed enums
- Clean Python integration without GIL dependencies

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
    
    /// Invalidate analyses affected by a transformation pass
    pub fn invalidate(&mut self, pass_name: &str, registered_analyses: &[Box<dyn Any>]) {
        // Remove invalidated analyses
        self.results.retain(|name, _| {
            // Check if this analysis is invalidated by pass_name
            // (requires runtime analysis pass lookup)
            !is_analysis_invalidated(name, pass_name, registered_analyses)
        });
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
/// The pass manager orchestrates forward execution and produces a CompilationRecord.
/// 
/// Unlike the CompilationRecord, the PassManager is NOT serializable.
/// It's just code — you reconstruct it when needed.
pub struct PassManager {
    passes: Vec<PipelineStep>,
    analysis_manager: AnalysisManager,
}

pub enum PipelineStep {
    /// Run a transformation pass
    Transform(Box<dyn ReversiblePass<Artifact = dyn Artifact>>),
    
    /// Run an analysis pass
    Analysis(Box<dyn AnalysisPass<Result = dyn Any>>),
    
    /// Run a nested sub-pipeline
    Pipeline { name: String, passes: Vec<PipelineStep> },
}

impl PassManager {
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            analysis_manager: AnalysisManager::new(),
        }
    }
    
    /// Add a transformation pass to the pipeline
    pub fn add_transform<P: ReversiblePass + 'static>(mut self, pass: P) -> Self {
        self.passes.push(PipelineStep::Transform(Box::new(pass)));
        self
    }
    
    /// Add an analysis pass to the pipeline
    pub fn add_analysis<P: AnalysisPass + 'static>(mut self, pass: P) -> Self {
        self.passes.push(PipelineStep::Analysis(Box::new(pass)));
        self
    }
    
    /// Run the pipeline and produce a compilation record
    pub fn run(&mut self, model: &mut Model) -> Result<CompilationRecord> {
        let mut entries = Vec::new();
        
        for step in &self.passes {
            match step {
                PipelineStep::Transform(pass) => {
                    let ctx = PassContext::new(&self.analysis_manager);
                    let artifact = pass.forward(model, &ctx)?;
                    
                    entries.push(PassEntry::Transform {
                        pass_name: pass.name().to_string(),
                        artifact: ErasedArtifact::new(&artifact)?,
                    });
                    
                    // Invalidate affected analyses
                    self.analysis_manager.invalidate(pass.name(), &[]);
                }
                
                PipelineStep::Analysis(pass) => {
                    let ctx = PassContext::new(&self.analysis_manager);
                    let result = pass.run(model, &ctx)?;
                    
                    // Store in analysis manager
                    // (requires dynamic dispatch or type-erased storage)
                    
                    entries.push(PassEntry::Analysis {
                        pass_name: pass.name().to_string(),
                    });
                }
                
                PipelineStep::Pipeline { name, passes } => {
                    // Recursively run sub-pipeline
                    let mut sub_manager = PassManager { 
                        passes: passes.clone(),
                        analysis_manager: self.analysis_manager.clone(),
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
    /// Fully qualified name of the Python class (for deserialization)
    class_path: String,
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
