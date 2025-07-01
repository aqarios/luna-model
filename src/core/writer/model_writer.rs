use crate::core::constraints::Constraints;
use crate::core::expression::One;
use crate::core::term::{HigherOrder, Linear, Quadratic};
use crate::core::writer::line_length_restrictor::LineLengthRestrictor;
use crate::core::{Bound, Constraint, Expression, Model, SharedEnvironment, Variable, Vtype};
use crate::types::Bias;
use hashbrown::HashMap;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;

pub struct ModelWriter {
    writer: LineLengthRestrictor,
    is_first: bool,
}

impl ModelWriter {
    pub fn new() -> Self {
        Self {
            writer: LineLengthRestrictor::new(0),
            is_first: true,
        }
    }

    pub fn write_model(&mut self, model: &Model) -> &mut Self {
        self.writer
            .write(&format!("Model: {}", model.name))
            .with_new_line(&model.sense.to_string())
            .increase_indent()
            .new_line();
        self.write_expression(&model.objective);
        self.writer.decrease_indent();
        let constraints = &model.constraints;
        if constraints.len() > 0 {
            self.writer
                .with_new_line("Subject To")
                .increase_indent()
                .new_line();
            self.write_constraints(&constraints);
            self.writer.decrease_indent();
        }
        self.write_bounds(&model.environment);
        self.write_variables(&model.environment)
    }

    pub fn write_expression(&mut self, expr: &Expression) -> &mut Self {
        if let Some(higher_order) = &expr.higher_order {
            self.write_higher_order(&expr.env, higher_order);
        }
        if let Some(quadratic) = &expr.quadratic {
            self.write_quadratic(&expr.env, quadratic);
        }
        self.write_linear(&expr.env, &expr.linear);
        if expr.offset != Bias::default() {
            self.write_offset(&expr.offset);
        }
        self.is_first = true;
        self
    }

    pub fn write_higher_order(
        &mut self,
        env: &SharedEnvironment,
        higher_order: &HigherOrder,
    ) -> &mut Self {
        for (indices, bias) in higher_order.iter_contrib() {
            if *bias != Bias::default() {
                let vnames = indices
                    .iter()
                    .map(|&idx| env.borrow()[idx].name.clone())
                    .collect::<Vec<_>>()
                    .join(" * ");
                if !self.is_first {
                    self.writer.space();
                }
                let s = format!("{}{vnames}", self.show_bias(bias));
                self.writer.write(&s);
                self.is_first = false;
            }
        }
        self
    }

    pub fn write_quadratic(&mut self, env: &SharedEnvironment, quadratic: &Quadratic) -> &mut Self {
        for (i, j, bias) in quadratic.iter_flat() {
            if bias != Bias::default() {
                let v_i = env.borrow()[i].name.clone();
                let v_j = env.borrow()[j].name.clone();
                if !self.is_first {
                    self.writer.space();
                }
                let s = format!("{}{v_i} * {v_j}", self.show_bias(&bias));
                self.writer.write(&s);
                self.is_first = false;
            }
        }
        self
    }

    pub fn write_linear(&mut self, env: &SharedEnvironment, linear: &Linear) -> &mut Self {
        for (i, bias) in linear.iter() {
            if *bias != Bias::default() {
                if !self.is_first {
                    self.writer.space();
                }
                let s = format!("{}{}", self.show_bias(bias), env.borrow()[i].name);
                self.writer.write(&s);
                self.is_first = false;
            }
        }
        self
    }

    pub fn write_offset(&mut self, bias: &Bias) -> &mut Self {
        let bias_string = bias.to_string();
        if self.is_first {
            self.writer.write(&bias_string);
        } else if *bias < Bias::default() {
            self.writer
                .space()
                .write(&format!("- {}", &bias_string[1..]));
        } else {
            self.writer.space().write(&format!("+ {}", &bias_string));
        }
        self
    }

    fn show_bias(&mut self, bias: &Bias) -> String {
        if *bias == Bias::one() {
            String::from(if self.is_first { "" } else { "+ " })
        } else if *bias == -Bias::one() {
            String::from(if self.is_first { "-" } else { "- " })
        } else if *bias < Bias::default() {
            let bias_string = bias.to_string();
            if self.is_first {
                format!("{bias_string} * ")
            } else {
                format!("- {} * ", &bias_string[1..])
            }
        } else {
            format!(
                "{}{} * ",
                if self.is_first { "" } else { "+ " },
                &bias.to_string()
            )
        }
    }

    pub fn write_constraints(&mut self, constraints: &Constraints) -> &mut Self {
        for (i, constr) in constraints.iter().enumerate() {
            if i > 0 {
                self.writer.new_line();
            }
            self.writer
                .write(&format!(
                    "{}: ",
                    constr.name.clone().unwrap_or(format!("c{i}"))
                ))
                .increase_indent();
            self.write_constraint(constr);
            self.writer.decrease_indent();
        }
        self
    }

    pub fn write_constraint(&mut self, constraint: &Constraint) -> &mut Self {
        self.write_expression(&constraint.lhs);
        self.writer
            .with_spaces(&constraint.comparator.to_string())
            .write(&constraint.rhs.to_string());
        self
    }

    pub fn write_bounds(&mut self, environment: &SharedEnvironment) -> &mut Self {
        let binding = environment.borrow();
        let vars = binding.variables();
        let ints_and_reals: Vec<_> = vars
            .into_iter()
            .filter(|v| v.vtype == Vtype::Integer || v.vtype == Vtype::Real)
            .collect();
        if ints_and_reals.len() == 0 {
            return self;
        }

        self.writer
            .with_new_line("Bounds")
            .increase_indent()
            .new_line();
        for (i, var) in ints_and_reals.iter().enumerate() {
            if i > 0 {
                self.writer.new_line();
            }
            // Only print var bounds for integer and real, ignore binary and spin
            if var.bounds.lower.is_bounded() || var.bounds.upper.is_bounded() {
                if let Bound::Some(l) = var.bounds.lower {
                    self.writer.write(&l.to_string()).with_spaces("<=");
                }
                self.writer.write(&var.name);
                if let Bound::Some(u) = var.bounds.upper {
                    self.writer.with_spaces("<=").write(&u.to_string());
                }
            } else {
                self.writer.write(&var.name).space().write("unbounded");
            }
        }
        self.writer.decrease_indent();
        self
    }

    pub fn write_variables(&mut self, env: &SharedEnvironment) -> &mut Self {
        let mut var_map = HashMap::new();
        for var in env.borrow().variables() {
            var_map
                .entry(var.vtype.to_string())
                .or_insert(vec![])
                .push(var.name.clone());
        }
        for vtype in Vtype::iter() {
            if let Some(var_names) = var_map.remove(&vtype.to_string()) {
                self.writer
                    .with_new_line(&vtype.to_string())
                    .increase_indent()
                    .new_line();
                for (i, var_name) in var_names.iter().enumerate() {
                    if i > 0 {
                        self.writer.space();
                    }
                    self.writer.write(var_name);
                }
                self.writer.decrease_indent();
            }
        }
        self
    }
}

impl Display for ModelWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.writer.to_string())
    }
}
