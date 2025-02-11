use crate::core::{
    environment::EnvId,
    exceptions::VariablesFromDifferentEnvsError,
    higher_order_operations::{
        TermC, TermMultiplication2, TermMultiplication3, TermVarMultiplicationC,
    },
    operations::{
        Term, TermAddition, TermConstantMultiplication, TermFloatMultiplication,
        TermMultiplication, TermSubtraction, TermVarMultiplication,
    },
    variable::VarId,
    Environment, VarRef, Vtype,
};
use std::{collections::HashMap, ops::MulAssign};

#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::{higher_order::HigherOrderKey, quadratic::QuadraticKey, HigherOrder, Quadratic};

#[cfg_attr(feature = "py", pyclass)]
#[derive(Clone, PartialEq)]
pub struct Linear {
    pub env_id: EnvId,
    pub variables: Option<HashMap<VarId, f64>>,
}

/// methods used to create a linear term efficiently.
impl Linear {
    pub fn empty(env_id: EnvId) -> Self {
        Self {
            env_id,
            variables: None,
        }
    }

    /// Efficient production of linear term for a single value.
    pub fn new(a: (&VarRef, f64)) -> Self {
        let (a_ref, av) = a;
        let mut variables = HashMap::new();
        variables.insert(a_ref.id, av);
        Self {
            // variables,
            variables: Some(variables),
            env_id: a_ref.env_id,
        }
    }
    /// Linear terms are created when two variables are added or subtracted.
    /// This generates either `a + b` or `a - b`
    /// What if a and b are equal? Then the sum of the passed values are stored.
    pub fn new_from_vars(
        a: (&VarRef, f64),
        b: (&VarRef, f64),
    ) -> Result<Self, VariablesFromDifferentEnvsError> {
        let (a_ref, av) = a;
        let (b_ref, bv) = b;

        if a_ref.env_id != b_ref.env_id {
            return Err(VariablesFromDifferentEnvsError);
        }

        let mut variables = HashMap::new();
        if a_ref.id == b_ref.id {
            variables.insert(a_ref.id, av + bv);
        } else {
            variables.insert(a_ref.id, av);
            variables.insert(b_ref.id, bv);
        }
        Ok(Self {
            variables: Some(variables),
            // variables,
            env_id: a_ref.env_id,
        })
    }

    pub fn as_string(&self, environment: &Environment) -> String {
        match &self.variables {
            Some(vs) => vs
                .iter()
                .map(|(key, value)| {
                    let var = environment.get(key);
                    if *value == 1.0 {
                        format!("{}", var.name)
                    } else if *value < 0.0 {
                        format!("{} * {}", -value, var.name)
                    } else {
                        format!("{} * {}", value, var.name)
                    }
                })
                .collect::<Vec<String>>()
                .join(" + "),
            None => String::from(""),
        }
    }

    pub fn append_variable(&mut self, var: &VarRef, value: Option<f64>) {
        match value {
            None => (),
            Some(v) => {
                if v == 0.0 {
                    return;
                }
                match self.has_variables() {
                    true => {
                        let vars = self.mutable_variables();
                        vars.insert(var.id, v);
                    }
                    false => {
                        let mut new = HashMap::new();
                        new.insert(var.id, v);
                        self.variables = Some(new);
                    }
                }
            }
        }
    }

    pub fn append_elem(&mut self, key: u32, value: f64) {
        match self.has_variables() {
            false => {
                let mut nh = HashMap::new();
                nh.insert(key, value);
                self.variables = Some(nh);
            }
            true => {
                self.mutable_variables().insert(key, value);
            }
        }
    }

    pub fn append(&mut self, other: Option<Self>) {
        match other {
            None => (),
            Some(l) => match self.has_variables() {
                true => match l.has_variables() {
                    true => {
                        let selfvars = self.mutable_variables();
                        for (key, value) in l.variables().iter() {
                            selfvars.insert(*key, *value);
                        }
                    }
                    false => (),
                },
                false => self.variables = l.variables.clone(),
            },
        }
    }
}

impl Term<VarId> for Linear {
    fn empty(env_id: EnvId) -> Self {
        Self {
            env_id,
            variables: None,
        }
    }

    fn new_from_other(other: &Self) -> Self {
        match &other.variables {
            Some(v) => Self {
                env_id: other.env_id,
                variables: Some(v.clone()),
            },
            None => Self {
                env_id: other.env_id,
                variables: None,
            },
        }
    }

    fn reset(&mut self) {
        self.variables = None
    }

    fn has_variables(&self) -> bool {
        self.variables.is_some()
    }

    fn mutable_variables(&mut self) -> &mut HashMap<VarId, f64> {
        self.variables.as_mut().unwrap()
    }

    fn variables(&self) -> &HashMap<VarId, f64> {
        self.variables.as_ref().unwrap()
    }

