use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Token {
    Fn,
    Var,
    Struct,
    If,
    Else,
    While,
    For,
    Return,
    Identifier(String),
    IntegerLiteral(i32),
    FloatLiteral(f32),
    CharLiteral(char),
    StringLiteral(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    Semicolon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Exclamation,
    ExclamationEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AndAnd,
    OrOr,
    Question,
    Dot,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            'a'..='z' | 'A'..='Z' => tokens.push(tokenize_identifier(&mut chars)),
            '0'..='9' => tokens.push(tokenize_number(&mut chars)),
            '"' => tokens.push(tokenize_string(&mut chars)),
            '\'' => tokens.push(tokenize_char(&mut chars)),
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '{' => {
                tokens.push(Token::LBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RBrace);
                chars.next();
            }
            '[' => {
                tokens.push(Token::LBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RBracket);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
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
                tokens.push(Token::Asterisk);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '%' => {
                tokens.push(Token::Percent);
                chars.next();
            }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::EqualEqual);
                    chars.next();
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::ExclamationEqual);
                    chars.next();
                } else {
                    tokens.push(Token::Exclamation);
                }
            }
            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::LessEqual);
                    chars.next();
                } else {
                    tokens.push(Token::Less);
                }
            }
            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    tokens.push(Token::GreaterEqual);
                    chars.next();
                } else {
                    tokens.push(Token::Greater);
                }
            }
            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    tokens.push(Token::AndAnd);
                    chars.next();
                } else {
                    panic!("Unexpected character: &");
                }
            }
            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    tokens.push(Token::OrOr);
                    chars.next();
                } else {
                    panic!("Unexpected character: |");
                }
            }
            '?' => {
                tokens.push(Token::Question);
                chars.next();
            }
            '.' => {
                tokens.push(Token::Dot);
                chars.next();
            }
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }

    tokens
}

fn tokenize_identifier(chars: &mut Peekable<Chars>) -> Token {
    let mut identifier = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_alphabetic() || ch.is_ascii_digit() || ch == '_' {
            identifier.push(ch);
            chars.next();
        } else {
            break;
        }
    }

    match identifier.as_str() {
        "fn" => Token::Fn,
        "var" => Token::Var,
        "struct" => Token::Struct,
        "if" => Token::If,
        "else" => Token::Else,
        "while" => Token::While,
        "for" => Token::For,
        "return" => Token::Return,
        _ => Token::Identifier(identifier),
    }
}

fn tokenize_number(chars: &mut Peekable<Chars>) -> Token {
    let mut number = String::new();
    let mut is_float = false;
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            number.push(ch);
            chars.next();
        } else if ch == '.' {
            number.push(ch);
            chars.next();
            is_float = true;
        } else {
            break;
        }
    }

    if is_float {
        Token::FloatLiteral(number.parse::<f32>().unwrap())
    } else {
        Token::IntegerLiteral(number.parse::<i32>().unwrap())
    }
}

fn tokenize_string(chars: &mut Peekable<Chars>) -> Token {
    let mut string = String::new();
    chars.next(); // Skip opening quote
    while let Some(&ch) = chars.peek() {
        if ch == '"' {
            chars.next(); // Skip closing quote
            break;
        } else {
            string.push(ch);
            chars.next();
        }
    }

    Token::StringLiteral(string)
}

fn tokenize_char(chars: &mut Peekable<Chars>) -> Token {
    chars.next(); // Skip opening quote
    let ch = chars.next().unwrap();
    let next_char = chars.next();
    if next_char == Some('\'') {
        Token::CharLiteral(ch)
    } else {
        panic!("Invalid character literal");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "fn main() int { var x: int = 10; }";
        let tokens = tokenize(input);

        let expected_tokens = vec![
            Token::Fn,
            Token::Identifier("main".to_string()),
            Token::LParen,
            Token::RParen,
            Token::Identifier("int".to_string()),
            Token::LBrace,
            Token::Var,
            Token::Identifier("x".to_string()),
            Token::Colon,
            Token::Identifier("int".to_string()),
            Token::Equal,
            Token::IntegerLiteral(10),
            Token::Semicolon,
            Token::RBrace,
        ];

        assert_eq!(tokens, expected_tokens);
    }
}
