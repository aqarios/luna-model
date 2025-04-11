use std::{marker::PhantomData, rc::Rc};

use crate::{
    core::{
        expression::{
            BiasConstraints, ExpressionBaseAdd, ExpressionBaseCreation, IndexConstraints,
        },
        operations::{AddToExpression, MulAssignToExpression, MulToExpression, SubToExpression},
        Expression, MutRcEnvironment, VarRef,
    },
    errors::TranslationErr,
};

// ExprTree AST
#[derive(Debug, Clone)]
pub enum ExprTree<Bias>
where
    Bias: BiasConstraints,
{
    Number(Bias),
    Variable(String),
    Add(Box<ExprTree<Bias>>, Box<ExprTree<Bias>>),
    Sub(Box<ExprTree<Bias>>, Box<ExprTree<Bias>>),
    Mul(Box<ExprTree<Bias>>, Box<ExprTree<Bias>>),
    Pow(Box<ExprTree<Bias>>, Box<ExprTree<Bias>>),
}

// Evaluation context
pub struct EvalContext<Index, Bias, F>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
    F: Fn(&str) -> VarRef<Index>,
{
    pub resolve_variable: F,
    pub env: MutRcEnvironment<Index>,
    _phantom: PhantomData<Bias>,
}

impl<Index, Bias, F> EvalContext<Index, Bias, F>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
    F: Fn(&str) -> VarRef<Index>,
{
    pub fn new(resolve_variable: F, env: MutRcEnvironment<Index>) -> Self {
        Self {
            resolve_variable,
            env,
            _phantom: PhantomData,
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
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Caret);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '[' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            c if c.is_ascii_digit() || c == '.' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            c if c.is_alphabetic() => {
                let mut name = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_alphanumeric() {
                        name.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Variable(name));
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    tokens
}

// Parser state
struct Parser<Bias: BiasConstraints> {
    tokens: Vec<Token>,
    pos: usize,
    _phantom: PhantomData<Bias>,
}

impl<Bias: BiasConstraints> Parser<Bias> {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            _phantom: PhantomData,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse_expression(&mut self) -> ExprTree<Bias> {
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

    fn parse_term(&mut self) -> ExprTree<Bias> {
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

    fn parse_factor(&mut self) -> ExprTree<Bias> {
        let mut base = self.parse_atom();
        while let Some(Token::Caret) = self.current() {
            self.advance();
            base = ExprTree::Pow(Box::new(base), Box::new(self.parse_atom()));
        }
        base
    }

    fn parse_atom(&mut self) -> ExprTree<Bias> {
        match self.current() {
            Some(Token::Number(n)) => {
                let bias = Bias::from(*n).unwrap();
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

impl<Bias> ExprTree<Bias>
where
    Bias: BiasConstraints,
{
    pub fn build(input: &str) -> Self {
        let tokens = tokenize(input);
        let mut parser = Parser::<Bias>::new(tokens);
        parser.parse_expression()
    }

    pub fn optimize(&mut self) -> Self {
        use ExprTree::*;

        match self {
            Add(lhs, rhs) => {
                let lhs = lhs.optimize();
                let rhs = rhs.optimize();

                match (&lhs, &rhs) {
                    (Number(a), Number(b)) => Number(*a + *b),
                    (Number(z), e) | (e, Number(z)) if is_zero(z) => e.clone(),
                    _ => Add(Box::new(lhs), Box::new(rhs)),
                }
            }

            Sub(lhs, rhs) => {
                let lhs = lhs.optimize();
                let rhs = rhs.optimize();

                match (&lhs, &rhs) {
                    (Number(a), Number(b)) => Number(*a - *b),
                    (e, Number(z)) if is_zero(z) => e.clone(),
                    _ => Sub(Box::new(lhs), Box::new(rhs)),
                }
            }

            Mul(lhs, rhs) => {
                let lhs = lhs.optimize();
                let rhs = rhs.optimize();

                match (&lhs, &rhs) {
                    (Number(a), Number(b)) => Number(*a * *b),
                    (Number(z), _) | (_, Number(z)) if is_zero(z) => Number(z.clone()),
                    (Number(o), e) | (e, Number(o)) if is_one(o) => e.clone(),
                    _ => Mul(Box::new(lhs), Box::new(rhs)),
                }
            }

            Pow(base, exp) => {
                let base = base.optimize();
                let exp = exp.optimize();

                match (&base, &exp) {
                    (_, Number(z)) if is_zero(z) => Number(Bias::one()), // x^0 = 1
                    (e, Number(o)) if is_one(o) => e.clone(),            // x^1 = x
                    (Number(a), Number(b)) => Number(a.pow(*b)),         // const^const
                    _ => Pow(Box::new(base), Box::new(exp)),
                }
            }

            _ => self.clone(),
        }
    }

    pub fn evaluate<Index, F>(
        self: &Self,
        ctx: &EvalContext<Index, Bias, F>,
    ) -> Result<Expression<Index, Bias>, TranslationErr>
    where
        Index: IndexConstraints,
        Bias: BiasConstraints,
        F: Fn(&str) -> VarRef<Index>,
    {
        use ExprTree::*;

        match self {
            Number(bias) => {
                let mut out = Expression::empty(Rc::clone(&ctx.env));
                out.add_offset(*bias);
                Ok(out)
            }
            Variable(name) => {
                let var = (ctx.resolve_variable)(name);
                Ok(Expression::new_linear_single(
                    Rc::clone(&ctx.env),
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
                        Expression::new_linear_single(Rc::clone(&ctx.env), var.id, Bias::one());
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

fn is_zero<B: BiasConstraints>(b: &B) -> bool {
    *b == B::default()
}

fn is_one<B: BiasConstraints>(b: &B) -> bool {
    *b == B::one()
}
