//! Internal handle storage for the LunaModel C API.
//!
//! Storage is selected by semantics: mutable shared objects use
//! `Arc<RwLock<T>>`, `ArcEnv` is already synchronized internally, and
//! model-backed expression/collection handles retain the model they view.

use std::{ops::Deref, sync::Arc};

use lunamodel_capi::{
    LmCC, LmConstraint, LmEnv, LmExpr, LmLazyBounds, LmModel, LmSol, LmSpecs, LmVref,
    plugin::LmControlFlowPlan,
};
use lunamodel_core::ops::{LmAddAssign, LmMulAssign, LmPowAssign, LmSubAssign};
use lunamodel_core::prelude::{HigherOrder, LazyBounds, Linear, Quadratic, VarRef};
use lunamodel_core::{ArcEnv, Constraint, ConstraintCollection, Expression, Model, Solution};
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::ControlFlowPlan;
use lunamodel_types::{Bias, Specs, VarIdx};
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Associates one opaque C type with its exact host-side storage type.
///
/// # Safety
/// Each opaque handle type must have exactly one storage mapping in the process.
pub unsafe trait HandleKind: Sized + 'static {
    type Storage: 'static;
}

/// Associates a handle with ordinary synchronized value storage.
///
/// # Safety
/// `HandleKind::Storage` must be [`Shared<Self>`].
pub unsafe trait SharedHandleKind: HandleKind + Sized + 'static {
    type Value: 'static;
}

/// Shared synchronized storage for an ordinary mutable handle.
pub type Shared<H> = Arc<RwLock<<H as SharedHandleKind>::Value>>;

/// Creates synchronized storage for a shared opaque handle.
pub fn new_shared<H>(value: H::Value) -> Shared<H>
where
    H: SharedHandleKind,
{
    Arc::new(RwLock::new(value))
}

unsafe impl HandleKind for LmModel {
    type Storage = Shared<Self>;
}
unsafe impl SharedHandleKind for LmModel {
    type Value = Model;
}

unsafe impl HandleKind for LmSol {
    type Storage = Shared<Self>;
}
unsafe impl SharedHandleKind for LmSol {
    type Value = Solution;
}

unsafe impl HandleKind for LmConstraint {
    type Storage = Shared<Self>;
}
unsafe impl SharedHandleKind for LmConstraint {
    type Value = Constraint;
}

// ArcEnv contains its own Arc<RwLock<Environment>> and needs no outer lock.
unsafe impl HandleKind for LmEnv {
    type Storage = ArcEnv;
}

// These facade values are immutable or already retain synchronized parents,
// so their handles store the value directly without an additional lock.
unsafe impl HandleKind for LmVref {
    type Storage = VarRef;
}

unsafe impl HandleKind for LmLazyBounds {
    type Storage = LazyBounds;
}

unsafe impl HandleKind for LmSpecs {
    type Storage = Specs;
}

// A control-flow plan is a transient owned builder (created, populated with steps, then consumed by
// the host adapter within one control-flow `run`); it stores the plan value directly, with no lock.
unsafe impl HandleKind for LmControlFlowPlan {
    type Storage = ControlFlowPlan;
}

/// Storage for either a standalone expression or a live model-objective view.
#[derive(Clone)]
pub enum ExpressionHandle {
    Expression(Arc<RwLock<Expression>>),
    Model(Shared<LmModel>),
}

pub enum ExpressionReadGuard<'a> {
    Expression(RwLockReadGuard<'a, Expression>),
    Model(MappedRwLockReadGuard<'a, Expression>),
}

impl Deref for ExpressionReadGuard<'_> {
    type Target = Expression;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Expression(guard) => guard,
            Self::Model(guard) => guard,
        }
    }
}