    fn fill_variables(&mut self, variables: HashMap<VarId, f64>) -> &mut HashMap<VarId, f64> {
        self.variables.insert(variables)
    }

    fn env_id(&self) -> EnvId {
        self.env_id
    }

    // fn get_vtype(&self, key: &VarId, environment: &Environment) -> Vtype {
    //     // todo: can we remove this clone?
    //     environment.get(key).vtype.clone()
    // }
}

impl TermAddition<VarId> for Linear {}
impl TermSubtraction<VarId> for Linear {}
impl TermFloatMultiplication<VarId> for Linear {}
impl TermConstantMultiplication<VarId> for Linear {}

impl TermVarMultiplication<VarId, Quadratic, QuadraticKey> for Linear {
    fn mul(&self, rhs: &VarRef, environment: &Environment) -> (Self, Option<Quadratic>) {
        if !self.has_variables() {
            return (Linear::empty(self.env_id), None);
        }
        // We are dealing if a trivial variable here in the sense that it does not have a
        // factor associated yet, i.e., the factor of the variable is 1.0. Thus we can
        // take a lot of shortcuts here that are not directly applicable to the multiplication
        // with a variable from another expression, where the facctor of the variable can be
        // anything.
        //
        // This method is esentially only checking if a new quadratic term is created by
        // the multiplication.
        let mut out = Self::new_from_other(&self);
        let outvars = out.mutable_variables();

        let mut quadratic: Option<Quadratic> = None;

        for (key, value) in self.variables().iter() {
            let cur_vtype = environment.get(key).vtype;

            if *key == rhs.id {
                // The two variables are equal.
                //
                // We need to check the variable types. If Binary or Spin
                // it remains linear after multiplication, else it will
                // be quadratic.
                //
                // We can just look at a single vtype.
                match cur_vtype {
                    Vtype::Binary => (),
                    Vtype::Spin => (),
                    _ => {
                        // creating the new quadratic expression;
                        if quadratic.is_none() {
                            quadratic = Some(Quadratic::new_from_vars_with_value(key, rhs, *value));
                        } else {
                            quadratic
                                .as_mut()
                                .unwrap()
                                .append_elem(key, &rhs.id, *value);
                        }
                        // match quadratic {
                        //     None => quadratic = Some(new_quadratic),
                        //     Some(q) => q.append(&new_quadratic),
                        // }
                        // The current key can be removed from the output.
                        outvars.remove(key);
                    }
                }
            } else {
                // The two variables are not equal. Always result in a new
                // quadratic term.
                if quadratic.is_none() {
                    quadratic = Some(Quadratic::new_from_vars_with_value(key, rhs, *value));
                } else {
                    quadratic
                        .as_mut()
                        .unwrap()
                        .append_elem(key, &rhs.id, *value);
                }
                // match &quadratic {
                //     None => quadratic = Some(new_quadratic),
                //     Some(q) => q.append(&new_quadratic),
                // }
                // The current key can be removed from the output.
                outvars.remove(key);
            }
        }

        (out, quadratic)
    }
}

impl TermMultiplication<VarId, Linear, Quadratic, QuadraticKey> for Linear {
    fn mul(&self, rhs: &Linear, environment: &Environment) -> (Self, Option<Quadratic>) {
        if !self.has_variables() {
            return (Self::empty(self.env_id), None);
        }
        // let res = match (self.has_variables(), rhs.has_variables()) {
        //     (false, false) => Some(Self::empty(self.env_id)),
        //     (false, true) => Some(Self::empty(rhs.env_id)),
        //     (true, false) => Some(Self::new_from_other(&self)),
        //     (true, true) => None,
        // };
        // if res.is_some() {
        //     return (res.unwrap(), None);
        // }

        let mut out = Self::new_from_other(&self);
        let outvars = out.mutable_variables();

        let mut quadratic: Option<Quadratic> = None;

        for (key_a, value_a) in self.variables().iter() {
            for (key_b, value_b) in rhs.variables().iter() {
                if key_a == key_b {
                    // Same key, we need to check if it's Binary or Spin.
                    let vtype = environment.get(key_a).vtype;
                    if vtype == Vtype::Binary || vtype == Vtype::Spin {
                        // Alright. here we need to do some work on the value.
                        match outvars.get_mut(key_a) {
                            Some(e) => e.mul_assign(value_b),
                            None => (),
                        }
                    } else {
                        // A quadratic contribution is generated.
                        let newval = value_a * value_b;
                        if quadratic.is_none() {
                            quadratic = Some(Quadratic::new_from_keys_with_value(
                                self.env_id,
                                key_a,
                                key_b,
                                newval,
                            ))
                        } else {
                            quadratic
                                .as_mut()
                                .unwrap()
                                .append_elem(key_a, key_b, newval);
                        }
                        outvars.remove(key_a);
                    }
                } else {
                    // A quadratic contribution is generated.
                    let newval = value_a * value_b;
                    if quadratic.is_none() {
                        quadratic = Some(Quadratic::new_from_keys_with_value(
                            self.env_id,
                            key_a,
                            key_b,
                            newval,
                        ))
                    } else {
                        quadratic
                            .as_mut()
                            .unwrap()
                            .append_elem(key_a, key_b, newval);
                    }
                    outvars.remove(key_a);
                }
            }
        }

        (out, quadratic)
    }
}

