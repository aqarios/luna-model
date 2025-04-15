use crate::errors::{TranslationErr, VariablesFromDifferentEnvsErr};
use std::iter::Peekable;
use std::str::Chars;

use super::keywords::{CommentKeywords, EndKeywords};

impl From<VariablesFromDifferentEnvsErr> for TranslationErr {
    fn from(value: VariablesFromDifferentEnvsErr) -> Self {
        TranslationErr::new(value.to_string())
    }
}

pub fn starts_with_any(s: &str, prefixes: &Vec<String>) -> bool {
    prefixes
        .iter()
        .any(|prefix| s.to_lowercase().starts_with(&prefix.to_lowercase()))
}

pub fn is_comment(line: &str) -> bool {
    line.trim().is_empty() || starts_with_any(line, &CommentKeywords::all())
}

pub fn is_end(line: &str) -> bool {
    line.trim().is_empty() || starts_with_any(line, &EndKeywords::all())
}

pub fn chunks(s: &str, max_len: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    let mut chars = s.chars().peekable();

    while let Some(_) = chars.peek() {
        // Peek ahead to find the next token
        let token = next_token(&mut chars);

        // If it's too long to fit on a line, flush current and force it onto a new line
        let token_len = token.chars().count();
        let current_len = current.chars().count();

        if token_len > max_len {
            if !current.is_empty() {
                chunks.push(current.clone());
                current.clear();
            }
            chunks.push(token);
            continue;
        }

        // If adding it would exceed max, flush current
        if current_len + token_len > max_len {
            if !current.is_empty() {
                chunks.push(current.clone());
                current.clear();
            }
        }

        current.push_str(&token);
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

// Helper to extract next token
fn next_token(chars: &mut Peekable<Chars>) -> String {
    let mut token = String::new();

    if let Some(&c) = chars.peek() {
        // If it's part of an underscore group, take the whole word
        if c.is_alphanumeric() {
            while let Some(&ch) = chars.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    token.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }
        }
        // If it's a whitespace, include it alone
        else if c.is_whitespace() {
            token.push(c);
            chars.next();
        }
        // If it's a symbol, include that symbol and maybe more up to the next whitespace (smart chunk)
        else {
            let split_before = ['*', '+', '-', '[', '('];

            token.push(c);
            chars.next();

            // After a "split-before" character, grab up to next whitespace after a non-whitespace
            if split_before.contains(&c) {
                let mut found_non_space = false;
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_whitespace() {
                        token.push(next_c);
                        chars.next();
                        if found_non_space {
                            break;
                        }
                    } else {
                        found_non_space = true;
                        token.push(next_c);
                        chars.next();
                    }
                }
            }
            // After a "split-after" character, just return the single character
        }
    }

    token
}
