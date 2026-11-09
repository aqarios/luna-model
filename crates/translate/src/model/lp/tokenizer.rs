//! Splits LP text into tokens.

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Comparator;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    Num(f64),
    Name(&'a str),
    Cmp(Comparator),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Colon,
    LBracket,
    RBracket,
}

/// A token and the line it starts on (1-based).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spanned<'a> {
    pub token: Token<'a>,
    pub line: usize,
}

/// Characters allowed in names besides ASCII letters, digits and brackets.
const NAME_EXTRA: &[u8] = br#"_.,;(){}"'@#!$%&?"#;

pub fn tokenize(src: &str) -> LunaModelResult<Vec<Spanned<'_>>> {
    let bytes = src.as_bytes();
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut i = 0;

    while i < bytes.len() {
        let (token, end) = match bytes[i] {
            b'\n' => {
                line += 1;
                i += 1;
                continue;
            }
            b' ' | b'\t' | b'\r' => {
                i += 1;
                continue;
            }
            b'\\' => {
                // A comment runs until the end of the line.
                i = skip_while(bytes, i, |c| c != b'\n');
                continue;
            }
            b'+' => (Token::Plus, i + 1),
            b'-' => (Token::Minus, i + 1),
            b'*' => (Token::Star, i + 1),
            b'/' => (Token::Slash, i + 1),
            b'^' => (Token::Caret, i + 1),
            b':' => (Token::Colon, i + 1),
            b'[' => (Token::LBracket, i + 1),
            b']' => (Token::RBracket, i + 1),
            b'<' => (Token::Cmp(Comparator::Le), skip_eq(bytes, i + 1)),
            b'>' => (Token::Cmp(Comparator::Ge), skip_eq(bytes, i + 1)),
            b'=' => match bytes.get(i + 1) {
                Some(b'<') => (Token::Cmp(Comparator::Le), i + 2),
                Some(b'>') => (Token::Cmp(Comparator::Ge), i + 2),
                Some(b'=') => (Token::Cmp(Comparator::Eq), i + 2),
                _ => (Token::Cmp(Comparator::Eq), i + 1),
            },
            b'0'..=b'9' | b'.' => number(src, i, line)?,
            c if c.is_ascii_alphabetic() => name(src, i),
            _ => {
                let c = src[i..].chars().next().unwrap_or_default();
                return Err(error(line, format!("unexpected character '{c}'")));
            }
        };
        tokens.push(Spanned { token, line });
        i = end;
    }
    Ok(tokens)
}

/// Translation error that names the line it happened on.
pub fn error(line: usize, msg: String) -> LunaModelError {
    LunaModelError::Translation(format!("line {line}: {msg}").into())
}

/// Scans `digits[.digits][e[+-]digits]`. Rust's float parser does the conversion.
fn number(src: &str, start: usize, line: usize) -> LunaModelResult<(Token<'_>, usize)> {
    let bytes = src.as_bytes();
    let mut end = skip_while(bytes, start, |c| c.is_ascii_digit() || c == b'.');

    // `e` only belongs to the number if digits follow, so `3 e` stays a name.
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        let sign = matches!(bytes.get(end + 1), Some(b'+' | b'-'));
        let digits = end + 1 + sign as usize;
        if bytes.get(digits).is_some_and(u8::is_ascii_digit) {
            end = skip_while(bytes, digits, |c| c.is_ascii_digit());
        }
    }

    let text = &src[start..end];
    let value = text
        .parse()
        .map_err(|_| error(line, format!("invalid number '{text}'")))?;
    Ok((Token::Num(value), end))
}

/// Scans a name. A `]` belongs to the name only if it closes a `[` inside it,
/// so `x[0]` is one name, but in `[x * y]` the last `]` ends the quadratic term.
fn name(src: &str, start: usize) -> (Token<'_>, usize) {
    let bytes = src.as_bytes();
    let mut open = 0;
    let mut end = start;
    while let Some(&c) = bytes.get(end) {
        match c {
            b'[' => open += 1,
            b']' if open == 0 => break,
            b']' => open -= 1,
            c if c.is_ascii_alphanumeric() || NAME_EXTRA.contains(&c) => (),
            _ => break,
        }
        end += 1;
    }
    (Token::Name(&src[start..end]), end)
}

fn skip_while(bytes: &[u8], mut i: usize, keep: impl Fn(u8) -> bool) -> usize {
    while i < bytes.len() && keep(bytes[i]) {
        i += 1;
    }
    i
}

/// Skips an optional `=` after `<` or `>`.
fn skip_eq(bytes: &[u8], i: usize) -> usize {
    if bytes.get(i) == Some(&b'=') {
        i + 1
    } else {
        i
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    fn tokens(src: &str) -> Vec<Token<'_>> {
        tokenize(src)
            .unwrap()
            .into_iter()
            .map(|s| s.token)
            .collect()
    }

    #[test]
    fn numbers() {
        assert_eq!(
            tokens("2.5e-3 1E+2 .5 3."),
            [Num(2.5e-3), Num(100.0), Num(0.5), Num(3.0)]
        );
        assert_eq!(tokens("3 e"), [Num(3.0), Name("e")]);
        assert!(
            tokenize("x\n 1.2.3")
                .unwrap_err()
                .to_string()
                .contains("line 2")
        );
    }

    #[test]
    fn names() {
        assert_eq!(
            tokens("x.1 y(0) x[0,0] c1:"),
            [Name("x.1"), Name("y(0)"), Name("x[0,0]"), Name("c1"), Colon]
        );
        assert_eq!(
            tokens("[x[0] * y]"),
            [LBracket, Name("x[0]"), Star, Name("y"), RBracket]
        );
    }

    #[test]
    fn comparators() {
        use Comparator::*;
        assert_eq!(
            tokens("< <= =< > >= => = =="),
            [
                Cmp(Le),
                Cmp(Le),
                Cmp(Le),
                Cmp(Ge),
                Cmp(Ge),
                Cmp(Ge),
                Cmp(Eq),
                Cmp(Eq)
            ]
        );
    }

    #[test]
    fn comments_and_lines() {
        let spanned = tokenize("x \\ comment\n\n y").unwrap();
        assert_eq!(
            spanned,
            [
                Spanned {
                    token: Name("x"),
                    line: 1
                },
                Spanned {
                    token: Name("y"),
                    line: 3
                }
            ]
        );
    }
}
