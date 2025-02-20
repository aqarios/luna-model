use std::cell::RefCell;
use std::ops::{Add, AddAssign};
use std::rc::Rc;

use crate::core::term::types::SizeType;
use crate::core::term::{HigherOrder, Linear, Quadratic};
use crate::core::{Environment, Vtype};

pub trait One {
    fn one() -> Self;
}

pub trait IndexConstraints:
    Copy + Default + PartialOrd + Ord + Into<SizeType> + From<SizeType> + AddAssign + One + ToString
{
}
impl<
        T: Copy
            + Default
            + PartialOrd
            + Ord
            + Into<SizeType>
            + From<SizeType>
            + AddAssign
            + One
            + ToString,
    > IndexConstraints for T
{
}

pub trait BiasConstraints:
    Copy + Default + AddAssign + Add<Output = Self> + PartialEq + One
{
}
impl<T: Copy + Default + AddAssign + Add<Output = T> + PartialEq + One> BiasConstraints for T {}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}

pub trait ExpressionBase<Index, Bias> {
    // - // todo: add_quadratic for row, col (arrays) and row, col iterators. (2 extra fns)
    // - // todo: neighborhood iterators, start and end/ check if we really need this in rust
    // - // for linear and quadratic.

    /// Add offset.
    fn add_offset(&mut self, bias: Bias);
    /// Add linear bias to variable `v`.
    fn add_linear(&mut self, v: Index, bias: Bias);
    /// Add linear biases from another linear term.
    fn add_linear_from(&mut self, other: &Linear<Bias>);
    /// Add interaction between variables `v` and `u`.
    fn add_quadratic(&mut self, u: Index, v: Index, bias: Bias);
    /// Add quadratic biases from another quadratic term.
    fn add_quadratic_from(&mut self, other: &Quadratic<Index, Bias>);
    /// Add interaction between variables in `indices`.
    fn add_higher_order_direct(&mut self, index: &String, bias: Bias);
    /// Add interaction between variables in `indices`.
    fn add_higher_order(&mut self, indices: &Vec<Index>, bias: Bias);
    /// Add higher order biases from another higher order term.
    fn add_higher_order_from(&mut self, other: &HigherOrder<Index, Bias>);
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
    /// Test whether the model has no quadratic biases.
    fn is_linear(&self) -> bool;
    /// The linear bias of variable `v`.
    fn linear(&self, v: Index) -> Bias;
    // - // - /// Return the number of interactions in the quadratic model.
    // - // - fn num_interactions(&self) -> SizeType;
    // - // - /// Return the number of other variables that `v` interacts with.
    // - // - fn num_interactions_variable(&self, v: Index) -> SizeType;
    /// Return the number of variables in the quadratic model.
    fn num_variables(&self) -> SizeType;
    /// Return the offset.
    fn offset(&self) -> Bias;
    // - /// Return the quadratic bias associated with `u` and `v`.
    // - ///
    // - /// If `u` and `v` do not have a quadratic bias, return 0;
    // - ///
    // - /// Note that this function does not return a reference because
    // - /// each quadratic bias is stored twice.
    // - fn quadratic(&self, u: Index, v: Index) -> Bias;
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
    fn vartype(&self, _v: Index) -> Vtype {
        Vtype::default()
    }
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

// todo: needs a better name.
pub trait ExpressionBaseInternal<Index, Bias>: ExpressionBase<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    fn new(env: Rc<RefCell<Environment<Index>>>) -> Self;
    fn new_from(other: &Self) -> Self;
    fn new_linear(env: Rc<RefCell<Environment<Index>>>, linear_biases: &Vec<Bias>) -> Self;
    fn new_from_weighted_variable(
        env: Rc<RefCell<Environment<Index>>>,
        var: Index,
        weight: Bias,
    ) -> Self;
    fn new_linear_from_variables(
        env: Rc<RefCell<Environment<Index>>>,
        lhs: Index,
        rhs: Index,
        bias: Bias,
    ) -> Self;
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
    /// Hidden version of vartype() that allows users to distinguish between the
    /// `vartype_` called by mixin functions and the public API one.
    /// By default they are the same.
    fn vartype_(&self, v: Index) -> Vtype {
        self.vartype(v)
    }
}
