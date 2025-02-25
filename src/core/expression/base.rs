use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::rc::Rc;
use std::str::FromStr;

use crate::core::term::types::SizeType;
use crate::core::{Environment, Vtype};

use super::errors::VariableOutOfRangeError;

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
    + Copy
    + Default
    + AddAssign
    + Add<Output = Self>
    + PartialEq
    + One
    + MulAssign
    + Mul<Output = Self>
{
}
impl<
        T: Debug
            + Copy
            + Default
            + AddAssign
            + Add<Output = T>
            + PartialEq
            + One
            + MulAssign
            + Mul<Output = T>,
    > BiasConstraints for T
{
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
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
    fn new(env: Rc<RefCell<Environment<Index>>>) -> Self;
    fn new_from_other(other: &Self) -> Self;
    fn new_linear_single(env: Rc<RefCell<Environment<Index>>>, v: Index, bias: Bias) -> Self;
    fn new_linear(env: Rc<RefCell<Environment<Index>>>, u: Index, v: Index, bias: Bias) -> Self;
    fn new_quadratic(env: Rc<RefCell<Environment<Index>>>, u: Index, v: Index, bias: Bias) -> Self;
}

pub trait ExpressionBaseAdjustment<Index, Bias>: ExpressionBase<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    // fn new_linear_single(n: Index) -> Self;
    /// Increase the size of the model by one. Returns the index of the new variable.
    fn add_variable(&mut self) -> Index;
    /// Increase the size of the model by `n`. Returns the index of the first variable
    /// added.
    fn add_variables(&mut self, n: Index) -> Index;
    // // todo: make this rusty -> makes sense to return a & here??
    // /// Return an empty neighborhood; useful when a variable does not have an adjacency.
    // // fn empty_neighborhood(&self) -> &Vec<OneVarTerm<Index, Bias>>;
    /// Resize the model to contain `n` variables.
    fn resize(&mut self, n: Index);
}

pub trait ExpressionBaseSet<Index, Bias>: ExpressionBaseTypes
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    // -- /// Set offset.
    // -- fn set_offset(&mut self, bias: Bias);
    // -- /// Add linear bias to variable `v`.
    // -- fn set_linear(&mut self, v: Index, bias: Bias) -> Result<(), VariableOutOfRangeError>;
    // -- /// Add interaction between variables `v` and `u`.
    // -- fn set_quadratic(&mut self, u: Index, v: Index, bias: Bias);
    /// Add interaction between variables in `indices`.
    fn set_higher_order(
        &mut self,
        vars: &Vec<Index>,
        bias: Bias,
    ) -> Result<(), VariableOutOfRangeError>;
}

pub trait ExpressionBase<Index, Bias> {
    // - // todo: add_quadratic for row, col (arrays) and row, col iterators. (2 extra fns)
    // - // todo: neighborhood iterators, start and end/ check if we really need this in rust
    // - // for linear and quadratic.

    // removed as it's the same as using add
    // /// Add interaction between variables in `indices` overwriting an existing element.
    // fn add_higher_order_overwrite(&mut self, indices: &Vec<Index>, bias: Bias);

