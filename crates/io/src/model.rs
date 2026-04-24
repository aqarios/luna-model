use std::fmt::Formatter;

use lunamodel_core::Model;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Model {
    fn fmt(&self, fmt: &mut Formatter<'_>, format_opt: &FormatOpt) -> std::fmt::Result {
        match format_opt {
            FormatOpt::Rs => {
                let s = pymodelio::ModelWriter::new().write_model(self).to_string();
                fmt.write_str(&s)
            }
            #[cfg(feature = "py")]
            FormatOpt::Py => {
                let s = pymodelio::ModelWriter::new().write_model(self).to_string();
                fmt.write_str(&s)
            }
            #[cfg(feature = "py")]
            FormatOpt::PySol(_) => unreachable!("cannot format Model for PySol opts"),
        }
    }

    fn dbg(&self, fmt: &mut Formatter<'_>, format_opt: &FormatOpt) -> std::fmt::Result {
        match format_opt {
            FormatOpt::Rs => write!(fmt, "{:?}", self),
            #[cfg(feature = "py")]
            FormatOpt::Py => write!(
                fmt,
                "Model(name={}, sense={}, objective={}, constraints={})",
                self.name,
                self.sense,
                self.objective.format(FormatOpt::Py),
                self.constraints.format(FormatOpt::Py)
            ),
            #[cfg(feature = "py")]
            FormatOpt::PySol(_) => unreachable!("cannot format Model for PySol opts"),
        }
    }
}

// #[cfg(feature = "py")]
mod pymodelio {
    use std::fmt::{Display, Formatter};

    use lunamodel_core::{
        ArcEnv, ConstraintCollection, Expression, Model,
        prelude::{Constraint, HigherOrder, Linear, Quadratic},
    };
    use lunamodel_types::{Bias, Bound, Vtype};
    use std::collections::HashMap;
    use strum::IntoEnumIterator;

    const MAX_LINE_LENGTH: usize = 80;
    const INDENTATION: usize = 2;

    struct LineLengthRestrictor {
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

        pub fn write(&mut self, s: &str) -> &mut Self {
            if self.current_line_length + s.len() > MAX_LINE_LENGTH {
                self.current_string = self.current_string.trim().to_string();
                self.new_line();
            }
            self.current_string += s;
            self.current_line_length += s.len();
            self
        }

        pub fn new_line(&mut self) -> &mut Self {
            self.current_string += &format!("\n{}", " ".repeat(self.indent));
            self.current_line_length = self.indent;
            self
        }

        pub fn space(&mut self) -> &mut Self {
            // todo: this function is buggy, as it adds trailing spaces to finished lines...
            // change to LP translator probably best in the long term.
            if self.current_line_length < MAX_LINE_LENGTH {
                self.current_string += " ";
                self.current_line_length += 1;
                self
            } else {
                self.new_line()
            }
        }

        pub fn with_new_line(&mut self, s: &str) -> &mut Self {
            self.new_line().write(s)
        }

        pub fn with_spaces(&mut self, s: &str) -> &mut Self {
            self.space().write(s).space()
        }

        pub fn increase_indent(&mut self) -> &mut Self {
            self.indent += INDENTATION;
            self
        }

        pub fn decrease_indent(&mut self) -> &mut Self {
            self.indent -= INDENTATION;
            self
        }
    }

    impl Display for LineLengthRestrictor {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.current_string)
        }
    }

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
            if !constraints.is_empty() {
                self.writer
                    .with_new_line("Subject To")
                    .increase_indent()
                    .new_line();
                self.write_constraints(constraints);
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
            env: &ArcEnv,
            higher_order: &HigherOrder,
        ) -> &mut Self {
            for (indices, bias) in higher_order.iter_contrib() {
                if bias != Bias::default() {
                    let vnames = indices
                        .iter()
                        .map(|&idx| env.read_arc()[idx].name().to_string())
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

        pub fn write_quadratic(&mut self, env: &ArcEnv, quadratic: &Quadratic) -> &mut Self {
            for (i, j, bias) in quadratic.iter_flat() {
                if bias != Bias::default() {
                    let v_i = env.read_arc()[i].name().to_string();
                    let v_j = env.read_arc()[j].name().to_string();
                    if !self.is_first {
                        self.writer.space();
                    }
                    let s = format!("{}{v_i} * {v_j}", self.show_bias(bias));
                    self.writer.write(&s);
                    self.is_first = false;
                }
            }
            self
        }

        pub fn write_linear(&mut self, env: &ArcEnv, linear: &Linear) -> &mut Self {
            for (i, bias) in linear.iter() {
                if bias != Bias::default() {
                    if !self.is_first {
                        self.writer.space();
                    }
                    let s = format!("{}{}", self.show_bias(bias), env.read_arc()[i].name());
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

        fn show_bias(&mut self, bias: Bias) -> String {
            if bias == 1.0 {
                String::from(if self.is_first { "" } else { "+ " })
            } else if bias == -1.0 {
                String::from(if self.is_first { "-" } else { "- " })
            } else if bias < Bias::default() {
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

        pub fn write_constraints(&mut self, constraints: &ConstraintCollection) -> &mut Self {
            for (i, (name, constr)) in constraints.iter().enumerate() {
                if i > 0 {
                    self.writer.new_line();
                }
                self.writer.write(&format!("{}: ", name)).increase_indent();
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

        pub fn write_bounds(&mut self, environment: &ArcEnv) -> &mut Self {
            let vars = environment.vars();
            let ints_and_reals: Vec<_> = vars
                .into_iter()
                .filter(|v| {
                    v.vtype().unwrap() == Vtype::Integer || v.vtype().unwrap() == Vtype::Real
                })
                .collect();
            if ints_and_reals.is_empty() {
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
                let varbounds = var.bounds().unwrap();
                if varbounds.lower.is_bounded() || varbounds.upper.is_bounded() {
                    if let Bound::Bounded(l) = varbounds.lower {
                        self.writer.write(&l.to_string()).with_spaces("<=");
                    }
                    self.writer.write(&var.name().unwrap());
                    if let Bound::Bounded(u) = varbounds.upper {
                        self.writer.with_spaces("<=").write(&u.to_string());
                    }
                } else {
                    self.writer
                        .write(&var.name().unwrap())
                        .space()
                        .write("unbounded");
                }
            }
            self.writer.decrease_indent();
            self
        }

        pub fn write_variables(&mut self, env: &ArcEnv) -> &mut Self {
            let mut var_map = HashMap::new();
            for var in env.vars() {
                var_map
                    .entry(var.vtype().unwrap().to_string())
                    .or_insert(vec![])
                    .push(var.name().unwrap().to_string());
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
}