impl TermMultiplication3<VarId, QuadraticKey, Quadratic, HigherOrderKey, HigherOrder> for Linear {
    fn mul(
        &self,
        rhs: &Quadratic,
        environment: &Environment,
    ) -> (Self, Option<Quadratic>, Option<HigherOrder>) {
        if !self.has_variables() {
            return (Self::empty(self.env_id), None, None);
        }

        let mut lin = Self::new_from_other(&self);
        let linvars = lin.mutable_variables();

        if !rhs.has_variables() {
            return (lin, None, None);
        }

        let mut quad = Quadratic::new_from_other(&rhs);
        let quadvars = quad.mutable_variables();

        let mut higherorder: Option<HigherOrder> = None;

        for (linkey, linval) in self.variables().iter() {
            for (quadkey, quadval) in rhs.variables().iter() {
                // do something.
                let (quad_contrib_a, quad_contrib_b) = Quadratic::get_key_contributions(quadkey);

                if quad_contrib_a == *linkey || quad_contrib_b == *linkey {
                    // Check if Binary or Spin.
                    let vtype = environment.get(linkey).vtype;
                    if vtype == Vtype::Binary || vtype == Vtype::Spin {
                        match quadvars.get_mut(quadkey) {
                            None => (),
                            Some(e) => e.mul_assign(linval),
                        }
                        linvars.remove(linkey);
                    } else {
                        // A higher order contribution is generated.
                        let newval = linval * quadval;
                        if higherorder.is_none() {
                            higherorder = Some(HigherOrder::new_from_keys_with_value(
                                self.env_id,
                                *linkey,
                                quad_contrib_a,
                                quad_contrib_b,
                                newval,
                            ))
                        } else {
                            higherorder.as_mut().unwrap().append_elem(
                                *linkey,
                                quad_contrib_a,
                                quad_contrib_b,
                                newval,
                            )
                        }
                        linvars.remove(linkey);
                        quadvars.remove(quadkey);
                    }
                } else {
                    // A higher order contribution is generated.
                    let newval = linval * quadval;
                    if higherorder.is_none() {
                        higherorder = Some(HigherOrder::new_from_keys_with_value(
                            self.env_id,
                            *linkey,
                            quad_contrib_a,
                            quad_contrib_b,
                            newval,
                        ))
                    } else {
                        higherorder.as_mut().unwrap().append_elem(
                            *linkey,
                            quad_contrib_a,
                            quad_contrib_b,
                            newval,
                        )
                    }
                    linvars.remove(linkey);
                    quadvars.remove(quadkey);
                }
            }
        }

        (lin, Some(quad), higherorder)
    }
}

impl TermMultiplication2<VarId, HigherOrderKey, HigherOrder> for Linear {
    fn mul(&self, rhs: &HigherOrder, environment: &Environment) -> (Self, Option<HigherOrder>) {
        if !self.has_variables() {
            return (Self::empty(self.env_id), None);
        }

        let mut lin = Self::new_from_other(&self);
        let linvars = lin.mutable_variables();

        if !rhs.has_variables() {
            return (lin, None);
        }

        let mut ho = HigherOrder::new_from_other(&rhs);
        let hovars = ho.mutable_variables();

        for (linkey, linval) in self.variables().iter() {
            for (hokey, hoval) in rhs.variables().iter() {
                let hocontribs = HigherOrder::get_key_contributions(hokey.to_string());

                let mut is_key_contained: bool = false;
                for hocontrib in hocontribs.iter() {
                    if !is_key_contained && hocontrib == linkey {
                        is_key_contained = true;
                    }
                }

                let vtype = environment.get(linkey).vtype;
                if is_key_contained && (vtype == Vtype::Binary || vtype == Vtype::Spin) {
                    // We just need to update the value in the higher order term and remove
                    // it from the linear term.
                    match hovars.get_mut(hokey) {
                        None => (),
                        Some(e) => e.mul_assign(linval),
                    }
                    linvars.remove(linkey);
                } else {
                    // A new higher order entry is generated.
                    let newval = linval * hoval;
                    let newkey = HigherOrder::update_key(hokey.to_string(), *linkey);
                    hovars.insert(newkey, newval);

                    hovars.remove(hokey);
                    linvars.remove(linkey);
                }
            }
        }

        (lin, Some(ho))
    }
}
