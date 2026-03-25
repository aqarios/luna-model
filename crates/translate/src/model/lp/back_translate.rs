use std::{fs::File, io::Write, path::PathBuf};

use std::collections::HashSet;
use lunamodel_core::{ArcEnv, ConstraintCollection, Expression, Model, prelude::Bounds};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Bound, Comparator, VarId, Vtype};
use regex::Regex;

use super::LpTranslator;

static MAX_LINE_LENGTH: usize = 88;
static INDENT: &str = " ";
const SEP: &str = " ";

impl LpTranslator {
    pub fn back_translate(
        model: &Model,
        filepath: Option<PathBuf>,
    ) -> LunaModelResult<Option<String>> {
        let lpstr = Self::build_string(model)?;
        if let Some(pb) = filepath {
            Self::write_file(lpstr, &pb)?;
            Ok(None)
        } else {
            Ok(Some(lpstr))
        }
    }

    pub fn write_file(data: String, filepath: &PathBuf) -> LunaModelResult<()> {
        let mut file = File::create(filepath)
            .map_err(|why| LunaModelError::Translation(why.to_string().into()))?;
        file.write_all(data.as_bytes())
            .map_err(|why| LunaModelError::Translation(why.to_string().into()))?;
        Ok(())
    }

    pub fn build_string(model: &Model) -> LunaModelResult<String> {
        let mut out = String::new();
        out.push_str(&format!("\\ Model {}\n", model.name));
        out.push_str(&format!("\\ Problem name: {}\n", model.name));
        out.push_str("\n");
        out.push_str(&format!("{}\n", model.sense.to_string()));
        out.push_str(&format!(
            "{}\n",
            Self::expr_string(
                &model.objective,
                Some(
                    model
                        .constraints
                        .vars()
                        .map(|v| VarId(v.id()))
                        .collect::<Vec<_>>()
                        .as_slice()
                ),
                true
            )?
        ));
        out.push_str(&format!("Subject To\n"));
        out.push_str(&format!("{}\n", Self::constr_string(&model.constraints)?));
        out.push_str(&format!("Bounds\n"));
        out.push_str(&format!("{}\n", Self::bounds_string(&model.environment)?));
        out.push_str(&format!("{}\n", Self::variables_string(&model)?));
        out.push_str("End");
        Ok(out)
    }

    fn variables_string(model: &Model) -> LunaModelResult<String> {
        let mut bins = Vec::new();
        let mut gens = Vec::new();
        for v in model.vars() {
            match v.vtype()? {
                Vtype::Real => (),
                Vtype::Spin => {
                    return Err(LunaModelError::Translation(
                        "LP files cannot contain variables of type SPIN".into(),
                    ));
                }
                Vtype::InvertedBinary => todo!(),
                Vtype::Binary => bins.push(v.name()?),
                Vtype::Integer => gens.push(v.name()?),
            }
        }
        let mut out = String::new();
        if !bins.is_empty() {
            let mut binstr = String::from("Binaries\n ");
            let chunks = safe_chunks(&bins, MAX_LINE_LENGTH);
            binstr.push_str(&chunks.join("\n"));
            out.push_str(&binstr);
            out.push_str("\n");
        }
        if !gens.is_empty() {
            let mut genstr = String::from("Generals\n ");
            let chunks = safe_chunks(&gens, MAX_LINE_LENGTH);
            genstr.push_str(&chunks.join("\n"));
            out.push_str(&genstr);
        }
        Ok(out)
    }

    fn bounds_string(env: &ArcEnv) -> LunaModelResult<String> {
        let mut res = Vec::new();
        for v in env.vars() {
            match v.vtype()? {
                Vtype::Binary => (),
                Vtype::InvertedBinary => (),
                Vtype::Integer | Vtype::Real => {
                    let Bounds { lower, upper } = v.bounds()?;
                    let bstr = match (lower, upper) {
                        (Bound::Unbounded, Bound::Unbounded) => format!("{} free", v.name()?),
                        (Bound::Bounded(lower), Bound::Unbounded) => {
                            format!("{} >= {}", v.name()?, lower)
                        }
                        (Bound::Unbounded, Bound::Bounded(upper)) => {
                            format!("{} <= {}", v.name()?, upper)
                        }
                        (Bound::Bounded(lower), Bound::Bounded(upper)) => {
                            format!("{} <= {} <= {}", lower, v.name()?, upper)
                        }
                    };
                    res.push(format!("{INDENT}{bstr}"));
                }
                Vtype::Spin => {
                    return Err(LunaModelError::Translation(
                        "LP files cannot contain variables of type SPIN".into(),
                    ));
                }
            }
        }
        Ok(res.join("\n"))
    }