/// Write access to an expression handle.
///
/// There is deliberately no `DerefMut`: a model-objective view only edits through
/// [`Model::objective_mut`], so the model's environment and liveness checks apply
/// to every edit coming in over the C API. Standalone expressions are edited directly.
pub enum ExpressionWriteGuard<'a> {
    Expression(RwLockWriteGuard<'a, Expression>),
    Model(RwLockWriteGuard<'a, Model>),
}

impl Deref for ExpressionWriteGuard<'_> {
    type Target = Expression;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Expression(guard) => guard,
            Self::Model(guard) => guard.objective(),
        }
    }
}

impl ExpressionWriteGuard<'_> {
    pub fn add_assign<T>(&mut self, rhs: T) -> LunaModelResult<()>
    where
        Expression: LmAddAssign<T>,
    {
        match self {
            Self::Expression(expr) => LmAddAssign::add_assign(&mut **expr, rhs),
            Self::Model(model) => model.objective_mut().add_assign(rhs),
        }
    }

    pub fn sub_assign<T>(&mut self, rhs: T) -> LunaModelResult<()>
    where
        Expression: LmSubAssign<T>,
    {
        match self {
            Self::Expression(expr) => LmSubAssign::sub_assign(&mut **expr, rhs),
            Self::Model(model) => model.objective_mut().sub_assign(rhs),
        }
    }

    pub fn mul_assign<T>(&mut self, rhs: T) -> LunaModelResult<()>
    where
        Expression: LmMulAssign<T>,
    {
        match self {
            Self::Expression(expr) => LmMulAssign::mul_assign(&mut **expr, rhs),
            Self::Model(model) => model.objective_mut().mul_assign(rhs),
        }
    }

    pub fn pow_assign(&mut self, exp: usize) -> LunaModelResult<()> {
        match self {
            Self::Expression(expr) => LmPowAssign::pow_assign(&mut **expr, exp),
            Self::Model(model) => model.objective_mut().pow_assign(exp),
        }
    }

    pub fn set_offset(&mut self, offset: Bias) {
        match self {
            Self::Expression(expr) => expr.offset = offset,
            Self::Model(model) => model.objective_mut().set_offset(offset),
        }
    }

    pub fn add_linear_term(&mut self, var: VarIdx, bias: Bias) -> LunaModelResult<()> {
        match self {
            Self::Expression(expr) => {
                expr.linear += (var, bias);
                Ok(())
            }
            Self::Model(model) => model.objective_mut().add_linear_term(var, bias),
        }
    }

    pub fn add_quadratic_term(&mut self, u: VarIdx, v: VarIdx, bias: Bias) -> LunaModelResult<()> {
        match self {
            Self::Expression(expr) => {
                *expr.quadratic.get_or_insert_default() += (u, v, bias);
                Ok(())
            }
            Self::Model(model) => model.objective_mut().add_quadratic_term(u, v, bias),
        }
    }

    pub fn add_higher_order_term(&mut self, vars: &[VarIdx], bias: Bias) -> LunaModelResult<()> {
        match self {
            Self::Expression(expr) => {
                *expr.higher_order.get_or_insert_default() += (vars, bias);
                Ok(())
            }
            Self::Model(model) => model.objective_mut().add_higher_order_term(vars, bias),
        }
    }

    pub fn set_linear(&mut self, linear: Linear) -> LunaModelResult<()> {
        match self {
            Self::Expression(expr) => {
                expr.linear = linear;
                Ok(())
            }
            Self::Model(model) => model.objective_mut().set_linear(linear),
        }
    }

    pub fn set_quadratic(&mut self, quadratic: Option<Quadratic>) -> LunaModelResult<()> {
        match self {
            Self::Expression(expr) => {
                expr.quadratic = quadratic;
                Ok(())
            }
            Self::Model(model) => model.objective_mut().set_quadratic(quadratic),
        }
    }

    pub fn set_higher_order(&mut self, higher_order: Option<HigherOrder>) -> LunaModelResult<()> {
        match self {
            Self::Expression(expr) => {
                expr.higher_order = higher_order;
                Ok(())
            }
            Self::Model(model) => model.objective_mut().set_higher_order(higher_order),
        }
    }
}

