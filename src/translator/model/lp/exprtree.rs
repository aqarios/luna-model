use crate::core::expression::One;
use crate::{
    core::{
        environment::SharedEnvironment,
        expression::{ExpressionBaseAdd, ExpressionBaseCreation},
        operations::{AddToExpression, MulAssignToExpression, MulToExpression, SubToExpression},
        Expression, VarRef,
    },
    errors::TranslationErr,
    types::Bias,
};
use num::traits::Pow;
use std::ops::AddAssign;

// ExprTree AST
#[derive(Debug, Clone)]
pub enum ExprTree {
    Number(Bias),
    Variable(String),
    Add(Box<ExprTree>, Box<ExprTree>),
    Sub(Box<ExprTree>, Box<ExprTree>),
    Mul(Box<ExprTree>, Box<ExprTree>),
    Pow(Box<ExprTree>, Box<ExprTree>),
}

// Evaluation context
pub struct EvalContext<F>
where
    F: Fn(&str) -> VarRef,
{
    pub resolve_variable: F,
    pub env: SharedEnvironment,
}

impl<F> EvalContext<F>
where
    F: Fn(&str) -> VarRef,
{
    pub fn new(resolve_variable: F, env: SharedEnvironment) -> Self {
        Self {
            resolve_variable,
            env,
        }
    }
}

// Token type for simple tokenizer
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Variable(String),
    Plus,
    Minus,
    Star,
    Caret,
    LParen,
    RParen,
}

// Tokenizer function
// Tokenizer function with robust "/ 2" handling
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' => {
                i += 1;
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1;
            }
            '^' => {
                tokens.push(Token::Caret);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            '[' => {
                // Capture content inside brackets
                let mut bracket_content = String::new();
                i += 1;
                while i < chars.len() && chars[i] != ']' {
                    bracket_content.push(chars[i]);
                    i += 1;
                }

                // Skip closing ']'
                i += 1;

                // Check for optional "/ 2" with arbitrary spaces
                let mut divide_by_two = false;
                let mut j = i;

                // Skip whitespace
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }

                // Expect '/'
                if j < chars.len() && chars[j] == '/' {
                    j += 1;

                    // Skip whitespace after '/'
                    while j < chars.len() && chars[j].is_whitespace() {
                        j += 1;
                    }

                    // Expect '2'
                    if j < chars.len() && chars[j] == '2' {
                        divide_by_two = true;
                        i = j + 1; // Advance past the full "/ 2"
                    }
                }

                // Tokenize inside the brackets
                let sub_tokens = tokenize(&bracket_content);

                // Apply division if necessary
                if divide_by_two {
                    // Division by two: Multiply by '0.5' to ensure implicit multiplication by 1.0
                    // is handled.
                    tokens.push(Token::LParen);
                    tokens.push(Token::Number(0.5));
                    tokens.push(Token::Star);
                    tokens.push(Token::LParen);
                    tokens.extend(sub_tokens);
                    tokens.push(Token::RParen);
                    tokens.push(Token::RParen);
                } else {
                    // No division: Just wrap in parentheses
                    tokens.push(Token::LParen);
                    tokens.extend(sub_tokens);
                    tokens.push(Token::RParen);
                }
            }
            ']' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            c if c.is_ascii_digit() || c == '.' => {
                let mut num = String::new();
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    num.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::Number(num.parse::<f64>().unwrap()));
            }
            c if c.is_alphabetic() => {
                let mut name = String::new();
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    name.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::Variable(name));
            }
            c => panic!("Unexpected character: {}", c),
        }
    }

    tokens
}

