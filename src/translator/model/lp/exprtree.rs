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
    Neg(Box<ExprTree>),
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
// #[derive(Debug)]
struct Parser {
    //     tokens: Vec<Token>,
    //     pos: usize,
}

impl Parser {
    pub fn parse_expression(tokens: &[Token]) -> Result<ExprTree, String> {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Op {
            Add,
            Sub,
            Mul,
            Pow,
            UMinus,
            LParen,
        }

        fn prec(op: Op) -> u8 {
            match op {
                Op::Add | Op::Sub => 10,
                Op::Mul => 20,    // implicit '*' uses same precedence
                Op::Pow => 30,    // right-associative
                Op::UMinus => 40, // prefix, highest
                Op::LParen => 0,
            }
        }

        fn is_right_assoc(op: Op) -> bool {
            matches!(op, Op::Pow | Op::UMinus)
        }

        // Helpers to detect when implicit multiplication is required.
        fn can_end_term(tok: &Token) -> bool {
            matches!(tok, Token::Number(_) | Token::Variable(_) | Token::RParen)
        }
        fn can_start_term(tok: &Token) -> bool {
            matches!(tok, Token::Number(_) | Token::Variable(_) | Token::LParen)
        }

        // // Pretty helper for error messages
        // fn tok_str(t: &Token) -> &'static str {
        //     match t {
        //         Token::Number(_) => "number",
        //         Token::Variable(_) => "variable",
        //         Token::Plus => "+",
        //         Token::Minus => "-",
        //         Token::Star => "*",
        //         Token::Caret => "^",
        //         Token::LParen => "(",
        //         Token::RParen => ")",
        //     }
        // }

        let mut vals: Vec<ExprTree> = Vec::new();
        let mut ops: Vec<Op> = Vec::new();

        // Applies a single operator sitting on `ops` to `vals`.
        let apply_top = |ops: &mut Vec<Op>, vals: &mut Vec<ExprTree>| -> Result<(), String> {
            let op = ops.pop().expect("apply_top called with empty ops");

            match op {
                Op::UMinus => {
                    let a = vals.pop().ok_or("unary minus: missing operand")?;
                    let out = ExprTree::Neg(Box::new(a));
                    vals.push(out);
                }
                Op::Add | Op::Sub | Op::Mul | Op::Pow => {
                    let rhs = vals
                        .pop()
                        .ok_or_else(|| format!("operator {:?}: missing right operand", op))?;
                    let lhs = vals
                        .pop()
                        .ok_or_else(|| format!("operator {:?}: missing left operand", op))?;
                    let out: ExprTree = match op {
                        Op::Add => ExprTree::Add(Box::new(lhs), Box::new(rhs)), 
                        Op::Sub => ExprTree::Sub(Box::new(lhs), Box::new(rhs)),
                        Op::Mul => ExprTree::Mul(Box::new(lhs), Box::new(rhs)),
                        Op::Pow => ExprTree::Pow(Box::new(lhs), Box::new(rhs)),
                        _ => unreachable!(),
                    };
                    // example if building AST: let out = Expr::Bin(op_to_ast(op), Box::new(lhs), Box::new(rhs));
                    vals.push(out);
                }
                Op::LParen => unreachable!("should not directly apply LParen"),
            }

            Ok(())
        };

        // Push a (possibly implicit) operator, respecting precedence/associativity
        let push_op =
            |new_op: Op, ops: &mut Vec<Op>, vals: &mut Vec<ExprTree>| -> Result<(), String> {
                while let Some(&top) = ops.last() {
                    if top == Op::LParen {
                        break;
                    }
                    let p_top = prec(top);
                    let p_new = prec(new_op);

                    let should_pop = if is_right_assoc(new_op) {
                        // Right-assoc: pop only strictly higher precedence
                        p_top > p_new
                    } else {
                        // Left-assoc: pop higher *or equal* precedence
                        p_top >= p_new
                    };

                    if should_pop {
                        apply_top(ops, vals)?;
                    } else {
                        break;
                    }
                }
                ops.push(new_op);
                Ok(())
            };

        // Scan with implicit-multiplication insertion and unary-minus handling
        let mut i = 0usize;
        let mut prev_token: Option<&Token> = None;

        while i < tokens.len() {
            let t = &tokens[i];

            // Insert implicit '*' when a term-ending token is followed by a term-starting token.
            if let Some(prev) = prev_token {
                if can_end_term(prev) && can_start_term(t) {
                    // behave as if a '*' appeared here
                    push_op(Op::Mul, &mut ops, &mut vals)?;
                }
            }

            match t {
                Token::Number(n) => {
                    // let v: R = todo!("implement literal number {}", n);
                    // example AST: let v = Expr::Number(*n);
                    let v = ExprTree::Number(*n);
                    vals.push(v);
                }
                Token::Variable(name) => {
                    // let v: R = todo!("implement variable `{}`", name);
                    // example AST: let v = Expr::Var(name.clone());
                    let v = ExprTree::Variable(name.clone());
                    vals.push(v);
                }
                Token::LParen => {
                    ops.push(Op::LParen);
                }
                Token::RParen => {
                    // Pop until matching '('
                    while let Some(&top) = ops.last() {
                        if top == Op::LParen {
                            break;
                        }
                        apply_top(&mut ops, &mut vals)?;
                    }
                    match ops.pop() {
                        Some(Op::LParen) => { /* matched */ }
                        _ => return Err("unmatched ')'".to_string()),
                    }
                }
                Token::Plus => {
                    // Unary '+' is a no-op; detect and skip it.
                    let unary = match prev_token {
                        None => true,
                        Some(Token::LParen) => true,
                        Some(Token::Plus | Token::Minus | Token::Star | Token::Caret) => true,
                        _ => false,
                    };
                    if !unary {
                        push_op(Op::Add, &mut ops, &mut vals)?;
                    }
                }
                Token::Minus => {
                    // Detect unary vs binary '-'
                    let unary = match prev_token {
                        None => true,
                        Some(Token::LParen) => true,
                        Some(Token::Plus | Token::Minus | Token::Star | Token::Caret) => true,
                        _ => false,
                    };
                    if unary {
                        // Treat as prefix operator with highest precedence
                        push_op(Op::UMinus, &mut ops, &mut vals)?;
                    } else {
                        push_op(Op::Sub, &mut ops, &mut vals)?;
                    }
                }
                Token::Star => {
                    push_op(Op::Mul, &mut ops, &mut vals)?;
                }
                Token::Caret => {
                    push_op(Op::Pow, &mut ops, &mut vals)?;
                }
            }

            prev_token = Some(t);
            i += 1;
        }

        // Close any remaining operators
        while let Some(top) = ops.pop() {
            if top == Op::LParen {
                return Err("unmatched '('".to_string());
            }
            // We popped one already; re-apply via helper that expects it on the stack:
            ops.push(top);
            apply_top(&mut ops, &mut vals)?;
            // let _ = ops.pop(); // remove the one we just applied
        }

        if vals.len() == 1 {
            Ok(vals.pop().unwrap())
        } else if vals.is_empty() {
            Err("empty expression".to_string())
        } else {
            Err(format!(
                "parser ended with {} values on the stack (missing operators?)",
                vals.len()
            ))
        }
    }
}

pub struct ExprTreeTuple {
    pub lin: Option<ExprTree>,
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
        let tree = Parser::parse_expression(&tokens).unwrap(); // todo: handle unwrap
        tree
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
                    Box::new(ExprTree::Number(bias)),
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
                    Box::new(ExprTree::Number(bias)),
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
            Neg(a) => {
                let a = a.evaluate(ctx)?;
                Ok(a.mul(-1.0))
            }
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
            Neg(a) => {
                format!("- {}", a.to_string())
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
