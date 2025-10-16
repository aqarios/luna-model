use crate::core::expression::{Expression, ExpressionEvaluation};
use crate::core::operations::SubToExpression;
use crate::core::traits::ContentEquality;
use crate::core::writer::ModelWriter;
use crate::core::{ExpressionBase, SharedEnvironment, Substitution, ValueByIndex, VarRef, Vtype};
use crate::errors::{
    DifferentEnvsErr, DuplicateConstraintNameErr, GetConstraintErr, IllegalConstraintNameErr,
    IndexOutOfBoundsErr,
};
use crate::types::{Bias, VarIndex};
use indexmap::IndexMap;
use itertools::Itertools;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Mul};
use std::string::ToString;
use strum_macros::Display;

#[cfg(feature = "py")]
use pyo3::prelude::*;

/// [Constraint] names that can fail when translating to other formats due to bugs in
/// their LP file reading, interpreting any words that start with the constants elements
/// as a number instead of a string. Thus, we need to disallow them as well to ensure
/// consistency and elevated developer experience. The failing readers do not provide a
/// good error message so we catch theses cases early and show the users and appropriate
/// error mesage.
pub const FAILABLE_CONSTRAINT_NAMES: [&str; 2] = ["inf", "nan"];

/// Utility function to check the "legality" of a constraint name based on the disallowed
/// word beginnings as given in [`FAILABLE_CONSTRAINT_NAMES`]. Simply return a bool to let
/// the caller decide on how to handle this case.
pub fn starts_with_failable(s: &str) -> bool {
    FAILABLE_CONSTRAINT_NAMES
        .iter()
        .any(|prefix| s.to_lowercase().starts_with(&prefix.to_lowercase()))
}

#[cfg_attr(
    all(feature = "py", not(feature = "lq")),
    pyclass(eq, eq_int, name = "Comparator", module = "aqmodels._core")
)]
#[cfg_attr(
    all(feature = "py", feature = "lq"),
    pyclass(eq, eq_int, name = "Comparator", module = "luna_quantum._core")
)]
/// Comparison operators used to define constraints.
///
/// This enum represents the logical relation between the left-hand side (LHS)
/// and the right-hand side (RHS) of a [Constraint].
///
/// Attributes
/// ----------
/// Eq : Comparator
///     Equality constraint (==).
/// Le : Comparator
///     Less-than-or-equal constraint (<=).
/// Ge : Comparator
///     Greater-than-or-equal constraint (>=).
///
/// Examples
/// --------
/// >>> from luna_quantum import Comparator
/// >>> str(Comparator.Eq)
/// '=='
// we require the python config here, since wrapping an enum in the py_bindings is a tedious task.
#[derive(Debug, Copy, Clone, PartialEq, Display, Eq, Hash)]
pub enum Comparator {
    /// The Equality comparison (==) for a constraint where LHS == RHS.
    #[strum(to_string = "==")]
    Eq,
    /// The Less-than or equal comparison (<=) for a constraint where LHS <= RHS.
    #[strum(to_string = "<=")]
    Le,
    /// The Greater-than or equal comparison (>=) for a constraint where LHS >= RHS.
    #[strum(to_string = ">=")]
    Ge,
}

impl Comparator {
    /// Utility function used to determine if the [Constraint] is met given concrete LHS
    /// and RHS values. This function is used as part of the [Model](crate::core::Model) evaluation of samples
    /// or a solution.
    pub fn evaluate(&self, lhs: Bias, rhs: Bias) -> bool {
        match self {
            Self::Eq => lhs == rhs,
            Self::Le => lhs <= rhs,
            Self::Ge => lhs >= rhs,
        }
    }
}

/// A constraint is a collection of
#[derive(Debug, Clone)]
pub struct Constraint {
    /// The LHS expression of the constraint.
    pub lhs: Expression,
    /// The RHS of a constraint which can be an arbitrary `Bias` value.
    pub rhs: Bias,
    /// The comparator defines the relationship between the LHS and RHS of the constraint.
    /// See `Comparator` for all available options.
    pub comparator: Comparator,
    /// A Constraint can also be named for easier, more native indexing into a collection of
    /// constraints.
    ///
    /// Note: This is subject to change in the future from an option to a required parameter to
    /// ensure a more consistent and user safe API. In addition, it is required to enable
    /// enhancements in the transformation stack for operations working on constraints. For more
    /// details see <https://github.com/aqarios/aq-models-rs/issues/400>.
    pub name: Option<String>,
}