// Parser state
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse_expression(&mut self) -> ExprTree {
        let mut expr = self.parse_term();
        while let Some(token) = self.current() {
            match token {
                Token::Plus => {
                    self.advance();
                    expr = ExprTree::Add(Box::new(expr), Box::new(self.parse_term()));
                }
                Token::Minus => {
                    self.advance();
                    expr = ExprTree::Sub(Box::new(expr), Box::new(self.parse_term()));
                }
                _ => break,
            }
        }
        expr
    }

    fn parse_term(&mut self) -> ExprTree {
        let mut expr = self.parse_factor();

        while let Some(token) = self.current() {
            match token {
                Token::Star => {
                    self.advance();
                    expr = ExprTree::Mul(Box::new(expr), Box::new(self.parse_factor()));
                }

                // Handle implicit multiplication
                Token::Variable(_) | Token::Number(_) | Token::LParen => {
                    let rhs = self.parse_factor();
                    expr = ExprTree::Mul(Box::new(expr), Box::new(rhs));
                }

                _ => break,
            }
        }

        expr
    }

    fn parse_factor(&mut self) -> ExprTree {
        let mut base = self.parse_atom();
        while let Some(Token::Caret) = self.current() {
            self.advance();
            base = ExprTree::Pow(Box::new(base), Box::new(self.parse_atom()));
        }
        base
    }

    fn parse_atom(&mut self) -> ExprTree {
        match self.current() {
            Some(Token::Plus) => {
                self.advance(); // skip '+'
                self.parse_atom() // unary plus, just pass through
            }
            Some(Token::Minus) => {
                self.advance(); // skip '-'
                let expr = self.parse_atom();
                ExprTree::Mul(
                    Box::new(ExprTree::Number(Bias::one() * -1.0)),
                    Box::new(expr),
                )
            }
            Some(Token::Number(n)) => {
                let bias = Bias::from(*n);
                self.advance();
                ExprTree::Number(bias)
            }
            Some(Token::Variable(name)) => {
                let var = name.clone();
                self.advance();
                ExprTree::Variable(var)
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expression();
                self.advance(); // consume RParen
                expr
            }
            other => panic!("Unexpected token: {:?}", other),
        }
    }
}

pub struct ExprTreeTuple {
    lin: Option<ExprTree>,
    quad: Option<ExprTree>,
    ho: Option<ExprTree>,
    cons: Option<ExprTree>,
}

impl ExprTreeTuple {
    fn new(
        lin: Option<ExprTree>,
        quad: Option<ExprTree>,
        ho: Option<ExprTree>,
        cons: Option<ExprTree>,
    ) -> Self {
        Self {
            lin,
            quad,
            ho,
            cons,
        }
    }

    pub fn optimize(&mut self) -> &mut Self {
        self.lin = self.lin.as_mut().and_then(|e| Some(e.optimize()));
        self.quad = self.quad.as_mut().and_then(|e| Some(e.optimize()));
        self.ho = self.ho.as_mut().and_then(|e| Some(e.optimize()));
        self
    }
}

impl ExprTree {
    pub fn build(input: &str) -> Self {
        let tokens = tokenize(input);
        let mut parser = Parser::new(tokens);
        parser.parse_expression()
    }

    pub fn from_expression(
        expr: &Expression,
        is_constraint: bool,
    ) -> Result<ExprTreeTuple, TranslationErr> {
        // Constant
        let constant = if expr.offset != Bias::default() {
            Some(ExprTree::Number(expr.offset))
        } else {
            None
        };

        // Linear terms
        let lintree = if expr.linear.is_zero() {
            None
        } else {
            let mut lintree = ExprTree::Number(Bias::default());
            for (u, bias) in expr.linear.iter() {
                let vname = expr.env.access()[u].name.clone();
                let mul = ExprTree::Mul(
                    Box::new(ExprTree::Number(*bias)),
                    Box::new(ExprTree::Variable(vname)),
                );
                lintree = ExprTree::Add(Box::new(lintree), Box::new(mul));
            }
            Some(lintree)
        };
        // Quadratic terms
        let quadtree = if let Some(q) = &expr.quadratic {
            let mut quadtree = ExprTree::Number(Bias::default());
            for (u, v, bias) in q.iter_flat() {
                if u == v {
                    // Pow
                    let u_name = expr.env.access()[u].name.clone();
                    let pow = ExprTree::Pow(
                        Box::new(ExprTree::Variable(u_name)),
                        Box::new(ExprTree::Number(Bias::one() * 2.0)),
                    );
                    let num = if is_constraint { bias } else { bias * 2.0 };
                    let mul = ExprTree::Mul(Box::new(ExprTree::Number(num)), Box::new(pow));
                    quadtree = ExprTree::Add(Box::new(quadtree), Box::new(mul));
                } else {
                    // Mul
                    let u_name = expr.env.access()[u].name.clone();
                    let v_name = expr.env.access()[v].name.clone();
                    let vmul = ExprTree::Mul(
                        Box::new(ExprTree::Variable(u_name)),
                        Box::new(ExprTree::Variable(v_name)),
                    );
                    let num = if is_constraint { bias } else { bias * 2.0 };
                    let mul = ExprTree::Mul(Box::new(ExprTree::Number(num)), Box::new(vmul));
                    quadtree = ExprTree::Add(Box::new(quadtree), Box::new(mul));
                }
            }
            Some(quadtree)
        } else {
            None
        };
        // HigherOrder terms
        if expr.has_higher_order() {
            return Err(TranslationErr::new(
                "cannot create an LP file from a model with higher order terms".to_string(),
            ));
        }
        Ok(ExprTreeTuple::new(lintree, quadtree, None, constant))
    }