    // - // - /// Return the energy of the given sample.
    // - // - ///
    // - // - /// The `sample_start` must be a random access iterator pointing to the
    // - // - /// beginning of the sample.
    // - // - ///
    // - // - /// The behavior of this function is undefined when the sample is not
    // - // - /// `num_variables()` long.
    // - // - fn energy<Iter>(&self, sample_start: Iter) -> Bias;
    // - // - // Remove variable `v` from the model by fixing its value.
    // - // - //
    // - // - // Note that this causes reindexing, where all variables above `v` have
    // - // - // their index reduced by one.
    // - // - fn fix_variable(&mut self, v: Index, assignment: Bias);
    // - // - /// Check whether `u` and `v` have an interaction.
    // - // - fn has_interaction(&self, u: Index, v: Index) -> bool;
    // - // - /// Test whether two quadratic models are equal..
    // - // - fn is_equal<B, I>(&self, other: Self) -> bool
    // - // - where
    // - // -     B: BiasConstraints,
    // - // -     I: IndexConstraints;
    /// Return the offset.
    fn offset(&self) -> Bias;
    /// The linear bias of variable `v`.
    fn linear(&self, v: Index) -> Result<Bias, VariableOutOfRangeError>;
    /// Return the quadratic bias associated with `u` and `v`.
    ///
    /// If `u` and `v` do not have a quadratic bias, return 0;
    ///
    /// Note that this function does not return a reference because
    /// each quadratic bias is stored twice.
    /// // todo: we might be able to change this, as we store it just once.
    fn quadratic(&self, u: Index, v: Index) -> Result<Bias, VariableOutOfRangeError>;
    /// Return the higher order bias associated with the indices
    ///
    /// If indices do not have a quadratic bias, return 0;
    fn higher_order(&self, indices: &Vec<Index>) -> Result<Bias, VariableOutOfRangeError>;
    /// Test whether the model has no quadratic biases.
    fn is_linear(&self) -> bool;
    // - // - /// Return the number of interactions in the quadratic model.
    // - // - fn num_interactions(&self) -> SizeType;
    // - // - /// Return the number of other variables that `v` interacts with.
    // - // - fn num_interactions_variable(&self, v: Index) -> SizeType;
    /// Return the number of variables in the quadratic model.
    fn num_variables(&self) -> SizeType;
    // - /// return the quadratic bias associated with `u` and `v`.
    // - ///
    // - /// Note that this function does not return a reference because
    // - /// each quadratic bias is stored twice.
    // - ///
    // - /// Returns an `out_of_range` error if either `u` or `v` are not variables;
    // - /// if they do not have an interaction, the function throws an exception.
    // - fn quadratic_at(&self, u: Index, v: Index) -> ExpressionResult<Bias>;
    // - /// Remove the interaction between variables `u` and `v`.
    // - fn remove_interaction(&mut self, u: Index, v: Index) -> bool;
    // - /// Remove all interactions for which `filter` returns `true`.
    // - /// Returns the number of interactions removed.
    // - /// `filter` must be symmetric. That is `filter(u, v, bias)` must be equal
    // - /// `filter(v, u, bias)`.
    // - fn remove_interactions<Filter>(&mut self, filter: Filter) -> SizeType;
    // - /// Remove variable `v` from the model.
    // - ///
    // - /// Note that this causes reindexing, where all variables above `v` have their
    // - /// index reduced by one.
    // - fn remove_variable(&mut self, v: Index) {
    // -     // We use the trait implemented function to achieve the same behavior as
    // -     // `virtual` functions in cpp.
    // -     unimplemented!()
    // - }
    // - /// Remove multiple variables from the model and reindex accordingly.
    // - fn remove_variables(&mut self, variables: &Vec<Index>) {
    // -     // We use the trait implemented function to achieve the same behavior as
    // -     // `virtual` functions in cpp.
    // -     unimplemented!()
    // - }
    // - /// Multiply all biases by the value of `scalar`.
    // - fn scale(&mut self, scaler: Bias);
    // - /// Set the linear bias of variable `v`.
    // - fn set_linear(&mut self, v: Index, bias: Bias);
    // - /// Set the linear biases of the variables beginning with `v`.
    // - fn set_linear_from(&mut self, v: Index, biases: &[Bias]);
    // - /// Set the offset.
    // - fn set_offset(&mut self, offset: Bias);
    // - /// Set the quadratic bias between variables `u` and `v`
    // - fn set_quadratic(&mut self, u: Index, v: Index, bias: Bias);
    // - ///
    // - fn subsitute_variable(&mut self, v: Index, multiplier: Bias, offset: Bias);
    // - ///
    // - fn subsitute_variables(&mut self, multiplier: Bias, offset: Bias);
    // - /// Return the lower bound on variable `v`.
    // - fn lower_bound(&self, _v: Index) -> Bias {
    // -     // We use the trait implemented function to achieve the same behavior as
    // -     // `virtual` functions in cpp.
    // -     Bias::default()
    // - }
    // - /// Return the upper bound on variable `v`.
    // - fn upper_bound(&self, _v: Index) -> Bias {
    // -     Bias::default()
    // - }
    /// Return the variable type of variable `v`.
    fn vartype(&self, _v: Index) -> Vtype;
    // - /// Total bytes consumed by the biases and indices.
    // - ///
    // - /// If `capacity` is true, use the capacity of the underlying vectors rather
    // - /// than the size.
    // - fn nbytes(&self, capacity: Option<bool>) -> SizeType {
    // -     // We use the trait implemented function to achieve the same behavior as
    // -     // `virtual` functions in cpp.
    // -     let _cap = capacity.unwrap_or(false);
    // -     unimplemented!()
    // - }
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
    fn add_linear(&mut self, v: Index, bias: Bias) -> Result<(), VariableOutOfRangeError>;
    /// Add interaction between variables `v` and `u`.
    fn add_quadratic(
        &mut self,
        u: Index,
        v: Index,
        bias: Bias,
    ) -> Result<(), VariableOutOfRangeError>;
    /// Add interaction between variables in `indices`.
    fn add_higher_order(
        &mut self,
        vars: &Vec<Index>,
        bias: Bias,
    ) -> Result<(), VariableOutOfRangeError>;
    /// Add interaction between variables in `indices`.
    /// This is the same as `add_higher_order` but operates on the key used in the
    /// higher order representation directly.
    fn add_higher_order_direct(&mut self, key: &Self::HigherOrderKey, bias: Bias);

