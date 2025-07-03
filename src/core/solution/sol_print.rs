use crate::core::expression::One;
use crate::core::solution::sol::{SampleCol, ShowMetadata};
use crate::core::{PrintLayout, Sense, Solution, VarAssignment};
use crate::types::{
    Bias, BinaryAssignmentType, IntegerAssignmentType, RealAssignmentType, SpinAssignmentType,
};
use std::cmp::Ordering;
use std::time::Duration;

const SPACE_BETWEEN_COLS: usize = 1;

impl Solution {
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
        let idxs = self.get_sample_indices_sorted();

        if matches!(show_metadata, ShowMetadata::Before | ShowMetadata::After) {
            let feas_width = 4;
            meta_widths.push(feas_width);
            width_reached += feas_width + SPACE_BETWEEN_COLS;
            for (row_idx, feasible) in sorted_by_idxs(&self.feasible, &idxs)[..n_rows]
                .iter()
                .enumerate()
            {
                let s = match feasible {
                    None => "   ?",
                    Some(true) => "   t",
                    Some(false) => "   f",
                };
                metadata[row_idx].push(String::from(s))
            }

            let mut raws = Vec::new();
            let mut col_width = 3;
            for raw in sorted_by_idxs(&self.raw_energies, &idxs)[..n_rows].iter() {
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
            for obj in sorted_by_idxs(&self.obj_values, &idxs)[..n_rows].iter() {
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
            for &count in sorted_by_idxs(&self.counts, &idxs)[..n_rows].iter() {
                let s = Self::format_usize(count, max_col_size);
                col_width = col_width.max(s.chars().count());
                counts.push(s);
            }
            meta_widths.push(col_width);
            width_reached += col_width + SPACE_BETWEEN_COLS;
            for (row_idx, s) in counts.iter().enumerate() {
                metadata[row_idx].push(format!("{s:>col_width$}"))
            }

            width_reached += SPACE_BETWEEN_COLS;
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
                    for &v in sorted_by_idxs(&bins.data, &idxs)[..n_rows].iter() {
                        let s = Self::format_binary(v, col_width);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
                SampleCol::Spin(spins) => {
                    for &v in sorted_by_idxs(&spins.data, &idxs)[..n_rows].iter() {
                        let s = Self::format_spin(v, col_width);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
                SampleCol::Integer(ints) => {
                    for &v in sorted_by_idxs(&ints.data, &idxs)[..n_rows].iter() {
                        let s = Self::format_int(v, max_col_size);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
                SampleCol::Real(reals) => {
                    for &v in sorted_by_idxs(&reals.data, &idxs)[..n_rows].iter() {
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
                let too_long = width_old + 4 > max_line_length;
                collected.iter_mut().for_each(|cols| {
                    if too_long {
                        cols.pop();
                    }
                    cols.pop();
                    cols.push(String::from("..."));
                });
                if too_long {
                    col_widths.pop();
                    n_cols -= 1;
                }
                col_widths.push(-1); // magic value for '...' column
                break;
            }
        }

        if let ShowMetadata::Before = show_metadata {
            for (width, vname) in meta_widths.iter().zip(&meta_names) {
                var_names.push(format!("{vname:>width$}"));
            }
            var_names.push(String::from("│"));
        }
        for (mut vname, &col_width) in self.variable_names[..n_cols]
            .iter()
            .cloned()
            .zip(&col_widths)
        {
            if col_width < 0 {
                var_names.push(String::from("   "));
            } else {
                let cw = col_width as usize;
                vname.truncate(cw);
                var_names.push(format!("{vname:>cw$}"));
            }
        }
        if let ShowMetadata::After = show_metadata {
            var_names.push(String::from("│"));
            for (width, vname) in meta_widths.iter().zip(meta_names) {
                var_names.push(format!("{vname:>width$}"));
            }
        }

        let mut out = var_names.join(" ");
        for (meta, row) in metadata.iter().zip(collected) {
            out.push('\n');
            if let ShowMetadata::Before = show_metadata {
                let meta_c = meta.clone();
                out.push_str(&meta_c.join(" "));
                out.push_str(" │ ");
            }
            out.push_str(&row.join(" "));
            if let ShowMetadata::After = show_metadata {
                out.push_str(" │ ");
                let meta_c = meta.clone();
                out.push_str(&meta_c.join(" "));
            }
        }
        if n_rows < self.n_samples {
            out.push_str("\n...");
        }
        out.push_str(&self.format_other_metadata());
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

        let idxs = self.get_sample_indices_sorted();

        for (i, mut vname) in self.variable_names[..n_rows].iter().cloned().enumerate() {
            vname.truncate(max_var_name_length);
            col_widths[0] = col_widths[0].max(vname.chars().count());
            collected[i].push(vname);
        }

        let mut n_cols = self.n_samples + 1;
        for (i, sample_col) in self.samples.iter().enumerate() {
            if i == max_lines {
                break;
            }
            let mut width_reached = 0;
            for (j, &v) in sorted_by_idxs(&sample_col.as_vec(), &idxs)
                .iter()
                .enumerate()
            {
                let s = match v {
                    VarAssignment::Binary(b) => {
                        Self::format_binary(b, *col_widths.get(j).unwrap_or(&1).max(&max_col_size))
                    }
                    VarAssignment::Spin(spin) => {
                        Self::format_spin(spin, *col_widths.get(j).unwrap_or(&2).max(&max_col_size))
                    }
                    VarAssignment::Integer(int) => {
                        Self::format_int(int, *col_widths.get(j).unwrap_or(&2).max(&max_col_size))
                    }
                    VarAssignment::Real(real) => {
                        Self::format_real(real, *col_widths.get(j).unwrap_or(&4).max(&max_col_size))
                    }
                };
                let s_len = s.chars().count();
                width_reached += s_len + SPACE_BETWEEN_COLS;
                if j > n_cols || width_reached > max_line_length {
                    n_cols = n_cols.min(j);
                    break;
                }
                if col_widths.len() <= j + 1 {
                    col_widths.push(0);
                }
                col_widths[j + 1] = col_widths[j + 1].max(s_len);
                collected[i].push(s);
            }
        }

        let mut metadata = Vec::new();
        if matches!(show_metadata, ShowMetadata::Before | ShowMetadata::After) {
            let meta_names = vec![
                String::from("feasible"),
                String::from("raw energy"),
                String::from("objective value"),
                String::from("count"),
            ];
            for mut s in meta_names {
                s.truncate(max_var_name_length);
                let s = String::from(s.trim());
                col_widths[0] = col_widths[0].max(s.chars().count());
                metadata.push(vec![s]);
            }
            for (j, feasible) in sorted_by_idxs(&self.feasible, &idxs).iter().enumerate() {
                let s = match feasible {
                    None => "?",
                    Some(true) => "t",
                    Some(false) => "f",
                };
                if j > n_cols {
                    break;
                }
                metadata[0].push(String::from(s));
            }
            let mut width_reached = 0;
            for (j, raw) in sorted_by_idxs(&self.raw_energies, &idxs).iter().enumerate() {
                let s = match raw {
                    None => String::from("?"),
                    Some(bias) => Self::format_bias(*bias, max_col_size),
                };
                let s_len = s.chars().count();
                width_reached += s_len + SPACE_BETWEEN_COLS;
                if j > n_cols || width_reached > max_line_length {
                    n_cols = n_cols.min(j);
                    break;
                }
                if col_widths.len() <= j + 1 {
                    col_widths.push(0);
                }
                col_widths[j + 1] = col_widths[j + 1].max(s_len);
                metadata[1].push(String::from(s));
            }
            width_reached = 0;
            for (j, obj) in sorted_by_idxs(&self.obj_values, &idxs).iter().enumerate() {
                let s = match obj {
                    None => String::from("?"),
                    Some(bias) => Self::format_bias(*bias, max_col_size),
                };
                let s_len = s.chars().count();
                width_reached += s_len + SPACE_BETWEEN_COLS;
                if j > n_cols || width_reached > max_line_length {
                    n_cols = n_cols.min(j);
                    break;
                }
                if col_widths.len() <= j + 1 {
                    col_widths.push(0);
                }
                col_widths[j + 1] = col_widths[j + 1].max(s_len);
                metadata[2].push(String::from(s));
            }
            width_reached = 0;
            for (j, &count) in sorted_by_idxs(&self.counts, &idxs).iter().enumerate() {
                let s = Self::format_usize(count, max_col_size);
                let s_len = s.chars().count();
                width_reached += s_len + SPACE_BETWEEN_COLS;
                if j > n_cols || width_reached > max_line_length {
                    n_cols = n_cols.min(j);
                    break;
                }
                if col_widths.len() <= j + 1 {
                    col_widths.push(0);
                }
                col_widths[j + 1] = col_widths[j + 1].max(s_len);
                metadata[3].push(String::from(s));
            }
        }

        let mut total_width = col_widths[..n_cols].iter().sum::<usize>() + n_cols - 1;
        while n_cols <= self.n_samples + 1 && total_width > max_line_length - 4 {
            n_cols -= 1;
            total_width = col_widths[..n_cols].iter().sum::<usize>() + n_cols - 1;
        }
        if n_cols <= self.n_samples {
            total_width += 4;
        }

        let mut out = String::new();
        if let ShowMetadata::Before = show_metadata {
            for row in metadata.iter() {
                for (j, (&width, col)) in col_widths.iter().zip(row).enumerate() {
                    if j >= n_cols {
                        if n_cols <= self.n_samples + 1 {
                            out.push_str(&String::from(" ..."));
                        }
                        break;
                    }
                    if j > 0 {
                        out.push(' ')
                    }
                    out.push_str(&format!("{col:>width$}"))
                }
                out.push('\n');
            }

            out.push_str(&String::from("─".repeat(total_width)));
            out.push('\n');
        }

        for (i, row) in collected.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            for (j, (&width, col)) in col_widths.iter().zip(row).enumerate() {
                if j >= n_cols {
                    if n_cols <= self.n_samples + 1 {
                        out.push_str(&String::from(" ..."));
                    }
                    break;
                }
                if j > 0 {
                    out.push(' ')
                }
                out.push_str(&format!("{col:>width$}"))
            }
        }
        if self.samples.len() > max_lines {
            out.push_str("\n...");
        }

        if let ShowMetadata::After = show_metadata {
            out.push_str(&format!("\n{}\n", "─".repeat(total_width)));
            for (i, row) in metadata.iter().enumerate() {
                for (j, (&width, col)) in col_widths.iter().zip(row).enumerate() {
                    if j >= n_cols {
                        if n_cols <= self.n_samples + 1 {
                            out.push_str(&String::from(" ..."));
                        }
                        break;
                    }
                    if j > 0 {
                        out.push(' ')
                    }
                    out.push_str(&format!("{col:>width$}"))
                }
                if i < 3 {
                    out.push('\n');
                }
            }
        }
        out.push_str(&self.format_other_metadata());
        out
    }

    fn format_other_metadata(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("\n\nTotal samples: {}", self.n_samples));
        out.push_str(&format!("\nTotal variables: {}", self.samples.len()));
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

    fn format_binary(value: BinaryAssignmentType, col_width: usize) -> String {
        if value == BinaryAssignmentType::default() {
            format!("{:>col_width$}", 0)
        } else {
            format!("{:>col_width$}", 1)
        }
    }

    fn format_spin(value: SpinAssignmentType, col_width: usize) -> String {
        if value == SpinAssignmentType::one() {
            format!("{:>col_width$}", 1)
        } else {
            format!("{:>col_width$}", -1)
        }
    }

    fn format_int(value: IntegerAssignmentType, col_width: usize) -> String {
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

    fn format_real(value: RealAssignmentType, col_width: usize) -> String {
        let digits_int_part = format!("{:.0}", value).chars().count();
        if digits_int_part <= col_width - 2 {
            let decimals = col_width - digits_int_part - 1;
            let s = format!("{value:>col_width$.decimals$}");
            remove_trailing_zeros(s)
        } else {
            let decimals = col_width - 4;
            format!("{value:>col_width$.decimals$e}")
        }
    }

    fn format_bias(value: Bias, col_width: usize) -> String {
        let digits_int_part = format!("{:.0}", value).chars().count();
        if digits_int_part <= col_width - 2 {
            let decimals = col_width - digits_int_part - 1;
            let s = format!("{value:>col_width$.decimals$}");
            remove_trailing_zeros(s)
        } else {
            let decimals = col_width - 4;
            format!("{value:>col_width$.decimals$e}")
        }
    }

    fn get_sample_indices_sorted(&self) -> Vec<usize> {
        let sense = if self.sense == Sense::Min { -1.0 } else { 1.0 };
        let mut idxs = (0..self.n_samples).collect::<Vec<_>>();
        idxs.sort_by(|&idx1, &idx2| 'res: {
            let feas = self.feasible[idx2].cmp(&self.feasible[idx1]);
            if feas != Ordering::Equal {
                break 'res feas;
            }
            let obj = Self::cmp_bias(
                self.obj_values[idx2].map(|b| b * sense),
                self.obj_values[idx1].map(|b| b * sense),
            );
            if obj != Ordering::Equal {
                break 'res obj;
            }
            let obj = Self::cmp_bias(
                self.raw_energies[idx2].map(|b| b * sense),
                self.raw_energies[idx1].map(|b| b * sense),
            );
            if obj != Ordering::Equal {
                break 'res obj;
            }
            self.counts[idx2].cmp(&self.counts[idx1])
        });
        idxs
    }

    fn cmp_bias(bias1: Option<Bias>, bias2: Option<Bias>) -> Ordering {
        match (bias1, bias2) {
            (None, None) => Ordering::Equal,
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
            (Some(b1), Some(b2)) => {
                if b1 < b2 {
                    Ordering::Less
                } else if b1 > b2 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}
fn sorted_by_idxs<T>(values: &Vec<T>, idxs: &Vec<usize>) -> Vec<T>
where
    T: Copy,
{
    idxs.iter().map(|&i| values[i]).collect()
}
fn remove_trailing_zeros(mut s: String) -> String {
    if s.contains(|c| c == '.') && !s.contains(|c| c == 'e') {
        while s.ends_with('0') && !s.ends_with(".0") {
            s.pop();
        }
    }
    s
}