impl Constraint {
    /// A constraint contains an [Expression] for the LHS which in turn has a shared reference
    /// to an [Environment](crate::core::Environment) (specifically a [SharedEnvironment]). Simply cloning a [Constraint]
    /// results in a copy/clone of most of the data with a reference to the same [SharedEnvironment]
    /// as the cloned constraint
    /// To get a free-standing copy/clone independent of the original [SharedEnvironment], this
    /// function can be used with a new [SharedEnvironment] the [Expression]s should reference to.
    pub fn deep_clone(&self, env: SharedEnvironment) -> Self {
        Self {
            lhs: self.lhs.deep_clone(env),
            rhs: self.rhs,
            comparator: self.comparator,
            name: self.name.clone(),
        }
    }
}

impl Constraint {
    /// Create a new constraint.
    ///
    /// This function will change from accepting an `Option<String>` to a `String` for the name
    /// with <https://github.com/aqarios/aq-models-rs/400>.
    pub fn new(
        lhs: Expression,
        rhs: Bias,
        comparator: Comparator,
        name: Option<String>,
    ) -> Result<Self, IllegalConstraintNameErr> {
        Self::validate_name(&name)?;
        let lhs_constant = lhs.offset();
        let actual_rhs = rhs - lhs_constant;
        let lhs = lhs.sub(lhs_constant);
        Ok(Self {
            lhs,
            rhs: actual_rhs,
            comparator,
            name,
        })
    }

    /// Utility function to validate the correctness/legality of a [Constraint] name.
    /// A [Constraint] name is considered legal if it is:
    /// - not an empty string
    /// - the first char is alphabetical
    /// - starts with any of the failable word beginnings defined in
    /// (FAILABLE_CONSTRAINT_NAMES)[crate::core::constraints::constraint::FAILABLE_CONSTRAINT_NAMES].
    /// checked in [starts_with_failable].
    /// This function returns an error when an illegal [Constraint] name is given as an argument.
    pub fn validate_name(name: &Option<String>) -> Result<(), IllegalConstraintNameErr> {
        if let Some(name) = &name {
            if name.is_empty() {
                return Err(IllegalConstraintNameErr(
                    "constraint names cannot be empty strings".to_string(),
                ));
            }
            let first_char = name.chars().next().unwrap();
            let first_char_alpha = first_char.is_alphabetic();

            if !first_char_alpha {
                return Err(IllegalConstraintNameErr(format!(
                    "constraint names must start with an alphabetical character, is {}",
                    first_char
                )));
            }

            if starts_with_failable(name) {
                return Err(IllegalConstraintNameErr(format!(
                    "constraint names cannot start with one of '{}', is {}",
                    FAILABLE_CONSTRAINT_NAMES.join(", "),
                    name
                )));
            }
        }
        Ok(())
    }

    /// Set the name of the [Constraint]. If the [Constraint] has a name set already. The old name
    /// is overwritten. In case the new name is illegal an error ([IllegalConstraintNameErr]) is returned.
    pub fn set_name(&mut self, name: Option<String>) -> Result<(), IllegalConstraintNameErr> {
        Self::validate_name(&name)?;
        self.name = name;
        Ok(())
    }

    /// To check if a [Constraint] (`self`) is satisfied given a concrete Sample.
    /// Since we cannot ensure the data contained in the Sample is aligned with the variables
    /// of the [Constraint]'s expression, this function also requires an index map for computing
    /// the actual value of an [Expression] for the given Sample. This `index_map` maps the
    /// [VarRef]'s index to the position in the Sample.
    pub fn evaluate_sample<'a, Elem: 'a, Sample: ValueByIndex<VarIndex, Output = Elem>, F>(
        &self,
        sample: &'a Sample,
        index_map: F,
    ) -> bool
    where
        Elem: Mul<Bias, Output = Bias>,
        F: Fn(VarIndex) -> VarIndex,
    {
        let val = self.lhs.evaluate_sample(sample, &index_map);
        self.comparator.evaluate(val, self.rhs)
    }

