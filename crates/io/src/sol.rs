use lunamodel_core::Solution;

use crate::{CustomFormat, FormatOpt};

pub use super::options::{PrintLayout, PySolFormatOpts, ShowMetadata};

impl CustomFormat<FormatOpt> for Solution {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        match format_type {
            FormatOpt::Rs => write!(
                fmt,
                "{}",
                pysolio::solpystr(self, &PySolFormatOpts::default())
            ),
            #[cfg(feature = "py")]
            FormatOpt::Py => write!(
                fmt,
                "{}",
                pysolio::solpystr(self, &PySolFormatOpts::default())
            ),
            #[cfg(feature = "py")]
            FormatOpt::PySol(opts) => write!(fmt, "{}", pysolio::solpystr(self, opts)),
        }
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, format_type: &FormatOpt) -> std::fmt::Result {
        match format_type {
            FormatOpt::Rs => write!(fmt, "{:?}", self),
            #[cfg(feature = "py")]
            FormatOpt::Py | FormatOpt::PySol(_) => {
                let samples = format!(
                    "[{}]",
                    self.samples()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                let objvalues = match &self.obj_values {
                    Some(vs) => format!(
                        "[{}]",
                        vs.iter()
                            .map(|&v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    None => "None".to_string(),
                };
                let rawengs = match &self.raw_energies {
                    Some(vs) => format!(
                        "[{}]",
                        vs.iter()
                            .map(|&v| v.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    None => "None".to_string(),
                };
                let feasible = match &self.feasible {
                    Some(vs) => format!(
                        "[{}]",
                        vs.iter()
                            .map(|&b| pysolio::fmtbool(b))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                    None => "None".to_string(),
                };
                let counts = format!("{:?}", self.counts);

                let constraints = match self.constraints.is_empty() {
                    true => "[]".to_string(),
                    false => format!(
                        "[{}]",
                        (0..self.len())
                            .map(|i| {
                                format!(
                                    "{{{}}}",
                                    self.constraints
                                        .iter()
                                        .map(|(cname, vs)| format!(
                                            "{}: {}",
                                            cname,
                                            pysolio::fmtbool(vs[i])
                                        ))
                                        .collect::<Vec<String>>()
                                        .join(", ")
                                )
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                };

                let vbounds = match self.variable_bounds.is_empty() {
                    true => "[]".to_string(),
                    false => format!(
                        "[{}]",
                        (0..self.len())
                            .map(|i| {
                                format!(
                                    "[{}]",
                                    self.variable_names()
                                        .iter()
                                        .map(|vname| pysolio::fmtbool(
                                            self.variable_bounds[vname][i]
                                        )
                                        .to_string())
                                        .collect::<Vec<String>>()
                                        .join(", ")
                                )
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                };

                let runtime = match &self.timing {
                    Some(t) => t
                        .total()
                        .map_or_else(|e| e.to_string(), |d| d.as_secs_f64().to_string())
                        .to_string(),
                    None => "None".to_string(),
                };

                let nsamples = self.n_samples().to_string();
                let varnames = format!("[{}]", self.variable_names().join(", "));
                let sense = self.sense.to_string();

                write!(
                    fmt,
                    "Solution(samples={samples}, obj_values={objvalues}, raw_energies={rawengs}, counts={counts}, constraints={constraints}, variable_bounds={vbounds}, feasible={feasible}, runtime={runtime}, n_samples={nsamples}, variable_names={varnames}, sense={sense})"
                )
            }
        }
    }
}
mod pysolio {
    use std::{cmp::Ordering, time::Duration};

    use lunamodel_core::{
        Solution,
        solution::{Assignment, Column},
    };
    use lunamodel_types::{
        Bias, BinaryAssignment, IntegerAssignment, RealAssignment, Sense, SpinAssignment,
    };

    use crate::sol::{PrintLayout, PySolFormatOpts, ShowMetadata};

    const SPACE_BETWEEN_COLS: usize = 1;

    pub fn solpystr(sol: &Solution, opts: &PySolFormatOpts) -> String {
        match opts.layout {
            PrintLayout::Row => print_row_layout(sol, opts),
            PrintLayout::Col => print_col_layout(sol, opts),
        }
    }

    fn print_row_layout(sol: &Solution, opts: &PySolFormatOpts) -> String {
        let n_rows = opts.max_lines.min(sol.samples.len());
        let mut collected = vec![Vec::new(); n_rows];
        let mut col_widths = vec![0];

        let idxs = get_sample_indices_sorted(sol);
        let variable_names = sol.variable_names();

        for (i, mut vname) in variable_names[..n_rows].iter().cloned().enumerate() {
            vname.truncate(opts.max_var_name_len);
            col_widths[0] = col_widths[0].max(vname.chars().count());
            collected[i].push(vname);
        }

        let mut n_cols = sol.len() + 1;
        for (i, (_, sample_col)) in sol.samples.iter().enumerate() {
            if i == opts.max_lines {
                break;
            }
            let mut width_reached = 0;
            for (j, &v) in sorted_by_idxs(&sample_col.as_assignments(), &idxs)
                .iter()
                .enumerate()
            {
                let s = match v {
                    Assignment::Binary(b) => {
                        format_binary(b, *col_widths.get(j).unwrap_or(&1).max(&opts.max_col_len))
                    }
                    Assignment::Spin(spin) => format_spin(
                        spin,
                        *col_widths.get(j).unwrap_or(&2).max(&opts.max_col_len),
                    ),
                    Assignment::Integer(int) => {
                        format_int(int, *col_widths.get(j).unwrap_or(&2).max(&opts.max_col_len))
                    }
                    Assignment::Real(real) => format_real(
                        real,
                        *col_widths.get(j).unwrap_or(&4).max(&opts.max_col_len),
                    ),
                };
                let s_len = s.chars().count();
                width_reached += s_len + SPACE_BETWEEN_COLS;
                if j > n_cols || width_reached > opts.max_line_len {
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
        if matches!(
            opts.show_metadata,
            ShowMetadata::Before | ShowMetadata::After
        ) {
            let meta_names = vec![
                String::from("feasible"),
                String::from("raw energy"),
                String::from("objective value"),
                String::from("count"),
            ];
            for mut s in meta_names {
                s.truncate(opts.max_var_name_len);
                let s = String::from(s.trim());
                col_widths[0] = col_widths[0].max(s.chars().count());
                metadata.push(vec![s]);
            }
            for (j, feasible) in maybe_sorted_by_idxs(&sol.feasible, &idxs)
                .iter()
                .enumerate()
            {
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
            for (j, raw) in maybe_sorted_by_idxs(&sol.raw_energies, &idxs)
                .iter()
                .enumerate()
            {
                let s = match raw {
                    None => String::from("?"),
                    Some(bias) => format_bias(*bias, opts.max_col_len),
                };
                let s_len = s.chars().count();
                width_reached += s_len + SPACE_BETWEEN_COLS;
                if j > n_cols || width_reached > opts.max_line_len {
                    n_cols = n_cols.min(j);
                    break;
                }
                if col_widths.len() <= j + 1 {
                    col_widths.push(0);
                }
                col_widths[j + 1] = col_widths[j + 1].max(s_len);
                metadata[1].push(s);
            }
            width_reached = 0;
            for (j, obj) in maybe_sorted_by_idxs(&sol.obj_values, &idxs)
                .iter()
                .enumerate()
            {
                let s = match obj {
                    None => String::from("?"),
                    Some(bias) => format_bias(*bias, opts.max_col_len),
                };
                let s_len = s.chars().count();
                width_reached += s_len + SPACE_BETWEEN_COLS;
                if j > n_cols || width_reached > opts.max_line_len {
                    n_cols = n_cols.min(j);
                    break;
                }
                if col_widths.len() <= j + 1 {
                    col_widths.push(0);
                }
                col_widths[j + 1] = col_widths[j + 1].max(s_len);
                metadata[2].push(s);
            }
            width_reached = 0;
            for (j, &count) in sorted_by_idxs(&sol.counts, &idxs).iter().enumerate() {
                let s = format_usize(count, opts.max_col_len);
                let s_len = s.chars().count();
                width_reached += s_len + SPACE_BETWEEN_COLS;
                if j > n_cols || width_reached > opts.max_line_len {
                    n_cols = n_cols.min(j);
                    break;
                }
                if col_widths.len() <= j + 1 {
                    col_widths.push(0);
                }
                col_widths[j + 1] = col_widths[j + 1].max(s_len);
                metadata[3].push(s);
            }
        }

        let mut total_width = col_widths[..n_cols].iter().sum::<usize>() + n_cols - 1;
        while n_cols <= sol.len() + 1 && total_width > opts.max_line_len - 4 {
            n_cols -= 1;
            total_width = col_widths[..n_cols].iter().sum::<usize>() + n_cols - 1;
        }
        if n_cols <= sol.len() {
            total_width += 4;
        }

        let mut out = String::new();
        if let ShowMetadata::Before = opts.show_metadata {
            for row in metadata.iter() {
                for (j, (&width, col)) in col_widths.iter().zip(row).enumerate() {
                    if j >= n_cols {
                        if n_cols <= sol.len() + 1 {
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

            out.push_str(&"─".repeat(total_width));
            out.push('\n');
        }

        for (i, row) in collected.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            for (j, (&width, col)) in col_widths.iter().zip(row).enumerate() {
                if j >= n_cols {
                    if n_cols <= sol.len() + 1 {
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
        if sol.samples.len() > opts.max_lines {
            out.push_str("\n...");
        }

        if let ShowMetadata::After = opts.show_metadata {
            out.push_str(&format!("\n{}\n", "─".repeat(total_width)));
            for (i, row) in metadata.iter().enumerate() {
                for (j, (&width, col)) in col_widths.iter().zip(row).enumerate() {
                    if j >= n_cols {
                        if n_cols <= sol.len() + 1 {
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
        out.push_str(&format_other_metadata(sol));
        out
    }

    fn print_col_layout(sol: &Solution, opts: &PySolFormatOpts) -> String {
        let mut n_cols = 0;
        let mut col_widths = Vec::new();
        let mut meta_widths = Vec::with_capacity(3);
        let mut width_reached = 0;

        let n_rows = opts.max_lines.min(sol.len());
        let mut collected = vec![Vec::new(); n_rows];
        let mut metadata = vec![Vec::new(); n_rows];
        let meta_names = vec![
            String::from("feas"),
            String::from("raw"),
            String::from("obj"),
            String::from("count"),
        ];
        let mut var_names = Vec::new();
        let idxs = get_sample_indices_sorted(sol);

        let variable_names = sol.variable_names();

        if matches!(
            opts.show_metadata,
            ShowMetadata::Before | ShowMetadata::After
        ) {
            let feas_width = 4;
            meta_widths.push(feas_width);
            width_reached += feas_width + SPACE_BETWEEN_COLS;
            for (row_idx, feasible) in maybe_sorted_by_idxs(&sol.feasible, &idxs)[..n_rows]
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
            for raw in maybe_sorted_by_idxs(&sol.raw_energies, &idxs)[..n_rows].iter() {
                let s = match raw {
                    None => String::from("?"),
                    Some(bias) => format_bias(*bias, opts.max_col_len),
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
            for obj in maybe_sorted_by_idxs(&sol.obj_values, &idxs)[..n_rows].iter() {
                let s = match obj {
                    None => String::from("?"),
                    Some(bias) => format_bias(*bias, opts.max_col_len),
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
            for &count in sorted_by_idxs(&sol.counts, &idxs)[..n_rows].iter() {
                let s = format_usize(count, opts.max_col_len);
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

        for (vname, col) in &sol.samples {
            let vname_len = vname.chars().count().min(opts.max_col_len);
            let mut col_width = match col {
                Column::Binary(_) => vname_len,
                Column::Spin(_) => vname_len.max(2),
                Column::Integer(_) => vname_len.max(2),
                Column::Real(_) => vname_len.max(4),
            };
            let mut vals = Vec::with_capacity(n_rows);
            match col {
                Column::Binary(bins) => {
                    for &v in sorted_by_idxs(&bins.data(), &idxs)[..n_rows].iter() {
                        let s = format_binary(v, col_width);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
                Column::Spin(spins) => {
                    for &v in sorted_by_idxs(&spins.data(), &idxs)[..n_rows].iter() {
                        let s = format_spin(v, col_width);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
                Column::Integer(ints) => {
                    for &v in sorted_by_idxs(&ints.data(), &idxs)[..n_rows].iter() {
                        let s = format_int(v, opts.max_col_len);
                        col_width = col_width.max(s.chars().count());
                        vals.push(s);
                    }
                }
                Column::Real(reals) => {
                    for &v in sorted_by_idxs(&reals.data(), &idxs)[..n_rows].iter() {
                        let s = format_real(v, opts.max_col_len);
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
            if width_reached <= opts.max_line_len {
                col_widths.push(col_width as isize);
            } else {
                let too_long = width_old + 4 > opts.max_line_len;
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

        if let ShowMetadata::Before = opts.show_metadata {
            for (width, vname) in meta_widths.iter().zip(&meta_names) {
                var_names.push(format!("{vname:>width$}"));
            }
            var_names.push(String::from("│"));
        }
        for (mut vname, &col_width) in variable_names[..n_cols].iter().cloned().zip(&col_widths) {
            if col_width < 0 {
                var_names.push(String::from("   "));
            } else {
                let cw = col_width as usize;
                vname.truncate(cw);
                var_names.push(format!("{vname:>cw$}"));
            }
        }
        if let ShowMetadata::After = opts.show_metadata {
            var_names.push(String::from("│"));
            for (width, vname) in meta_widths.iter().zip(meta_names) {
                var_names.push(format!("{vname:>width$}"));
            }
        }

        let mut out = var_names.join(" ");
        for (meta, row) in metadata.iter().zip(collected) {
            out.push('\n');
            if let ShowMetadata::Before = opts.show_metadata {
                let meta_c = meta.clone();
                out.push_str(&meta_c.join(" "));
                out.push_str(" │ ");
            }
            out.push_str(&row.join(" "));
            if let ShowMetadata::After = opts.show_metadata {
                out.push_str(" │ ");
                let meta_c = meta.clone();
                out.push_str(&meta_c.join(" "));
            }
        }
        if n_rows < sol.len() {
            out.push_str("\n...");
        }
        out.push_str(&format_other_metadata(sol));
        out
    }

    fn get_sample_indices_sorted(sol: &Solution) -> Vec<usize> {
        let sense = if sol.sense == Sense::Min { -1.0 } else { 1.0 };
        let mut idxs = (0..sol.len()).collect::<Vec<_>>();
        idxs.sort_by(|&idx1, &idx2| 'res: {
            let feas = sol
                .feasible
                .as_ref()
                .map(|f| f[idx2])
                .cmp(&sol.feasible.as_ref().map(|f| f[idx1]));
            if feas != Ordering::Equal {
                break 'res feas;
            }
            let obj = cmp_bias(
                sol.obj_values.as_ref().map(|o| o[idx2]).map(|b| b * sense),
                sol.obj_values.as_ref().map(|o| o[idx1]).map(|b| b * sense),
            );
            if obj != Ordering::Equal {
                break 'res obj;
            }
            let obj = cmp_bias(
                sol.raw_energies
                    .as_ref()
                    .map(|e| e[idx2])
                    .map(|b| b * sense),
                sol.raw_energies
                    .as_ref()
                    .map(|e| e[idx1])
                    .map(|b| b * sense),
            );
            if obj != Ordering::Equal {
                break 'res obj;
            }
            sol.counts[idx2].cmp(&sol.counts[idx1])
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
    fn sorted_by_idxs<T>(values: &[T], idxs: &[usize]) -> Vec<T>
    where
        T: Copy,
    {
        idxs.iter().map(|&i| values[i]).collect()
    }

    fn maybe_sorted_by_idxs<T>(values: &Option<Vec<T>>, idxs: &[usize]) -> Vec<Option<T>>
    where
        T: Copy,
    {
        match &values {
            Some(vals) => idxs.iter().map(|&i| Some(vals[i])).collect(),
            None => idxs.iter().map(|_| None).collect(),
        }
    }

    fn format_other_metadata(sol: &Solution) -> String {
        let mut out = String::new();
        out.push_str(&format!("\n\nTotal samples: {}", sol.n_samples()));
        out.push_str(&format!("\nUnique samples: {}", sol.len()));
        out.push_str(&format!("\nTotal variables: {}", sol.samples.len()));
        if let Some(t) = sol.timing {
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

    fn format_binary(value: BinaryAssignment, col_width: usize) -> String {
        if value == BinaryAssignment::default() {
            format!("{:>col_width$}", 0)
        } else {
            format!("{:>col_width$}", 1)
        }
    }

    fn format_spin(value: SpinAssignment, col_width: usize) -> String {
        if value == 1 {
            format!("{:>col_width$}", 1)
        } else {
            format!("{:>col_width$}", -1)
        }
    }

    fn format_int(value: IntegerAssignment, col_width: usize) -> String {
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

    fn format_real(value: RealAssignment, col_width: usize) -> String {
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

    fn remove_trailing_zeros(mut s: String) -> String {
        if s.contains('.') && !s.contains('e') {
            while s.ends_with('0') && !s.ends_with(".0") {
                s.pop();
            }
        }
        s
    }

    #[cfg(feature = "py")]
    pub fn fmtbool(b: bool) -> String {
        match b {
            true => "True".to_string(),
            false => "False".to_string(),
        }
    }
}