    pub fn from_expression_internal(expr: &Expression) -> Result<ExprTreeTuple, TranslationErr> {
        // Constant
        let constant = if expr.offset != Bias::default() {
            Some(ExprTree::Number(expr.offset))
        } else {
            None
        };

        // Linear terms
        let lintree = if expr.linear.is_zero() {
            None
        } else {
            let mut lintree = ExprTree::Number(Bias::default());
            for (u, bias) in expr.linear.iter() {
                let vname = expr.env.access()[u].name.clone();
                let mul = ExprTree::Mul(
                    Box::new(ExprTree::Number(*bias)),
                    Box::new(ExprTree::Variable(vname)),
                );
                lintree = ExprTree::Add(Box::new(lintree), Box::new(mul));
            }
            Some(lintree)
        };
        // Quadratic terms
        let quadtree = if let Some(q) = &expr.quadratic {
            let mut quadtree = ExprTree::Number(Bias::default());
            for (u, v, bias) in q.iter_flat() {
                if u == v {
                    // Pow
                    let u_name = expr.env.access()[u].name.clone();
                    let pow = ExprTree::Pow(
                        Box::new(ExprTree::Variable(u_name)),
                        Box::new(ExprTree::Number(Bias::one() * 2.0)),
                    );
                    let num = bias;
                    let mul = ExprTree::Mul(Box::new(ExprTree::Number(num)), Box::new(pow));
                    quadtree = ExprTree::Add(Box::new(quadtree), Box::new(mul));
                } else {
                    // Mul
                    let u_name = expr.env.access()[u].name.clone();
                    let v_name = expr.env.access()[v].name.clone();
                    let vmul = ExprTree::Mul(
                        Box::new(ExprTree::Variable(u_name)),
                        Box::new(ExprTree::Variable(v_name)),
                    );
                    let num = bias;
                    let mul = ExprTree::Mul(Box::new(ExprTree::Number(num)), Box::new(vmul));
                    quadtree = ExprTree::Add(Box::new(quadtree), Box::new(mul));
                }
            }
            Some(quadtree)
        } else {
            None
        };
        // HigherOrder terms
        let hotree = if let Some(ho) = &expr.higher_order {
            let mut hotree = ExprTree::Number(Bias::default());
            for (vs, bias) in ho.iter_contrib() {
                let mut ho_mul = ExprTree::Number(*bias);
                for v in vs {
                    let v_name = expr.env.access()[v].name.clone();
                    ho_mul = ExprTree::Mul(Box::new(ho_mul), Box::new(ExprTree::Variable(v_name)))
                }
                hotree = ExprTree::Add(Box::new(hotree), Box::new(ho_mul));
            }
            Some(hotree)
        } else {
            None
        };
        Ok(ExprTreeTuple::new(lintree, quadtree, hotree, constant))
    }
    pub fn optimize(&self) -> Self {
        use ExprTree::*;

        match self {
            Add(lhs, rhs) => {
                let lhs = lhs.optimize();
                let rhs = rhs.optimize();

                match (&lhs, &rhs) {
                    (Number(a), Number(b)) => Number(*a + *b),
                    (Number(z), e) | (e, Number(z)) if is_zero(z) => e.optimize(),
                    _ => {
                        if is_zero_expr(&lhs) {
                            rhs
                        } else if is_zero_expr(&rhs) {
                            lhs
                        } else {
                            Add(Box::new(lhs), Box::new(rhs))
                        }
                    }
                }
            }

            Sub(lhs, rhs) => {
                let lhs = lhs.optimize();
                let rhs = rhs.optimize();

                match (&lhs, &rhs) {
                    (Number(a), Number(b)) => Number(*a - *b),
                    (e, Number(z)) if is_zero(z) => e.optimize(),
                    _ => {
                        if is_zero_expr(&rhs) {
                            lhs
                        } else {
                            Sub(Box::new(lhs), Box::new(rhs))
                        }
                    }
                }
            }

            Mul(lhs, rhs) => {
                let lhs = lhs.optimize();
                let rhs = rhs.optimize();

                match (&lhs, &rhs) {
                    (Number(a), Number(b)) => Number(*a * *b),
                    (Number(z), _) | (_, Number(z)) if is_zero(z) => Number(*z),
                    (Number(o), e) | (e, Number(o)) if is_one(o) => e.optimize(),
                    _ => Mul(Box::new(lhs), Box::new(rhs)),
                }
            }

            Pow(base, exp) => {
                let base = base.optimize();
                let exp = exp.optimize();

                match (&base, &exp) {
                    (_, Number(z)) if is_zero(z) => Number(Bias::one()), // x^0 = 1
                    (e, Number(o)) if is_one(o) => e.optimize(),         // x^1 = x
                    (Number(a), Number(b)) => Number(a.pow(*b)),
                    _ => Pow(Box::new(base), Box::new(exp)),
                }
            }

            Number(bias) if *bias == Bias::default() => Number(Bias::default()),

            _ => self.clone(),
        }
    }