    /// A [VarRef] in our LHS [Expression] can be replaced by a new [Expression] using this function.
    /// It's a convenience function to enable the [substitution](Expression::substitute) operation on
    /// the LHS [Expression] of a constraint. All substitution logic and operations are defined on
    /// the [Expression] [here](Expression::substitute).
    pub fn substitute(
        &mut self,
        target: &VarRef,
        replacement: &Expression,
    ) -> Result<(), DifferentEnvsErr> {
        self.lhs = (&self.lhs).substitute(target, replacement)?;
        Ok(())
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_constraint(&self).to_string();
        f.write_str(&s)
    }
}

impl ContentEquality for Constraint {
    /// Since the [Constraint] (indirectly) has a reference to a [SharedEnvironment] we cannot simply
    /// check by equality using the builtin (derived) equality primitives. Since two [Constraint]s
    /// might be equal albeit being defined in different [SharedEnvironment]s. We still want to maintain
    /// the option to check if two [Constraint]s are identical (including the [SharedEnvironment])
    /// so we define this function to keep the `==` operator for identity checks.
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.lhs.is_equal_contents(&other.lhs)
            && self.rhs == other.rhs
            && self.comparator == other.comparator
            && self.name == other.name
    }
}

/// The Constraints struct is an insertion ordered collection of one or more [Constraint]s.
#[derive(Debug, Clone, PartialEq)]
pub struct Constraints {
    /// A map to help in indexing into this collection when [ConstraintKey::Str] is used.
    pub index_map: IndexMap<String, usize>,
    /// All [Constraint]s contained in the collection in the order they were added to [Self].
    pub constraints: Vec<Constraint>,
}

/// A [Constraint] can be either identified by an Int or a String. Access is unified by this enum.
///
/// Note: This is subject to change in the future to allow indexing only using a constraints name
/// (String) to ensure a more consistent and user safe API. In addition, it is required to enable
/// enhancements in the transformation stack for operations working on constraints. For more
/// details see <https://github.com/aqarios/aq-models-rs/issues/400>.
#[cfg_attr(feature = "py", derive(FromPyObject))]
pub enum ConstraintKey {
    Int(usize),
    Str(String),
}

impl Display for ConstraintKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(idx) => write!(f, "{}", idx),
            Self::Str(name) => write!(f, "{}", name),
        }
    }
}

impl Constraints {
    /// Since the [Constraints] collection (indirectly via the [Constraint]s' LHS [Expression]s) have a reference to
    /// a [SharedEnvironment] we cannot simply check by equality using the builin (derived) equality
    /// primitives. Since two [Constraint]s might be equal albeit being defined in different
    /// [SharedEnvironment]s.
    /// We still want to maintain the option to check if two [Constraint]s are identical (including the
    /// [SharedEnvironment]) so we define this function to keep the `==` operator for identity checks.
    pub fn deep_clone(&self, env: SharedEnvironment) -> Self {
        Self {
            index_map: self.index_map.clone(),
            constraints: self
                .constraints
                .iter()
                .map(|c| c.deep_clone(env.clone()))
                .collect(),
        }
    }
}

impl Constraints {
    /// Create an empty Constraints collection.
    pub fn default() -> Self {
        Self {
            index_map: IndexMap::new(),
            constraints: Vec::new(),
        }
    }

    /// Create a Constraints collection from another constraints collection. The newly created
    /// object has the same reference to the [SharedEnvironment] as the constraints it is created
    /// from. I.e., it does not use the [Constraint::deep_clone] for all constraints of the given
    /// constraints collection.
    pub fn new_from(other: &Self) -> Self {
        Self {
            index_map: other.index_map.clone(),
            constraints: other.constraints.clone(),
        }
    }

    /// Create a Constraints collection from multiple [Constraint] instances contained in a `Vec<_>`.
    pub fn new_from_vec(constraints: Vec<Constraint>) -> Self {
        let mut slf = Self::default();
        constraints.into_iter().enumerate().for_each(|(idx, c)| {
            let name = (&c).name.clone().unwrap_or(format!("c{idx}").to_string());
            slf.index_map.insert(name.clone(), idx);
            slf.constraints.push(c);
        });
        slf
    }

