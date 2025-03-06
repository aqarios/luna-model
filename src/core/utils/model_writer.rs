use crate::core::constraints::Constraints;
use crate::core::expression::{BiasConstraints, IndexConstraints};
use crate::core::term::{HigherOrder, Linear, Quadratic};
use crate::core::{Constraint, Environment, Expression, Model, Vtype};
use hashbrown::HashMap;
use std::cell::Ref;
use std::marker::PhantomData;

const MAX_LINE_LENGTH: usize = 80;
const INDENTATION: usize = 2;

pub struct LineLengthRestrictor {
    current_string: String,
    current_line_length: usize,
    indent: usize,
}

impl LineLengthRestrictor {
    pub fn new(indent: usize) -> Self {
        Self {
            current_string: String::new(),
            current_line_length: 0,
            indent,
        }
    }

    pub fn write(&mut self, s: &str) {
        if self.current_line_length + s.len() > MAX_LINE_LENGTH {
            self.new_line();
        }
        self.current_string += s;
        self.current_line_length += s.len();
    }

    pub fn new_line(&mut self) {
        self.current_string += &format!("\n{}", " ".repeat(self.indent));
        self.current_line_length = self.indent
    }

    pub fn space(&mut self) {
        if self.current_line_length < MAX_LINE_LENGTH {
            self.current_string += " ";
            self.current_line_length += 1;
        } else {
            self.new_line();
        }
    }

    pub fn with_new_line(&mut self, s: &str) {
        self.new_line();
        self.write(s);
    }

    pub fn with_spaces(&mut self, s: &str) {
        self.space();
        self.write(s);
        self.space();
    }

    pub fn increase_indent(&mut self) {
        self.indent += INDENTATION;
    }

    pub fn decrease_indent(&mut self) {
        self.indent -= INDENTATION;
    }

    pub fn to_string(self) -> String {
        self.current_string
    }
}

pub struct ModelWriter<Index, Bias> {
    writer: LineLengthRestrictor,
    is_first: bool,
    phantom_index: PhantomData<Index>,
    phantom_bias: PhantomData<Bias>,
}

