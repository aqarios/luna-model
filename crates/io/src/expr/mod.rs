// mod exprtree;

use hashbrown::HashSet;
use lunamodel_core::Expression;
use lunamodel_types::Bias;
use regex::Regex;

use crate::{CustomFormat, FormatOpt}; // , expr::exprtree::ExprTree};

impl CustomFormat<FormatOpt> for Expression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>, _: &FormatOpt) -> std::fmt::Result {
        // let tree: ExprTree = self.into();
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

    let mut exprvarids = HashSet::new();
    for (vars, b) in expr.items() {
        let bstr = match b.abs() {
            1.0 => String::new(),
            _ => format!("{} ", b),
        };
        match &vars[..] {
            [] => {
                if b != Bias::default() {
                    const_str.push_str(&format!("+{}", &b.to_string()).replace("+-", "-"))
                }
            }
            [v] => {
                exprvarids.insert(v.id());
                if lins.is_empty() {
                    lins.push(format!("{bstr}{}", v.name().unwrap()));
                } else {
                    lins.push(format!("+{bstr}{}", v.name().unwrap()).replace("+-", "-"));
                }
            }
            [u, v] => {
                exprvarids.insert(u.id());
                exprvarids.insert(v.id());
                if quads.is_empty() {
                    quads.push(format!("{bstr}{} {}", u.name().unwrap(), v.name().unwrap()));
                } else {
                    quads.push(
                        format!("+{bstr}{} {}", u.name().unwrap(), v.name().unwrap())
                            .replace("+-", "-"),
                    );
                }
            }
            vars => {
                let vs = vars
                    .iter()
                    .map(|v| v.name().unwrap())
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

    let mut resstrs = Vec::new();
    if !hos.is_empty() {
        resstrs.append(&mut hos);
    }
    if !quads.is_empty() {
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
    out = out.replace("-", "- ");
    out = out.replace("+", "+ ");
    let re = Regex::new(r"\s+").unwrap();
    out = re.replace_all(&out, " ").to_string();
    out = out.replace("+ -", "-");
    out.to_string()
}