    /// Get the number of [Constraint]s contained in the constraints collection.
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Iterate over all [Constraint]s in the constraint collection.
    pub fn iter(&self) -> ConstraintsIterator<'_> {
        ConstraintsIterator::new(&self)
    }

    /// Check if the [Constraints] collection contains any data.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a specific [Constraint] from the constraints collection given a [ConstraintKey].
    /// Returns an error if no constraint exists at the index if [ConstraintKey::Int]` is
    /// passed or if no constraint for the name exists if [ConstraintKey::Str] is passed.
    pub fn get_constraint(&self, key: ConstraintKey) -> Result<&Constraint, GetConstraintErr> {
        let index = match &key {
            ConstraintKey::Int(idx) => Some(idx),
            ConstraintKey::Str(name) => self.index_map.get(name),
        };
        match index {
            Some(idx) => {
                let constr = self.constraints.get(*idx);
                match constr {
                    Some(constr) => Ok(constr),
                    Option::None => Err(GetConstraintErr::IndexOutOfBoundsErr(
                        IndexOutOfBoundsErr::new(*idx, self.len()),
                    )),
                }
            }
            Option::None => Err(GetConstraintErr::NoConstraintForKeyErr(key.to_string())),
        }
    }

    /// Set (overwrite) a new [Constraint] for the given [ConstraintKey]. A constraint for this key
    /// must already exist. If no constraint is available for the given [ConstraintKey] an error is
    /// returned.
    ///
    /// Note: This might change in the future when allowing indexing only using a constraint name
    /// (String) to ensure a more consistent and user safe API. This might allow users to use this
    /// function to register a new constraint in the constraints collection directly.
    /// For more details see <https://github.com/aqarios/aq-models-rs/issues/400>.
    pub fn set_constraint(
        &mut self,
        key: ConstraintKey,
        constr: Constraint,
    ) -> Result<(), GetConstraintErr> {
        let index = match &key {
            ConstraintKey::Int(idx) => Some(idx),
            ConstraintKey::Str(name) => self.index_map.get(name),
        };
        match index {
            Some(idx) => {
                self.constraints[*idx] = constr;
                Ok(())
            }
            Option::None => Err(GetConstraintErr::NoConstraintForKeyErr(key.to_string())),
        }
    }

    /// Remove a [Constraint] for the given [ConstraintKey]. If no cosntraint exists for the given
    /// [ConstraintKey], this function is esentially a no-op.
    pub fn remove_constraint(&mut self, key: ConstraintKey) {
        let (idx, name) = match &key {
            ConstraintKey::Int(idx) => {
                let v = self.index_map.iter().find(|(_, b)| **b == *idx);
                if let Some((name, _)) = v {
                    (*idx, name.clone())
                } else {
                    return ();
                }
            }
            ConstraintKey::Str(name) => {
                if let Some(idx) = self.index_map.get(name) {
                    (*idx, name.clone())
                } else {
                    return ();
                }
            }
        };
        self.constraints.remove(idx);
        self.index_map.shift_remove(&name);
        self.index_map
            .iter_mut()
            .filter(|(_, index)| **index > idx)
            .for_each(|(_, index)| *index -= 1);
    }

    /// A [VarRef] in the LHS [Expression] of all [Constraint]s in the [Constraints] collection can be
    /// replaced by a new [Expression] using this function.
    /// This is a convenience function to enable the [substitution operation](Constraint::substitute)
    /// on the LHS expression of all constraints within the constraints collection.
    /// All substitution logic and operations are defined on the [Expression]
    /// [here](Expression::substitute).
    pub fn substitute(
        &mut self,
        target: &VarRef,
        replacement: &Expression,
    ) -> Result<(), DifferentEnvsErr> {
        for constr in self.constraints.iter_mut() {
            constr.substitute(target, replacement)?;
        }
        Ok(())
    }

    /// In contrast to what this function might suggest based on it's name it has nothing to do
    /// with `C` (language) types. This function returns all **unique** [Comparator]s used by all
    /// [Constraint]s within this [Constraints] collection.
    pub fn ctypes(&self) -> Vec<Comparator> {
        self.constraints
            .iter()
            .map(|c| c.comparator)
            .unique()
            .collect_vec()
    }

    /// This function returns all **unique** [Variable types](Vtype) used by the LHS of all
    /// [Constraint]s within this [Constraints] collection.
    pub fn vtypes(&self) -> Vec<Vtype> {
        self.constraints
            .iter()
            .map(|c| c.lhs.vtypes())
            .flatten()
            .unique()
            .collect_vec()
    }

    /// Implements mutable adding a [Constraint] to the [Constraints] collection.
    /// This function does not implement the `+=` operation, and this function needs to be
    /// called to mutably add a [Constraint] to [Self].
    pub fn add_assign(&mut self, rhs: &Constraint) -> Result<(), DuplicateConstraintNameErr> {
        if let Some(name) = &rhs.name {
            if self.index_map.contains_key(name) {
                return Err(DuplicateConstraintNameErr(name.to_string()));
            } else {
                self.index_map
                    .insert(name.to_string(), self.constraints.len());
            }
        } else {
            let idx = self.constraints.len();
            self.index_map.insert(format!("c{}", idx), idx);
        }
        Ok(self.constraints.push(rhs.clone()))
    }
}

