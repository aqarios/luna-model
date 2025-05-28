use crate::core::expression::{BiasConstraints, One};
use crate::core::solution::sol::{SampleCol, ShowMetadata};
use crate::core::solution::AssignmentBaseTypes;
use crate::core::{PrintLayout, Solution, VarAssignment};
use std::time::Duration;

const SPACE_BETWEEN_COLS: usize = 1;

impl<Bias, AssignmentTypes> Solution<Bias, AssignmentTypes>
where
    Bias: BiasConstraints,
    AssignmentTypes: AssignmentBaseTypes,
{
    pub fn print(
        &self,
        max_line_length: usize,
        max_col_size: usize,
        max_lines: usize,
        max_var_name_length: usize,
        layout: PrintLayout,
        show_metadata: ShowMetadata,
    ) -> String {
        match layout {
            PrintLayout::Row => self.print_row_layout(
                max_line_length,
                max_col_size,
                max_lines,
                max_var_name_length,
                show_metadata,
            ),
            PrintLayout::Col => {
                self.print_col_layout(max_line_length, max_col_size, max_lines, show_metadata)
            }
        }
    }

    fn print_col_layout(
        &self,
        max_line_length: usize,
        max_col_size: usize,
        max_lines: usize,
        show_metadata: ShowMetadata,
    ) -> String {
        let mut n_cols = 0;
        let mut col_widths = Vec::new();
        let mut meta_widths = Vec::with_capacity(3);
        let mut width_reached = 0;

        let n_rows = max_lines.min(self.n_samples);
        let mut collected = vec![Vec::new(); n_rows];
        let mut metadata = vec![Vec::new(); n_rows];
        let meta_names = vec![
            String::from("feas"),
            String::from("raw"),
            String::from("obj"),
            String::from("count"),
        ];
        let mut var_names = Vec::new();

        if matches!(show_metadata, ShowMetadata::Left | ShowMetadata::Right) {
            let feas_width = 4;
            meta_widths.push(feas_width);
            width_reached += feas_width + SPACE_BETWEEN_COLS;
            for (row_idx, feasible) in self.feasible[..n_rows].iter().enumerate() {
                let s = match feasible {
                    None => "   ?",
                    Some(true) => "   t",
                    Some(false) => "   f",
                };
                metadata[row_idx].push(String::from(s))
            }

            let mut raws = Vec::new();
            let mut col_width = 3;
            for raw in self.raw_energies[..n_rows].iter() {
                let s = match raw {
                    None => String::from("?"),
                    Some(bias) => Self::format_bias(*bias, max_col_size),
                };
                col_width = col_width.max(s.chars().count());
                raws.push(s);
            }
            meta_widths.push(col_width);
            width_reached += col_width + SPACE_BETWEEN_COLS;
            for (row_idx, s) in raws.iter().enumerate() {
                metadata[row_idx].push(format!("{s:>col_width$}"))
            }

            let mut objs = Vec::new();
            col_width = 3;
            for obj in self.obj_values[..n_rows].iter() {
                let s = match obj {
                    None => String::from("?"),
                    Some(bias) => Self::format_bias(*bias, max_col_size),
                };
                col_width = col_width.max(s.chars().count());
                objs.push(s);
            }
            meta_widths.push(col_width);
            width_reached += col_width + SPACE_BETWEEN_COLS;
            for (row_idx, s) in objs.iter().enumerate() {
                metadata[row_idx].push(format!("{s:>col_width$}"))
            }

            let mut counts = Vec::new();
            col_width = 5;
            for &count in self.counts[..n_rows].iter() {
                let s = Self::format_usize(count, max_col_size);
                col_width = col_width.max(s.chars().count());
                counts.push(s);
            }
            meta_widths.push(col_width);
            width_reached += col_width + SPACE_BETWEEN_COLS;
            for (row_idx, s) in counts.iter().enumerate() {
                metadata[row_idx].push(format!("{s:>col_width$}"))
            }

            width_reached += 2; // for extra spacing
        }

        for (col, vname) in self.samples.iter().zip(&self.variable_names) {
            let vname_len = vname.chars().count().min(max_col_size);
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
                        let s = Self::format_int(v, max_col_size);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
                SampleCol::Real(reals) => {
                    for &v in reals[..n_rows].iter() {
                        let s = Self::format_real(v, max_col_size);
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
            n_cols += 1;
            if width_reached <= max_line_length {
                col_widths.push(col_width as isize);
            } else {
                let too_long = width_old + 3 > max_line_length;
                collected.iter_mut().for_each(|cols| {
                    if too_long {
                        cols.pop();
                    }
                    cols.push(String::from("..."));
                });
                if !too_long {
                    col_widths.push(col_width as isize);
                    n_cols += 1;
                }
                col_widths.push(-1); // magic value for '...' column
                break;
            }
        }

        if let ShowMetadata::Left = show_metadata {
            for (width, vname) in meta_widths.iter().zip(&meta_names) {
                var_names.push(format!("{vname:>width$}"));
            }
            var_names.push(String::from("│"));
        }
        for (mut vname, col_width) in self.variable_names[..n_cols]
            .iter()
            .cloned()
            .zip(col_widths)
        {
            if col_width < 0 {
                var_names.push(String::from("   "));
            } else {
                let cw = col_width as usize;
                vname.truncate(cw);
                var_names.push(format!("{vname:>cw$}"));
            }
        }
        if let ShowMetadata::Right = show_metadata {
            var_names.push(String::from("│"));
            for (width, vname) in meta_widths.iter().zip(meta_names) {
                var_names.push(format!("{vname:>width$}"));
            }
        }

        let mut out = var_names.join(" ");
        for (meta, row) in metadata.iter().zip(collected) {
            out.push('\n');
            if let ShowMetadata::Left = show_metadata {
                let meta_c = meta.clone();
                out.push_str(&meta_c.join(" "));
                out.push_str(" │ ");
            }
            out.push_str(&row.join(" "));
            if let ShowMetadata::Right = show_metadata {
                out.push_str(" │ ");
                let meta_c = meta.clone();
                out.push_str(&meta_c.join(" "));
            }
        }
        if n_rows < self.n_samples {
            out.push_str("\n...");
        }

        out.push_str(&format!("\n\nTotal rows: {}", self.n_samples));
        out.push_str(&format!("\nTotal columns: {}", self.samples.len()));

        if let Some(t) = self.timing {
            out.push_str("\n\nTiming:");
            out.push_str(&format!(
                "\nTotal: {}s",
                t.total().unwrap_or(Duration::ZERO).as_secs_f64()
            ));
            if let Some(qpu) = t.qpu {
                out.push_str(&format!("\nQPU: {qpu}s"))
            }
        }

        out
    }

    fn print_row_layout(
        &self,
        max_line_length: usize,
        max_col_size: usize,
        max_lines: usize,
        max_var_name_length: usize,
        show_metadata: ShowMetadata,
    ) -> String {
        let n_rows = max_lines.min(self.samples.len());
        let mut collected = vec![Vec::new(); n_rows];
        let mut col_widths = vec![0];

        for (i, mut vname) in self.variable_names[..n_rows].iter().cloned().enumerate() {
            vname.truncate(max_var_name_length);
            col_widths[0] = col_widths[0].max(vname.chars().count());
            collected[i].push(vname);
        }

        let mut n_cols = usize::MAX;
        for (i, sample_col) in self.samples[..max_lines].iter().enumerate() {
            let mut width_reached = 0;
            for (j, &v) in sample_col.as_vec().iter().enumerate() {
                let (s, w) = match v {
                    VarAssignment::Binary(bin) => {
                        let w = *col_widths.get(j).unwrap_or(&0).max(&max_col_size);
                        (Self::format_binary(bin, w), w)
                    }
                    VarAssignment::Spin(spin) => {
                        let w = *col_widths.get(j).unwrap_or(&2).max(&max_col_size);
                        (Self::format_spin(spin, w), w)
                    }
                    VarAssignment::Integer(int) => {
                        let w = *col_widths.get(j).unwrap_or(&2).max(&max_col_size);
                        (Self::format_int(int, w), w)
                    }
                    VarAssignment::Real(real) => {
                        let w = *col_widths.get(j).unwrap_or(&4).max(&max_col_size);
                        (Self::format_real(real, w), w)
                    }
                };
                let s_len = s.chars().count();
                width_reached += s_len + SPACE_BETWEEN_COLS;
                if j > n_cols || width_reached > max_line_length {
                    n_cols = collected.iter().min_by_key(|x| x.len());
                    break;
                }
                col_widths[j + 1] = w.max(s_len);
                collected[i].push(s);
            }
        }

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

    fn format_usize(value: usize, col_width: usize) -> String {
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

    fn format_bias(value: Bias, col_width: usize) -> String {
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
