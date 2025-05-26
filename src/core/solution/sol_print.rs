use crate::core::expression::{BiasConstraints, One};
use crate::core::solution::sol::SampleCol;
use crate::core::solution::AssignmentBaseTypes;
use crate::core::{PrintLayout, Solution};

impl<Bias, AssignmentTypes> Solution<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn print(
        &self,
        max_line_length: usize,
        max_chars_per_var: usize,
        max_lines: usize,
        layout: PrintLayout,
    ) -> String {
        match layout {
            PrintLayout::Row => {
                self.print_row_layout(max_line_length, max_chars_per_var, max_lines)
            }
            PrintLayout::Col => {
                self.print_col_layout(max_line_length, max_chars_per_var, max_lines)
            }
        }
    }

    fn print_col_layout(
        &self,
        max_line_length: usize,
        max_chars_per_var: usize,
        max_lines: usize,
    ) -> String {
        println!("{max_line_length}, {max_chars_per_var}, {max_lines}");
        const SPACE_BETWEEN_COLS: usize = 1;
        let mut n_cols = 0;
        let mut col_widths = Vec::new();
        let mut width_reached = 0;

        let n_rows = max_lines.min(self.n_samples);
        let mut collected = vec![Vec::with_capacity(n_cols); n_rows];

        for (col, vname) in self.samples.iter().zip(&self.variable_names) {
            let vname_len = vname.chars().count().min(max_chars_per_var);
            let mut col_width = match col {
                SampleCol::Binary(_) => vname_len,
                SampleCol::Spin(_) => vname_len.max(2),
                SampleCol::Integer(_) => vname_len.max(2),
                SampleCol::Real(_) => vname_len.max(4),
            };
            let mut vals = Vec::with_capacity(n_rows);
            match col {
                SampleCol::Binary(bins) => {
                    for &v in bins[..n_rows].iter() {
                        let s = Self::format_binary(v, col_width);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
                SampleCol::Spin(spins) => {
                    for &v in spins[..n_rows].iter() {
                        let s = Self::format_spin(v, col_width);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
                SampleCol::Integer(ints) => {
                    for &v in ints[..n_rows].iter() {
                        let s = Self::format_int(v, max_chars_per_var);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
                SampleCol::Real(reals) => {
                    for &v in reals[..n_rows].iter() {
                        let s = Self::format_real(v, max_chars_per_var);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
            }
            for (row_idx, s) in vals.iter().enumerate() {
                collected[row_idx].push(format!("{s:>col_width$}"))
            }

            let width_old = width_reached;
            width_reached += col_width + SPACE_BETWEEN_COLS;
            if width_reached <= max_line_length {
                n_cols += 1;
                col_widths.push(col_width);
            } else {
                collected.iter_mut().for_each(|cols| {
                    if width_old + 3 > max_line_length {
                        cols.pop();
                    }
                    cols.push(String::from("..."))
                });
                break;
            }
        }

        let mut var_names = Vec::with_capacity(n_cols);
        for (mut vname, col_width) in self.variable_names[..n_cols]
            .iter()
            .cloned()
            .zip(col_widths)
        {
            vname.truncate(col_width);
            var_names.push(format!("{vname:>col_width$}"));
        }

        let mut out = var_names.join(" ");
        for row in collected {
            out.push('\n');
            out.push_str(&row.join(" "));
        }
        out
    }

    fn print_row_layout(
        &self,
        max_line_length: usize,
        max_chars_per_var: usize,
        max_lines: usize,
    ) -> String {
        todo!()
    }

    fn format_binary(value: AssignmentTypes::BinaryType, col_width: usize) -> String {
        if value == AssignmentTypes::BinaryType::default() {
            format!("{:>col_width$}", 0)
        } else {
            format!("{:>col_width$}", 1)
        }
    }

    fn format_spin(value: AssignmentTypes::SpinType, col_width: usize) -> String {
        if value == AssignmentTypes::SpinType::one() {
            format!("{:>col_width$}", 1)
        } else {
            format!("{:>col_width$}", -1)
        }
    }

    fn format_int(value: AssignmentTypes::IntegerType, col_width: usize) -> String {
        if value.to_string().chars().count() <= col_width {
            format!("{value}")
        } else {
            format!("{value:>col_width$e}")
        }
    }

    fn format_real(value: AssignmentTypes::RealType, col_width: usize) -> String {
        let digits_int_part = format!("{:.0}", value).chars().count();
        if digits_int_part <= col_width - 2 {
            let decimals = col_width - digits_int_part - 1;
            format!("{value:>col_width$.decimals$}")
        } else {
            let decimals = col_width - 4;
            format!("{value:>col_width$.decimals$e}")
        }
    }
}