/// Implements adding a [Constraint] to the [Constraints] collection for a copy using the
/// `+` operation.
impl Add<Constraint> for Constraints {
    /// Addition can result in either a new [Constraints] instance or a [DuplicateConstraintNameErr].
    /// See [Self::add] for details.
    type Output = Result<Constraints, DuplicateConstraintNameErr>;
    /// Implements adding a [Constraint] to the [Constraints] collection using the `+` operation on
    /// [Constraints]. This operation might return an error in case the name of the constraint to
    /// be added is already registered within the target constraints collection ([DuplicateConstraintNameErr]).
    /// A new [Constraints] object is returned from this function based on `self`. This function does
    /// not edit `self`!
    fn add(self, rhs: Constraint) -> Self::Output {
        let mut out = Constraints::new_from(&self);
        out.add_assign(&rhs)?;
        Ok(out)
    }
}

impl Add<&Constraint> for &Constraints {
    type Output = Result<Constraints, DuplicateConstraintNameErr>;

    /// Borrowed version of [Constraints::add].
    fn add(self, rhs: &Constraint) -> Self::Output {
        let mut out = Constraints::new_from(&self);
        out.add_assign(rhs)?;
        Ok(out)
    }
}

impl PartialEq for Constraint {
    /// Check if two [Constraint]s are identical using `==` operation.
    fn eq(&self, other: &Self) -> bool {
        self.comparator == other.comparator && self.rhs == other.rhs && self.lhs == other.lhs
    }
}

impl Display for Constraints {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = ModelWriter::new().write_constraints(&self).to_string();
        f.write_str(&s)
    }
}

impl ContentEquality for Constraints {
    /// Check if two [Constraints] have equal contents. Uses [Constraint::is_equal_contents] for
    /// all [Constraint]s. See [here](Constraint::is_equal_contents) for details.
    fn is_equal_contents(&self, other: &Self) -> bool {
        self.index_map == other.index_map && self.constraints.is_equal_contents(&other.constraints)
    }
}

pub struct ConstraintsIterator<'a> {
    collection: &'a Constraints,
    names: Vec<&'a String>,
    indices: Vec<usize>,
    current: usize,
}

impl<'a> ConstraintsIterator<'a> {
    fn new(collection: &'a Constraints) -> Self {
        let mut names = Vec::new();
        let mut indices = Vec::new();
        collection.index_map.iter().for_each(|(key, idx)| {
            names.push(key);
            indices.push(*idx);
        });
        Self {
            collection,
            current: 0,
            indices,
            names,
        }
    }
}

impl<'a> Iterator for ConstraintsIterator<'a> {
    type Item = (&'a String, &'a Constraint);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.names.len() {
            return None;
        }
        let name = self.names[self.current];
        let index = self.indices[self.current];
        let constr = &self.collection.constraints[index];
        self.current += 1;
        Some((name, constr))
    }
}