    /// Add linear biases from another linear term.
    fn add_linear_from(&mut self, other: &Self::LinearType);
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
    fn add_quadratic_back(
        &mut self,
        u: Index,
        v: Index,
        bias: Bias,
    ) -> Result<(), VariableOutOfRangeError>;
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
    fn add_quadratic_from_dense(
        &mut self,
        dense: &[Bias],
        num_variables: Index,
    ) -> Result<(), VariableOutOfRangeError>;
}

/// Implements multiplication of variables, biases (scalars) and terms to `self`.
/// This basically implements MulAssign directly operating on the expression level.
pub trait ExpressionBaseMul<Index, Bias>: ExpressionBaseTypes
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    /// Multiply two offset and add to self.
    fn mul_offset(&mut self, lhs: Bias, rhs: Bias);
    /// Multiply two linear terms and add to self.
    fn mul_linear(&mut self, lhs: &Self::LinearType, rhs: &Self::LinearType);
    /// Multiply two quadratic terms and add to self.
    fn mul_quadratic(&mut self, lhs: &Self::QuadraticType, rhs: &Self::QuadraticType);
    /// Multiply two higher order terms and add to self.
    fn mul_higher_order(&mut self, lhs: &Self::HigherOrderType, rhs: &Self::HigherOrderType);
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
    fn mul_with_offset(
        &mut self,
        offset: Bias,
        v: Index,
        bias: Bias,
    ) -> Result<(), VariableOutOfRangeError>;
    /// Mul the variable `v` to the linear term.
    /// This function edits self, based on the information from linear and the rest.
    fn mul_with_linear(
        &mut self,
        linear: &Self::LinearType,
        v: Index,
        bias: Bias,
    ) -> Result<(), VariableOutOfRangeError>;
    /// Mul the variable `v` to the quadratic term.
    /// This function edits self, based on the information from quadratic and the rest.
    fn mul_with_quadratic(
        &mut self,
        quadratic: &Self::QuadraticType,
        v: Index,
        bias: Bias,
    ) -> Result<(), VariableOutOfRangeError>;
    /// Mul the variable `v` to the higher order term.
    /// This function edits self, based on the information from higher_order and the rest.
    fn mul_with_higher_order(
        &mut self,
        higher_order: &Self::HigherOrderType,
        v: Index,
        bias: Bias,
    ) -> Result<(), VariableOutOfRangeError>;
}