impl<Index, Bias> ModelWriter<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub fn new() -> Self {
        Self {
            writer: LineLengthRestrictor::new(0),
            is_first: true,
            phantom_index: PhantomData,
            phantom_bias: PhantomData,
        }
    }

    pub fn write_model(&mut self, model: &Model<Index, Bias>) {
        self.writer.write(&format!("Model: {}", model.name));
        self.writer.with_new_line("Minimize"); // TODO: replace with model.sense
        self.writer.increase_indent();
        self.writer.new_line();
        self.write_expression(&model.objective.borrow());
        self.writer.decrease_indent();
        let constraints = model.constraints.borrow();
        if constraints.len() > 0 {
            self.writer.with_new_line("Subject To");
            self.writer.increase_indent();
            self.writer.new_line();
            self.write_constraints(&constraints);
            self.writer.decrease_indent();
        }
        self.writer.with_new_line("Bounds");
        self.writer.increase_indent();
        self.writer.new_line();
        self.write_bounds(model.environment.borrow());
        self.writer.decrease_indent();
        self.write_variables(model.environment.borrow());
    }

    pub fn write_expression(&mut self, expr: &Expression<Index, Bias>) {
        if let Some(higher_order) = &expr.higher_order {
            self.write_higher_order(expr.env.borrow(), higher_order);
        }
        if let Some(quadratic) = &expr.quadratic {
            self.write_quadratic(expr.env.borrow(), quadratic);
        }
        self.write_linear(expr.env.borrow(), &expr.linear);
        if expr.offset != Bias::zero() {
            self.write_offset(&expr.offset);
        }
        self.is_first = true;
    }

    pub fn write_higher_order(
        &mut self,
        env: Ref<Environment<Index>>,
        higher_order: &HigherOrder<Index, Bias>,
    ) {
        for (indices, bias) in higher_order.iter_contrib() {
            if *bias != Bias::zero() {
                let vnames = indices
                    .iter()
                    .map(|&idx| env.variables[idx.into()].name.clone())
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
    }

    pub fn write_quadratic(
        &mut self,
        env: Ref<Environment<Index>>,
        quadratic: &Quadratic<Index, Bias>,
    ) {
        for (i, j, bias) in quadratic.iter_flat() {
            if bias != Bias::zero() {
                let v_i = env.variables[i.into()].name.clone();
                let v_j = env.variables[j.into()].name.clone();
                if !self.is_first {
                    self.writer.space();
                }
                let s = format!("{}{v_i} * {v_j}", self.show_bias(&bias));
                self.writer.write(&s);
                self.is_first = false;
            }
        }
    }

    pub fn write_linear(&mut self, env: Ref<Environment<Index>>, linear: &Linear<Bias>) {
        for (i, bias) in linear.iter() {
            if *bias != Bias::zero() {
                if !self.is_first {
                    self.writer.space();
                }
                let s = format!("{}{}", self.show_bias(bias), env.variables[i].name);
                self.writer.write(&s);
                self.is_first = false;
            }
        }
    }

    pub fn write_offset(&mut self, bias: &Bias) {
        let bias_string = bias.to_string();
        if self.is_first {
            self.writer.write(&bias_string);
        } else if *bias < Bias::zero() {
            self.writer.space();
            self.writer.write(&format!("- {}", &bias_string[1..]));
        } else {
            self.writer.space();
            self.writer.write(&format!("+ {}", &bias_string));
        }
    }

    fn show_bias(&mut self, bias: &Bias) -> String {
        if *bias == Bias::one() {
            String::from(if self.is_first { "" } else { "+ " })
        } else if Some(bias) == Bias::negative_one().as_ref() {
            String::from(if self.is_first { "-" } else { "- " })
        } else if *bias < Bias::zero() {
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

    pub fn write_constraints(&mut self, constraints: &Constraints<Index, Bias>) {
        for (i, constr) in constraints.iter().enumerate() {
            if i > 0 {
                self.writer.new_line();
            }
            self.writer.write(&format!("c{i}: ")); // TODO: replace with constraint name
            self.writer.increase_indent();
            self.write_constraint(constr);
            self.writer.decrease_indent();
        }
    }

    pub fn write_constraint(&mut self, constraint: &Constraint<Index, Bias>) {
        self.write_expression(&constraint.lhs.borrow());
        self.writer.with_spaces(&constraint.comparator.to_string());
        self.writer.write(&constraint.rhs.to_string());
    }

    pub fn write_bounds(&mut self, env: Ref<Environment<Index>>) {
        for (i, var) in env.iter().enumerate() {
            if i > 0 {
                self.writer.new_line();
            }
            if var.bounds.lower.is_some() || var.bounds.upper.is_some() {
                if let Some(l) = var.bounds.lower {
                    self.writer.write(&l.to_string());
                    self.writer.with_spaces("<=");
                }
                self.writer.write(&var.name);
                if let Some(u) = var.bounds.upper {
                    self.writer.with_spaces("<=");
                    self.writer.write(&u.to_string());
                }
            } else {
                self.writer.write(&var.name);
                self.writer.write(" unbounded");
            }
        }
    }

    pub fn write_variables(&mut self, env: Ref<Environment<Index>>) {
        let mut var_map = HashMap::new();
        for var in env.iter() {
            var_map
                .entry(var.vtype)
                .or_insert(vec![])
                .push(var.name.clone());
        }
        for vtype in Vtype::iter() {
            if let Some(var_names) = var_map.remove(&vtype) {
                self.writer.with_new_line(&vtype.to_string());
                self.writer.increase_indent();
                self.writer.new_line();
                for (i, var_name) in var_names.iter().enumerate() {
                    if i > 0 {
                        self.writer.space();
                    }
                    self.writer.write(var_name);
                }
                self.writer.decrease_indent();
            }
        }
    }

    pub fn to_string(self) -> String {
        self.writer.to_string()
    }
}
