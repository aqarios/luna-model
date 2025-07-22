use crate::core::environment::SharedEnvironment;
use crate::core::term::types::SizeType;
use crate::core::{ValueByIndex, Vtype};
use crate::types::{Bias, VarIndex};
use num::pow::Pow;
use num::NumCast;
use std::fmt::{Debug, Display, LowerExp};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};
use std::str::FromStr;

use super::errors::VariableOutOfRangeErr;

pub trait One {
    fn one() -> Self;
}

pub trait IndexConstraints:
    Copy
    + Debug
    + Default
    + PartialOrd
    + Ord
    + Into<SizeType>
    + From<SizeType>
    + AddAssign
    + One
    + ToString
    + FromStr
    + Hash
{
}
impl<
        T: Copy
            + Debug
            + Default
            + PartialOrd
            + Ord
            + Into<SizeType>
            + From<SizeType>
            + AddAssign
            + One
            + ToString
            + FromStr
            + Hash,
    > IndexConstraints for T
{
}

pub trait BiasConstraints:
    Debug
    + Neg<Output = Self>
    + Display
    + Copy
    + Default
    + AddAssign
    + Add<Output = Self>
    + Sub<Output = Self>
    + Pow<Self, Output = Self>
    + PartialEq
    + PartialOrd
    + One
    + MulAssign
    + Mul<Output = Self>
    + Mul<Bias, Output = Self>
    + Div<Bias, Output = Self>
    + PartialEq<Bias>
    + PartialOrd<Bias>
    + Neg<Output = Self>
    + NumCast
    + FromStr
    + LowerExp
{
}
impl<
        T: Debug
            + Neg<Output = Self>
            + Display
            + Copy
            + Default
            + AddAssign
            + Add<Output = T>
            + Sub<Output = T>
            + Pow<T, Output = T>
            + PartialEq
            + PartialOrd
            + One
            + MulAssign
            + Mul<Output = T>
            + Mul<Bias, Output = Self>
            + Div<Bias, Output = Self>
            + PartialEq<Bias>
            + PartialOrd<Bias>
            + Neg<Output = T>
            + NumCast
            + FromStr
            + LowerExp,
    > BiasConstraints for T
{
}

pub trait ExpressionBaseTypes {
    /// The key used by higher order terms. This is implementation dependent.
    /// Thus we cannot fix it to some value here.
    type HigherOrderKey;
    /// The type of the linear terms used by the expression implementation.
    type LinearType;
    /// The type of the quadratic terms used by the expression implementation.
    type QuadraticType;
    /// The type of the higher order terms used by the expression implementation.
    type HigherOrderType;
}

pub trait ExpressionBaseCreation<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn simple(env: SharedEnvironment, offset: Bias) -> Self;
    fn empty(env: SharedEnvironment) -> Self;
    fn new(env: SharedEnvironment, active: Vec<bool>, num_variables: usize) -> Self;
    fn new_from_other(other: &Self) -> Self;
    fn new_linear_single(env: SharedEnvironment, v: Index, bias: Bias) -> Self;
    fn new_linear(env: SharedEnvironment, u: (Index, Bias), v: (Index, Bias)) -> Self;
    fn new_linear_and_offset(env: SharedEnvironment, v: Index, bias: Bias, offset: Bias) -> Self;
    fn new_quadratic(env: SharedEnvironment, u: Index, v: Index, bias: Bias) -> Self;
}

pub trait ExpressionBaseAdjustment<Index, Bias>: ExpressionBase<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn add_variable(&mut self, v: Index) -> SizeType;
    fn add_variables(&mut self, vars: &Vec<Index>);
    fn remove_variable(&mut self, v: Index);
    fn remove_variables(&mut self, vars: &Vec<Index>);
    /// Resize the model to contain `n` variables.
    fn resize(&mut self, n: Index);
}

pub trait ExpressionBaseSet<Index, Bias>: ExpressionBaseTypes
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    /// Add interaction between variables in `indices`.
    fn set_higher_order(&mut self, vars: &Vec<Index>, bias: Bias);
}

