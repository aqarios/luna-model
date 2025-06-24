use crate::{
    core::{Comparator, Constraints, Expression, Model, SharedEnvironment, Vtype},
    serialization::{encodable::BytesEncodable, utils::force_u32},
};

use super::{SerConstraints, SerEnvironment, SerExpression, SerModel};

pub fn encode_for_hash(model: &Model) -> Vec<u8> {
    let sermodel = SerModel {
        objective: encode_expr_for_hash(&model.objective),
        constraints: encode_constrs_for_hash(&model.constraints),
        environment: encode_env_for_hash(&model.environment),
        sense: model.sense.to_string(),
        name: model.name.clone(),
    };
    sermodel.encode_to_bytes()
}

fn encode_expr_for_hash(expr: &Expression) -> Vec<u8> {
    let mut serexpr = SerExpression {
        num_variables: force_u32(expr.num_variables),
        active: expr.active.clone(),
        offset: expr.offset,
        linear: expr.linear.to_vec().clone(),
        quad_size: u32::default(),
        quad_neighborhood_indices: Vec::new(),
        quad_neighborhoods: Vec::new(),
        quad_neighborhoods_values: Vec::new(),
        quad_neighborhoods_len: Vec::new(),
        ho_size: u32::default(),
        ho_values: Vec::new(),
        ho_indices: Vec::new(),
        ho_lens: Vec::new(),
    };

    if let Some(quad) = &expr.quadratic {
        serexpr.quad_size = force_u32(quad.len());
        for (u, neighborhood) in quad.iter() {
            if !neighborhood.is_empty() {
                // only store data if the neighborhood is not empty.
                serexpr.quad_neighborhood_indices.push(force_u32(u));
                serexpr
                    .quad_neighborhoods_len
                    .push(force_u32(neighborhood.len()));
                neighborhood.iter().for_each(|e| {
                    serexpr.quad_neighborhoods.push(e.index.0);
                    serexpr.quad_neighborhoods_values.push(e.bias);
                });
            }
        }
    }

    if let Some(ho) = &expr.higher_order {
        serexpr.ho_size = force_u32(ho.len());
        for (ids, bias) in ho.iter_contrib() {
            serexpr.ho_lens.push(force_u32(ids.len()));
            serexpr.ho_values.push(*bias);
            ids.iter().for_each(|id| {
                serexpr.ho_indices.push(id.0);
            });
        }
    }
    serexpr.encode_to_bytes()
}

fn encode_constrs_for_hash(constrs: &Constraints) -> Vec<u8> {
    let mut serconstrs = SerConstraints {
        lhsides: Vec::new(),
        rhsides: Vec::new(),
        comparators: Vec::new(),
        names: Vec::new(),
    };
    for c in &constrs.constraints {
        let lhs_bytes = encode_expr_for_hash(&c.lhs);

        let comparator = match c.comparator {
            Comparator::Le => 0,
            Comparator::Eq => 1,
            Comparator::Ge => 2,
        };
        serconstrs.lhsides.push(lhs_bytes);
        serconstrs.rhsides.push(c.rhs);
        serconstrs.comparators.push(comparator);
        serconstrs
            .names
            .push(c.name.clone().unwrap_or("<NN>".to_string()));
    }

    serconstrs.encode_to_bytes()
}

fn encode_env_for_hash(env: &SharedEnvironment) -> Vec<u8> {
    let mut serenv = SerEnvironment {
        // the id was 0 (zero) for all environments in a past version. So we set it to exactly
        // this value.
        id: 0,
        varcount: env.borrow().varcount.0,
        binary: Vec::new(),
        spin: Vec::new(),
        integer: Vec::new(),
        real: Vec::new(),
        binary_names: Vec::new(),
        spin_names: Vec::new(),
        integer_names: Vec::new(),
        real_names: Vec::new(),
        integer_bounds_bounded_lower: Vec::new(),
        integer_bounds_bounded_upper: Vec::new(),
        integer_bounds_lower: Vec::new(),
        integer_bounds_upper: Vec::new(),
        real_bounds_bounded_lower: Vec::new(),
        real_bounds_bounded_upper: Vec::new(),
        real_bounds_lower: Vec::new(),
        real_bounds_upper: Vec::new(),
    };

    for (i, var) in env.borrow().variables.iter().enumerate() {
        match var.vtype {
            Vtype::Binary => {
                serenv.binary.push(force_u32(i));
                serenv.binary_names.push(var.name.clone());
            }
            Vtype::Spin => {
                serenv.spin.push(force_u32(i));
                serenv.spin_names.push(var.name.clone());
            }
            Vtype::Integer => {
                serenv.integer.push(force_u32(i));
                serenv.integer_names.push(var.name.clone());

                if var.bounds.lower.is_bounded() {
                    serenv.integer_bounds_bounded_lower.push(true);
                    serenv.integer_bounds_lower.push(var.bounds.lower.unwrap());
                } else {
                    serenv.integer_bounds_bounded_lower.push(false);
                }
                if var.bounds.upper.is_bounded() {
                    serenv.integer_bounds_bounded_upper.push(true);
                    serenv.integer_bounds_upper.push(var.bounds.upper.unwrap());
                } else {
                    serenv.integer_bounds_bounded_upper.push(false);
                }
            }
            Vtype::Real => {
                serenv.real.push(force_u32(i));
                serenv.real_names.push(var.name.clone());
                if var.bounds.lower.is_bounded() {
                    serenv.real_bounds_bounded_lower.push(true);
                    serenv.real_bounds_lower.push(var.bounds.lower.unwrap());
                } else {
                    serenv.real_bounds_bounded_lower.push(false);
                }
                if var.bounds.upper.is_bounded() {
                    serenv.real_bounds_bounded_upper.push(true);
                    serenv.real_bounds_upper.push(var.bounds.upper.unwrap());
                } else {
                    serenv.real_bounds_bounded_upper.push(false);
                }
            }
        }
    }

    serenv.encode_to_bytes()
}
