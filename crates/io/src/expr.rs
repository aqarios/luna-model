use indexmap::IndexMap;
use lunamodel_core::Expression;
use lunamodel_types::Bias;
use regex::Regex;

use crate::{CustomFormat, FormatOpt};

impl CustomFormat<FormatOpt> for Expression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, _: &FormatOpt) -> std::fmt::Result {
        write!(fmt, "{}", expr_string(self))
    }

    fn dbg(&self, fmt: &mut std::fmt::Formatter<'_>, _: &FormatOpt) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}
fn expr_string(expr: &Expression) -> String {
    let mut const_str = String::new();
    let mut lins = Vec::new();
    let mut quads = Vec::new();
    let mut hos = Vec::new();

    for (vars, b) in expr.items() {
        let bstr = match b {
            0.0 => String::new(),
            1.0 => String::new(),
            -1.0 => String::from("-"),
            _ => format!("{} ", b),
        };
        match &vars[..] {
            [] => {
                if b != Bias::default() {
                    const_str.push_str(&format!("+{}", &b.to_string()).replace("+-", "-"))
                }
            }
            [v] => {
                if lins.is_empty() {
                    lins.push(format!("{bstr}{}", v.name().unwrap()));
                } else {
                    lins.push(format!("+{bstr}{}", v.name().unwrap()).replace("+-", "-"));
                }
            }
            [u, v] => {
                let vstr = match u.id() == v.id() {
                    true => format!("{}^2", u.name().unwrap()),
                    false => format!("{} {}", u.name().unwrap(), v.name().unwrap()),
                };
                if quads.is_empty() {
                    quads.push(format!("{bstr}{}", vstr));
                } else {
                    quads.push(format!("+{bstr}{}", vstr).replace("+-", "-"));
                }
            }
            vars => {
                let vs = vars
                    .iter()
                    .map(|v| v.name().unwrap())
                    .collect::<Vec<String>>();
                // .join(" ");
                let mut varwithcount: IndexMap<String, usize> = IndexMap::new();
                for v in vs {
                    *varwithcount.entry(v).or_insert(0) += 1;
                }
                let vs = varwithcount
                    .into_iter()
                    .map(|(varname, count)| match count {
                        1 => varname,
                        n => format!("{}^{}", varname, n),
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                if hos.is_empty() {
                    hos.push(format!("{bstr}{}", vs));
                } else {
                    hos.push(format!("+{bstr}{}", vs).replace("+-", "-"));
                }
            }
        }
    }
    if hos.is_empty() && quads.is_empty() && lins.is_empty() && !const_str.is_empty() {
        return const_str.replace("+", "");
    }

    let mut resstrs = Vec::new();
    if !hos.is_empty() {
        resstrs.append(&mut hos);
    }
    if !quads.is_empty() {
        if !resstrs.is_empty() {
            resstrs.push("+".to_string());
        }
        resstrs.append(&mut quads);
    }
    if !lins.is_empty() {
        if !resstrs.is_empty() {
            resstrs.push("+".to_string());
        }
        resstrs.append(&mut lins);
    }
    if !const_str.is_empty() {
        resstrs.push(const_str);
    }
    let mut out = resstrs.join(" ");
    out = out.replace(" -", " - ");
    out = out.replace(" +", " + ");
    let re = Regex::new(r"\s+").unwrap();
    out = re.replace_all(&out, " ").to_string();
    out = out.replace("+ -", "-");
    match out.is_empty() {
        true => String::from("0"),
        false => out.to_string(),
    }
}