pub trait ExpressionBase<Index, Bias> {
    /// Return the offset.
    fn offset(&self) -> Bias;
    /// The linear bias of variable `v`.
    fn linear(&self, v: Index) -> Result<Bias, VariableOutOfRangeErr>;
    /// Return the quadratic bias associated with `u` and `v`.
    ///
    /// If `u` and `v` do not have a quadratic bias, return 0;
    ///
    /// Note that this function does not return a reference because
    /// each quadratic bias is stored twice.
    /// // todo: we might be able to change this, as we store it just once.
    fn quadratic(&self, u: Index, v: Index) -> Result<Bias, VariableOutOfRangeErr>;
    /// Return the higher order bias associated with the indices
    /// If indices do not have a quadratic bias, return 0;
    fn higher_order(&self, indices: &Vec<Index>) -> Result<Bias, VariableOutOfRangeErr>;
    /// Test whether the model has no quadratic biases.
    fn is_linear(&self) -> bool;
    /// Return the number of variables in the expression.
    fn num_variables(&self) -> SizeType;
    /// Return the the variables indices in the expression.
    fn variables(&self) -> Vec<Index>;
    /// Return the variable type of variable `v`.
    fn vartype(&self, _v: Index) -> Vtype;
    
}

/// Implements addition of variables, biases (scalars) and terms to `self`.
/// This basically implements AddAssign directly operating on the expression level.
pub trait ExpressionBaseAdd<Index, Bias>: ExpressionBaseTypes
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    /// Add offset.
    fn add_offset(&mut self, bias: Bias);
    /// Add linear bias to variable `v`.
    fn add_linear(&mut self, v: Index, bias: Bias);
    /// Add interaction between variables `v` and `u`.
    fn add_quadratic(&mut self, u: Index, v: Index, bias: Bias);
    /// Add interaction between variables in `indices`.
    fn add_higher_order(&mut self, vars: &Vec<Index>, bias: Bias);
    /// Add interaction between variables in `indices`.
    /// This is the same as `add_higher_order` but operates on the key used in the
    /// higher order representation directly.
    fn add_higher_order_direct(&mut self, key: &Self::HigherOrderKey, bias: Bias);

    /// Add linear biases from another linear term.
    fn add_linear_from(&mut self, other: &Self::LinearType, other_active: &Vec<bool>);
    /// Add quadratic biases from another quadratic term.
    fn add_quadratic_from(&mut self, other: &Self::QuadraticType);
    /// Add higher order biases from another higher order term.
    fn add_higher_order_from(&mut self, other: &Self::HigherOrderType);
    /// Add quadratic bias for the given variables at the end of each other's neighborhoods.
    ///
    /// # Arguments
    ///
    /// * `u` - A variable
    /// * `v` - A variable
    /// * `bias` - the quadratic bias associated with `u` and `v`
    ///
    /// # Exceptions
    /// When `u` is less than the largest neighbor in `v`'s neighborhood,
    /// `v` is less than the largest neighbor in `u`'s neighborhood, or either
    /// `u` or `v` is greater than `num_variables()` then the behavior of
    /// this method is undefined.
    fn add_quadratic_back(&mut self, u: Index, v: Index, bias: Bias);
    /// Add quadratic biases from a dense matrix.
    ///
    /// `dense` must be an array of length `num_variables^2`.
    ///
    /// Values on the diagonal are treated differently depending on the variable
    /// type.
    ///
    /// # Exceptions
    /// The behavior of this method is undefined when the model has fewer than
    /// `num_variables` variables.
    fn add_quadratic_from_dense(&mut self, dense: &[Bias], num_variables: Index);
}

/// Implements multiplication of variables, biases (scalars) and terms to `self`.
/// This basically implements MulAssign directly operating on the expression level.
pub trait ExpressionBaseMul<Index, Bias>: ExpressionBaseTypes
// + ExpressionBaseMulComponents<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn multiply(lhs: &Self, rhs: &Self, result: &mut Self);
}