impl ExpressionHandle {
    pub fn new(value: Expression) -> Self {
        Self::Expression(Arc::new(RwLock::new(value)))
    }

    pub fn for_model(model: Shared<LmModel>) -> Self {
        Self::Model(model)
    }

    pub fn read(&self) -> ExpressionReadGuard<'_> {
        match self {
            Self::Expression(value) => ExpressionReadGuard::Expression(value.read()),
            Self::Model(model) => {
                ExpressionReadGuard::Model(RwLockReadGuard::map(model.read(), |model| {
                    model.objective()
                }))
            }
        }
    }

    pub fn write(&self) -> ExpressionWriteGuard<'_> {
        match self {
            Self::Expression(value) => ExpressionWriteGuard::Expression(value.write()),
            Self::Model(model) => ExpressionWriteGuard::Model(model.write()),
        }
    }
}

unsafe impl HandleKind for LmExpr {
    type Storage = ExpressionHandle;
}

/// Storage for a standalone collection or a live model-constraints view.
#[derive(Clone)]
pub enum ConstraintCollectionHandle {
    Collection(Arc<RwLock<ConstraintCollection>>),
    Model(Shared<LmModel>),
}

pub enum ConstraintCollectionReadGuard<'a> {
    Collection(RwLockReadGuard<'a, ConstraintCollection>),
    Model(MappedRwLockReadGuard<'a, ConstraintCollection>),
}

impl Deref for ConstraintCollectionReadGuard<'_> {
    type Target = ConstraintCollection;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Collection(guard) => guard,
            Self::Model(guard) => guard,
        }
    }
}

/// Write access to a constraint-collection handle.
///
/// There is deliberately no `DerefMut`: a model-constraints view only changes
/// through the checked [`Model`] API, so every constraint added over the C API
/// must belong to the model's environment. Standalone collections are edited directly.
pub enum ConstraintCollectionWriteGuard<'a> {
    Collection(RwLockWriteGuard<'a, ConstraintCollection>),
    Model(RwLockWriteGuard<'a, Model>),
}

impl Deref for ConstraintCollectionWriteGuard<'_> {
    type Target = ConstraintCollection;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Collection(guard) => guard,
            Self::Model(guard) => guard.constraints(),
        }
    }
}

impl ConstraintCollectionWriteGuard<'_> {
    pub fn add_constraint(
        &mut self,
        constr: Constraint,
        name: Option<String>,
    ) -> LunaModelResult<String> {
        match self {
            Self::Collection(cc) => cc.add_constraint(constr, name),
            Self::Model(model) => model.add_constraint(constr, name),
        }
    }

    pub fn add_many_constraints(
        &mut self,
        constrs: impl IntoIterator<Item = (Constraint, Option<String>)>,
    ) -> LunaModelResult<Vec<String>> {
        match self {
            Self::Collection(cc) => cc.add_many_constraints(constrs.into_iter()),
            Self::Model(model) => model.add_many_constraints(constrs),
        }
    }

    pub fn add_collection(
        &mut self,
        other: ConstraintCollection,
        prefix: Option<String>,
    ) -> LunaModelResult<Vec<String>> {
        match self {
            Self::Collection(cc) => cc.add_collection(other, prefix),
            Self::Model(model) => model.add_collection(other, prefix),
        }
    }

    pub fn set_constraint(&mut self, key: &str, constr: Constraint) -> LunaModelResult<()> {
        match self {
            Self::Collection(cc) => cc.set_constraint(key, constr),
            Self::Model(model) => model.set_constraint(key, constr),
        }
    }

    pub fn remove_constraint(&mut self, key: &str) -> LunaModelResult<()> {
        match self {
            Self::Collection(cc) => cc.remove_constraint(key),
            Self::Model(model) => model.remove_constraint(key),
        }
    }

    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> LunaModelResult<()> {
        match self {
            Self::Collection(cc) => cc.substitute(target, replacement),
            Self::Model(model) => {
                model.update_constraints(|_, _, constr| constr.substitute(target, replacement))
            }
        }
    }
}

