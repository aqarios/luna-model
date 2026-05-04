//! Tokenizer for LP expressions and statements.

use lunamodel_error::{LunaModelError, LunaModelResult};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Variable(String),
    Plus,
    Minus,
    Star,
    Caret,
    LBracket,
    RBracket,
}

pub fn tokenize(input: &str) -> LunaModelResult<Vec<Token>> {
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
                tokens.push(Token::LBracket);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RBracket);
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

                    // TODO: make this not expect 2.
                    // Expect '2'
                    if j < chars.len() && chars[j] == '2' {
                        divide_by_two = true;
                        i = j + 1; // Advance past the full "/ 2"
                    }
                }

                // Tokenize inside the brackets
                let sub_tokens = tokenize(&bracket_content)?;

                // Apply division if necessary
                if divide_by_two {
                    // Division by two: Multiply by '0.5' to ensure implicit multiplication by 1.0
                    // is handled.
                    tokens.push(Token::LBracket);
                    tokens.push(Token::Number(0.5));
                    tokens.push(Token::Star);
                    tokens.push(Token::LBracket);
                    tokens.extend(sub_tokens);
                    tokens.push(Token::RBracket);
                    tokens.push(Token::RBracket);
                } else {
                    // No division: Just wrap in parentheses
                    tokens.push(Token::LBracket);
                    tokens.extend(sub_tokens);
                    tokens.push(Token::RBracket);
                }
            }
            ']' => {
                tokens.push(Token::RBracket);
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
            c => {
                return Err(LunaModelError::Translation(
                    format!("Unexpected character: {}", c).into(),
                ));
            }
        }
    }
    Ok(tokens)
}
