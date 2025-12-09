use indexmap::IndexMap as HashMap;
use lunamodel_core::Expression;
use lunamodel_types::Bias;

#[derive(Debug, Clone)]
pub enum ExprTree {
    Num(Bias),
    Var(String),
    Neg(Box<ExprTree>),
    Add(Box<ExprTree>, Box<ExprTree>),
    Sub(Box<ExprTree>, Box<ExprTree>),
    Mul(Box<ExprTree>, Box<ExprTree>),
    Pow(Box<ExprTree>, Box<ExprTree>),
}

impl From<&Expression> for ExprTree {
    fn from(e: &Expression) -> Self {
        let mut base = Self::Num(0.0);
        for (vs, n) in e.items() {
            let b = n.abs();
            let new = match &vs[..] {
                [] => Self::Num(b),
                [u] => {
                    // linear
                    Self::Mul(
                        Box::new(Self::Num(b)),
                        Box::new(Self::Var(u.name().unwrap())),
                    )
                }
                [u, v] => {
                    // quadratic
                    Self::Mul(
                        Box::new(Self::Num(b)),
                        Box::new(Self::Mul(
                            Box::new(Self::Var(u.name().unwrap())),
                            Box::new(Self::Var(v.name().unwrap())),
                        )),
                    )
                }
                vars => {
                    // higher order
                    let mut map: HashMap<String, usize> = HashMap::new();
                    for v in vars {
                        // hashbrown::HashMap
                        // *map.entry_ref(&v.name().unwrap()).or_insert_with(|| 0) += 1;
                        // indexmap::IndexMap
                        *map.entry(v.name().unwrap()).or_insert_with(|| 0) += 1;
                    }

                    let mut ho = Self::Num(1.0);
                    for (name, &count) in map.iter() {
                        match count {
                            0 => unreachable!(),
                            1 => ho = Self::Mul(Box::new(ho), Box::new(Self::Var(name.into()))),
                            c => {
                                ho = Self::Mul(
                                    Box::new(ho),
                                    Box::new(Self::Pow(
                                        Box::new(Self::Var(name.into())),
                                        Box::new(Self::Num(c as f64)),
                                    )),
                                )
                            }
                        }
                    }
                    Self::Mul(Box::new(Self::Num(b)), Box::new(ho))
                }
            };
            if n < 0.0 {
                base = Self::Sub(Box::new(base), Box::new(new))
            } else {
                base = Self::Add(Box::new(base), Box::new(new))
            }
        }
        base.optimize()
    }
}

impl ExprTree {
    pub fn optimize(&self) -> Self {
        use ExprTree as T;
        match self {
            T::Add(lhs, rhs) => match (lhs.optimize(), rhs.optimize()) {
                (T::Num(a), T::Num(b)) => T::Num(a + b),
                (T::Num(n), e) | (e, T::Num(n)) if n == 0.0 => e.optimize(),
                (lhs, rhs) => {
                    if lhs.is_zero() {
                        rhs
                    } else if rhs.is_zero() {
                        lhs
                    } else {
                        T::Add(Box::new(lhs), Box::new(rhs))
                    }
                }
            },
            T::Sub(lhs, rhs) => match (lhs.optimize(), rhs.optimize()) {
                (T::Num(a), T::Num(b)) => T::Num(a - b),
                (e, T::Num(n)) if n == 0.0 => e.optimize(), // e - 0
                (T::Num(n), e) if n == 0.0 => T::Neg(Box::new(e.optimize())), // 0-e
                (lhs, rhs) => {
                    if rhs.is_zero() {
                        lhs
                    } else {
                        T::Sub(Box::new(lhs), Box::new(rhs))
                    }
                }
            },
            T::Mul(lhs, rhs) => match (lhs.optimize(), rhs.optimize()) {
                (T::Num(a), T::Num(b)) => T::Num(a * b),
                (T::Num(n), _) | (_, T::Num(n)) if n == 0.0 => T::Num(n),
                (T::Num(n), e) | (e, T::Num(n)) if n == 1.0 => e.optimize(),
                (lhs, rhs) => T::Mul(Box::new(lhs), Box::new(rhs)),
            },
            T::Pow(base, exp) => match (base.optimize(), exp.optimize()) {
                (_, T::Num(n)) if n == 0.0 => T::Num(1.0),  // x^0 = 1
                (e, T::Num(n)) if n == 1.0 => e.optimize(), // x^1 = x
                (T::Num(a), T::Num(b)) => T::Num(a.powf(b)),
                (base, exp) => T::Pow(Box::new(base), Box::new(exp)),
            },
            _ => self.clone(),
        }
    }

    pub fn is_zero(&self) -> bool {
        matches!(self, ExprTree::Num(b) if *b == 0.0)
    }
}

impl ToString for ExprTree {
    fn to_string(&self) -> String {
        use ExprTree as T;
        match self {
            T::Num(n) => format!("{}", n),
            T::Var(name) => name.into(),
            T::Add(lhs, rhs) => match (&**lhs, &**rhs) {
                (T::Num(n), e) | (e, T::Num(n)) => format!("{} + {}", e.to_string(), n),
                (l, T::Neg(r)) => format!("{} - {}", l.to_string(), r.to_string().replace("-", "")),
                (l, r) => format!("{} + {}", l.to_string(), r.to_string()),
            },
            T::Sub(lhs, rhs) => format!("{} - {}", lhs.to_string(), rhs.to_string()),
            T::Mul(lhs, rhs) => match (&**lhs, &**rhs) {
                (T::Num(n), T::Var(v)) | (T::Var(v), T::Num(n)) => format!("{} {}", n, v),
                (T::Num(n), T::Pow(base, exp)) | (T::Pow(base, exp), T::Num(n)) => {
                    format!("{} {}^{}", n, base.to_string(), exp.to_string())
                }
                (T::Num(n), T::Mul(l, r)) | (T::Mul(l, r), T::Num(n)) => match &**l {
                    T::Var(v) => format!("{} {} * {}", n, v, r.to_string()),
                    l => format!("{} * {} * {}", n, l.to_string(), r.to_string()),
                },
                _ => format!("{} * {}", lhs.to_string(), rhs.to_string()),
            },
            T::Pow(base, exp) => match &**exp {
                T::Num(n) => format!("{}^{}", base.to_string(), *n as usize),
                e => format!("{}^{}", base.to_string(), e.to_string()),
            },
            T::Neg(a) => format!("-{}", a.to_string()),
        }
    }
}