impl ConstraintCollectionHandle {
    pub fn new(value: ConstraintCollection) -> Self {
        Self::Collection(Arc::new(RwLock::new(value)))
    }

    pub fn for_model(model: Shared<LmModel>) -> Self {
        Self::Model(model)
    }

    pub fn read(&self) -> ConstraintCollectionReadGuard<'_> {
        match self {
            Self::Collection(value) => ConstraintCollectionReadGuard::Collection(value.read()),
            Self::Model(model) => {
                ConstraintCollectionReadGuard::Model(RwLockReadGuard::map(model.read(), |model| {
                    model.constraints()
                }))
            }
        }
    }

    pub fn write(&self) -> ConstraintCollectionWriteGuard<'_> {
        match self {
            Self::Collection(value) => ConstraintCollectionWriteGuard::Collection(value.write()),
            Self::Model(model) => ConstraintCollectionWriteGuard::Model(model.write()),
        }
    }
}

unsafe impl HandleKind for LmCC {
    type Storage = ConstraintCollectionHandle;
}

/// Moves concrete storage behind an opaque owned handle.
pub fn handle_from_storage<H: HandleKind>(storage: H::Storage) -> *mut H {
    Box::into_raw(Box::new(storage)).cast()
}

/// Creates ordinary synchronized storage and returns an owned handle.
pub fn new_handle<H>(value: H::Value) -> *mut H
where
    H: SharedHandleKind<Storage = Shared<H>>,
{
    handle_from_shared::<H>(new_shared::<H>(value))
}

/// Creates an owned handle retaining ordinary synchronized storage.
pub fn handle_from_shared<H>(shared: Shared<H>) -> *mut H
where
    H: SharedHandleKind<Storage = Shared<H>>,
{
    handle_from_storage::<H>(shared)
}

/// Borrows the concrete storage behind an opaque handle.
///
/// # Safety
/// `handle` must be null or a live handle of type `H` created by this crate.
pub unsafe fn handle_ref<'a, H: HandleKind>(handle: *const H) -> Option<&'a H::Storage> {
    unsafe { handle.cast::<H::Storage>().as_ref() }
}

/// Mutably borrows the concrete storage behind an owned opaque handle.
///
/// # Safety
/// `handle` must be null or a live owned handle of type `H`, exclusively borrowed for `'a` (no other
/// reference to the same handle may be live).
pub unsafe fn handle_mut<'a, H: HandleKind>(handle: *mut H) -> Option<&'a mut H::Storage> {
    unsafe { handle.cast::<H::Storage>().as_mut() }
}

/// Consumes an opaque handle and returns its concrete storage.
///
/// # Safety
/// `handle` must be owned and must not be accessed or freed afterwards.
pub unsafe fn handle_into_storage<H: HandleKind>(handle: *mut H) -> Option<H::Storage> {
    if handle.is_null() {
        return None;
    }
    Some(*unsafe { Box::from_raw(handle.cast::<H::Storage>()) })
}

/// Consumes an ordinary synchronized handle and returns its shared storage.
///
/// # Safety
/// `handle` must be owned and must not be accessed or freed afterwards.
pub unsafe fn handle_into_shared<H>(handle: *mut H) -> Option<Shared<H>>
where
    H: SharedHandleKind<Storage = Shared<H>>,
{
    unsafe { handle_into_storage(handle) }
}

/// Releases an owned opaque handle. Passing null is a no-op.
///
/// # Safety
/// `handle` must be null or owned and must not be freed twice.
pub unsafe fn drop_handle<H: HandleKind>(handle: *mut H) {
    if !handle.is_null() {
        drop(unsafe { Box::from_raw(handle.cast::<H::Storage>()) });
    }
}