    pub fn evaluate<F>(self: &Self, ctx: &EvalContext<F>) -> Result<Expression, TranslationErr>
    where
        F: Fn(&str) -> VarRef,
    {
        use ExprTree::*;

        match self {
            Number(bias) => {
                let mut out = Expression::empty(ctx.env.clone());
                out.add_offset(*bias);
                Ok(out)
            }
            Variable(name) => {
                let var = (ctx.resolve_variable)(name);
                Ok(Expression::new_linear_single(
                    ctx.env.clone(),
                    var.id,
                    Bias::one(),
                ))
            }
            Add(lhs, rhs) => {
                let l = lhs.evaluate(ctx)?;
                let r = rhs.evaluate(ctx)?;
                Ok(l.add(&r)?)
            }
            Sub(lhs, rhs) => {
                let l = lhs.evaluate(ctx)?;
                let r = rhs.evaluate(ctx)?;
                Ok(l.sub(&r)?)
            }
            Mul(lhs, rhs) => {
                let l = lhs.evaluate(ctx)?;
                let r = rhs.evaluate(ctx)?;
                Ok(l.mul(&r)?)
            }
            Pow(base, exp) => match (&**base, &**exp) {
                (Variable(name), Number(bias)) => {
                    let var = (ctx.resolve_variable)(name);
                    let mut base =
                        Expression::new_linear_single(ctx.env.clone(), var.id, Bias::one());
                    let mut count = Bias::one();
                    while count < *bias {
                        base.mul_assign(&var)?;
                        count.add_assign(Bias::one());
                    }
                    Ok(base)
                }
                _ => panic!("Pow is only supported for Variable ^ Number"),
            },
        }
    }
}