pub trait ExpressionBaseMulComponents<Index, Bias>: ExpressionBaseTypes
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn mul_offsets(&mut self, lhs: &Bias, rhs: &Bias);

    fn mul_linear_with_offset(&mut self, linear: &Vec<(Index, Bias)>, offset: &Bias);
    fn mul_linears(&mut self, lhs: &Vec<(Index, Bias)>, rhs: &Vec<(Index, Bias)>);
    fn mul_quadratic_with_offset(&mut self, lhs: &Self::QuadraticType, offset: &Bias);
    fn mul_quadratic_with_linear(&mut self, lhs: &Self::QuadraticType, rhs: &Vec<(Index, Bias)>);
    fn mul_quadratics(&mut self, lhs: &Self::QuadraticType, rhs: &Self::QuadraticType);

    fn mul_higher_order_with_offset(&mut self, lhs: &Self::HigherOrderType, offset: &Bias);
    fn mul_higher_order_with_linear(
        &mut self,
        lhs: &Self::HigherOrderType,
        rhs: &Vec<(Index, Bias)>,
    );
    fn mul_higher_order_with_quadratic(
        &mut self,
        lhs: &Self::HigherOrderType,
        rhs: &Self::QuadraticType,
    );
    fn mul_higher_orders(&mut self, lhs: &Self::HigherOrderType, rhs: &Self::HigherOrderType);
}

/// Implements multiplication of variables, biases (scalars) and terms to `self`.
/// This basically implements MulAssign directly operating on the expression level.
///
/// In contrast to the methods defined in `ExpressionBaseMulAssign` the methods defined
/// in this trait use the information on a variable / bias (scalar) level without the
/// representation as a linear, quadratic or higher oder term. In theory one could
/// also create the terms based on the variable, and then use the other trait.
/// However, this introduces additional memory and access required to implement the
/// operation which is undesired regarding performance optimizations.
///
/// The results are written to `self`
pub trait ExpressionBaseMulDirect<Index, Bias>: ExpressionBaseTypes
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    /// Mul the variable `v` to the offset, producing a new linear entry.
    /// This function edits self, based on the information from offset and the rest.
    fn mul_with_offset(&mut self, offset: Bias, v: Index, bias: Bias);
    /// Mul the variable `v` to the linear term.
    /// This function edits self, based on the information from linear and the rest.
    fn mul_with_linear(&mut self, linear: &Self::LinearType, v: Index, bias: Bias);
    /// Mul the variable `v` to the quadratic term.
    /// This function edits self, based on the information from quadratic and the rest.
    fn mul_with_quadratic(&mut self, quadratic: &Self::QuadraticType, v: Index, bias: Bias);
    /// Mul the variable `v` to the higher order term.
    /// This function edits self, based on the information from higher_order and the rest.
    fn mul_with_higher_order(&mut self, higher_order: &Self::HigherOrderType, v: Index, bias: Bias);
}

pub trait ExpressionEvaluation<Idx, Bias>
where
    Idx: IndexConstraints,
    Bias: BiasConstraints,
{
    fn evaluate_sample<'a, Elem: 'a, Sample: ValueByIndex<Idx, Output = Elem>, F>(
        &self,
        sample: &'a Sample,
        index_map: F,
    ) -> Bias
    where
        Elem: Mul<Bias, Output = Bias>,
        F: Fn(VarIndex) -> Idx;

    fn evaluate_sampleset<
        'a,
        Elem: 'a,
        Sample: ValueByIndex<Idx, Output = Elem> + 'a,
        SampleSet: Iterator<Item = &'a Sample> + Copy,
        F,
    >(
        &self,
        sampleset: &'a SampleSet,
        index_map: F,
    ) -> Vec<Bias>
    where
        Elem: Mul<Bias, Output = Bias>,
        F: Fn(VarIndex) -> Idx;
}
