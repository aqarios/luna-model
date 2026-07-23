//! Internal handle storage for the LunaModel C API.
//!
//! Storage is selected by semantics: mutable shared objects use
//! `Arc<RwLock<T>>`, `ArcEnv` is already synchronized internally, and
//! model-backed expression/collection handles retain the model they view.

use std::{ops::Deref, sync::Arc};

use lunamodel_capi::{
    LmCC, LmConstraint, LmEnv, LmExpr, LmLazyBounds, LmModel, LmSol, LmSpecs, LmVref,
};
use lunamodel_core::prelude::{LazyBounds, VarRef};
use lunamodel_core::{ArcEnv, Constraint, ConstraintCollection, Expression, Model, Solution};
use lunamodel_types::Specs;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

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

pub enum ExpressionWriteGuard<'a> {
    Expression(RwLockWriteGuard<'a, Expression>),
    Model(MappedRwLockWriteGuard<'a, Expression>),
}

impl Deref for ExpressionWriteGuard<'_> {
    type Target = Expression;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Expression(guard) => guard,
            Self::Model(guard) => guard,
        }
    }
}

impl std::ops::DerefMut for ExpressionWriteGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Expression(guard) => guard,
            Self::Model(guard) => guard,
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
                    &model.objective
                }))
            }
        }
    }

    pub fn write(&self) -> ExpressionWriteGuard<'_> {
        match self {
            Self::Expression(value) => ExpressionWriteGuard::Expression(value.write()),
            Self::Model(model) => {
                ExpressionWriteGuard::Model(RwLockWriteGuard::map(model.write(), |model| {
                    &mut model.objective
                }))
            }
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

pub enum ConstraintCollectionWriteGuard<'a> {
    Collection(RwLockWriteGuard<'a, ConstraintCollection>),
    Model(MappedRwLockWriteGuard<'a, ConstraintCollection>),
}

impl Deref for ConstraintCollectionWriteGuard<'_> {
    type Target = ConstraintCollection;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Collection(guard) => guard,
            Self::Model(guard) => guard,
        }
    }
}

impl std::ops::DerefMut for ConstraintCollectionWriteGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Collection(guard) => guard,
            Self::Model(guard) => guard,
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
                    &model.constraints
                }))
            }
        }
    }

    pub fn write(&self) -> ConstraintCollectionWriteGuard<'_> {
        match self {
            Self::Collection(value) => ConstraintCollectionWriteGuard::Collection(value.write()),
            Self::Model(model) => ConstraintCollectionWriteGuard::Model(RwLockWriteGuard::map(
                model.write(),
                |model| &mut model.constraints,
            )),
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