impl ToString for ExprTree {
    fn to_string(&self) -> String {
        use ExprTree::*;

        match self {
            Number(b) => {
                if b <= &Bias::default() {
                    b.to_string().replace("-", "- ")
                } else {
                    b.to_string()
                }
            }

            Variable(name) => name.clone(),

            Add(lhs, rhs) => {
                let tmp = match (&**lhs, &**rhs) {
                    (Number(b), r) => format!("{} + {}", r.to_string(), b.to_string()),
                    (l, Number(b)) => format!("{} + {}", l.to_string(), b.to_string()),
                    (l, r) => format!("{} + {}", l.to_string(), r.to_string()),
                };
                tmp.replace("+ -", "- ")
            }

            Sub(lhs, rhs) => {
                format!("{} - {}", lhs.to_string(), rhs.to_string())
            }

            Mul(lhs, rhs) => match (&**lhs, &**rhs) {
                (Number(b), Variable(v)) => format!("{} {}", b.to_string(), v),
                (Variable(v), Number(b)) => format!("{} {}", b.to_string(), v),
                (Number(b), Pow(base, exp)) => format!(
                    "{} {} ^ {}",
                    b.to_string(),
                    base.to_string(),
                    exp.to_string()
                ),
                (Number(b), Mul(lhs, rhs)) => match &**lhs {
                    Variable(v) => {
                        format!("{} {} * {}", b.to_string(), v.to_string(), rhs.to_string())
                    }
                    _ => format!(
                        "{} * {} * {}",
                        b.to_string(),
                        lhs.to_string(),
                        rhs.to_string()
                    ),
                },
                _ => {
                    format!("{} * {}", lhs.to_string(), rhs.to_string())
                }
            },

            Pow(base, exp) => {
                format!("{} ^ {}", base.to_string(), exp.to_string())
            }
        }
    }
}

impl ExprTreeTuple {
    pub fn to_string(&self, is_obj: bool) -> String {
        let mut result = String::new();
        let quadstr = self.quad.as_ref().and_then(|q| Some(q.to_string()));
        if let Some(qs) = &quadstr {
            result.push_str("[ ");
            result.push_str(&qs);
            result.push_str(" ]");
            if is_obj {
                result.push_str(" / 2");
            }
        }
        if let Some(lin) = &self.lin {
            let linstr = lin.to_string();
            if quadstr.is_none() {
                result.push_str(&format!("{linstr}"));
            } else {
                result.push_str(&format!(" + {linstr}"));
            }
        }
        if let Some(constant) = &self.cons {
            result.push_str(&format!(" + {}", constant.to_string()));
        }
        result.replace("+ -", "-").replace("+-", "-")
    }

    pub fn to_repr(&self) -> String {
        let mut result = String::new();
        let linstr = self.lin.as_ref().and_then(|l| Some(l.to_string()));
        let quadstr = self.quad.as_ref().and_then(|q| Some(q.to_string()));
        let hostr = self.ho.as_ref().and_then(|h| Some(h.to_string()));
        let cons = self.cons.as_ref().and_then(|c| Some(c.to_string()));

        if let Some(lin) = &linstr {
            result.push_str(lin)
        }

        if let Some(quad) = &quadstr {
            if linstr.is_some() {
                result.push_str(&format!("+ {}", quad));
            } else {
                result.push_str(quad);
            }
        }

        if let Some(ho) = &hostr {
            if linstr.is_some() || quadstr.is_some() {
                result.push_str(&format!("+ {}", ho));
            } else {
                result.push_str(ho);
            }
        }

        if let Some(cons) = &cons {
            if linstr.is_some() || quadstr.is_some() || hostr.is_some() {
                result.push_str(&format!(" + {}", cons));
            } else {
                result.push_str(cons);
            }
        }
        result.replace("+ -", "-").replace("+-", "-")
    }
}

fn is_zero(b: &Bias) -> bool {
    *b == Bias::default()
}

fn is_zero_expr(e: &ExprTree) -> bool {
    matches!(e, ExprTree::Number(b) if is_zero(b))
}

fn is_one(b: &Bias) -> bool {
    *b == Bias::one()
}