    fn expr_string(
        expr: &Expression,
        constr_vars: Option<&[VarId]>,
        obj: bool,
    ) -> LunaModelResult<String> {
        let mut const_str = String::new();
        let mut lins = Vec::new();
        let mut quads = Vec::new();
        let env = &expr.env;

        // additional_linears are variables contained in the constraints but not in the expression.
        let mut exprvarids = HashSet::new();
        for (vars, b) in expr.items() {
            match &vars[..] {
                [] => {
                    if b != Bias::default() {
                        const_str.push_str(&format!("+{}", &b.to_string()).replace("+-", "-"))
                    }
                }
                [v] => {
                    exprvarids.insert(v.id());
                    if lins.is_empty() {
                        lins.push(format!("{b} {}", v.name()?));
                    } else {
                        lins.push(format!("+{b} {}", v.name()?).replace("+-", "-"));
                    }
                }
                [u, v] => {
                    exprvarids.insert(u.id());
                    exprvarids.insert(v.id());
                    let bias = if obj { 2.0 * b } else { b };
                    if quads.is_empty() {
                        quads.push(format!("{} {} * {}", bias, u.name()?, v.name()?));
                    } else {
                        quads.push(
                            format!("+{} {} * {}", bias, u.name()?, v.name()?).replace("+-", "-"),
                        );
                    }
                }
                // vars => ho_str.push_str(&format!(
                //     "{b} {}",
                //     vars.iter().map(|v| env.name(v)).join(" ")
                // )),
                _ => {
                    return Err(LunaModelError::Translation(
                        "cannot create an LP file from a model with higher order terms".into(),
                    ));
                }
            }
        }

        if let Some(cv) = constr_vars {
            let mut additional_linears: HashSet<VarId> = cv.iter().map(|e| *e).collect();
            additional_linears.retain(|e| !exprvarids.contains(&e.0));
            for al in &additional_linears {
                let var: String = env.read_arc().get(al.0)?.name().into();
                lins.push(format!("+0 {}", var));
            }
        }

        // let mut resstr = String::new();
        let mut resstrs = Vec::new();
        if !quads.is_empty() {
            quads.insert(0, "[".to_string());
            if obj {
                quads.push("] / 2".to_string());
            } else {
                quads.push("]".to_string());
            }
            // resstr.push_str(&format!("+{quadstr}"));
            // let mut split_quad = quadstr.split("+").map(|e| e.to_string()).collect_vec();
            resstrs.append(&mut quads);
        }
        if !lins.is_empty() {
            if !resstrs.is_empty() {
                resstrs.push("+".to_string());
            }
            resstrs.append(&mut lins);
        }
        if !const_str.is_empty() {
            // resstr.push_str(&format!("+{const_str}"));
            resstrs.push(const_str);
        }
        let out = if obj {
            let mut outs = Vec::new();
            let chunks = safe_chunks(&resstrs, MAX_LINE_LENGTH);
            for chunk in chunks {
                // let chunk = chunk.replace("+-", "-");
                let mut chunk = chunk.replace("-", "- ");
                chunk = chunk.replace("+", "+ ");
                // let chunk = chunk.replace("  ", " ");
                let chunk = chunk.trim().to_string();
                outs.push(format!("{INDENT}{chunk}"));
            }
            outs.join("\n")
        } else {
            let mut out = resstrs.join(" ");
            out = out.replace("-", "- ");
            out = out.replace("+", "+ ");
            out
        };

        let re = Regex::new(r"\s+").unwrap();
        let mut out = re.replace_all(&out, " ").to_string();
        out = out.replace("+ -", "-");
        Ok(out.to_string())
    }

    fn constr_string(constr: &ConstraintCollection) -> LunaModelResult<String> {
        let mut out: Vec<String> = Vec::new();
        for (name, c) in constr.iter() {
            let lhsstr = Self::expr_string(&c.lhs, None, false)?;
            let cstr = format!(
                "{name}: {} {} {}",
                lhsstr,
                match &c.comparator {
                    Comparator::Le => "<=",
                    Comparator::Eq => "=",
                    Comparator::Ge => ">=",
                },
                c.rhs,
            );
            out.push(format!("{INDENT}{cstr}"));
        }
        Ok(out.join("\n"))
    }
}

fn safe_chunks(d: &[String], max_len: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut buffer: String = String::default();
    for entry in d {
        let next_length = buffer.len() + entry.len();
        if next_length > max_len {
            // Adding `entry` to the current buffer would result in a
            // line exceeding the maximum allowed length.
            // => Flush the buffer into the chunks output and create a new buffer.
            chunks.push(buffer);
            buffer = entry.clone();
            continue;
        }
        // We can safely add the current entry to the current buffer.
        // We only need the separator if it is NOT the first entry.
        if buffer.is_empty() {
            buffer.push_str(entry);
        } else {
            buffer.push_str(&format!("{SEP}{entry}"));
        }
    }
    // Finally, we push the last buffer into the chunks if it is not empty.
    if !buffer.is_empty() {
        chunks.push(buffer);
    }
    chunks
}
