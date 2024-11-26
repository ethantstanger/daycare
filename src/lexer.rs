use crate::errors::DaycareError;
use std::collections::binary_heap::PeekMut;
use std::f32::consts::E;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

#[derive(Debug)]
pub enum Token {
    // kids
    Kid, // add personalities
    // memory
    Imagine,
    Is,
    Tell,
    To,
    Forget,
    // hot-potato
    Pass,
    Drop,
    HotPotato,
    // toys
    Share,
    With,
    // brackets
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    // literals
    BooleanLiteral(bool),
    Identifier(String),
    NumberLiteral(f32),
    StringLiteral(String),
}

impl FromStr for Token {
    type Err = DaycareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "kid" => Ok(Token::Kid),
            "imagine" => Ok(Token::Imagine),
            "is" => Ok(Token::Is),
            "tell" => Ok(Token::Tell),
            "to" => Ok(Token::To),
            "forget" => Ok(Token::Forget),
            "pass" => Ok(Token::Pass),
            "drop" => Ok(Token::Drop),
            "hot-potato" => Ok(Token::HotPotato),
            "share" => Ok(Token::Share),
            "with" => Ok(Token::With),
            "{" => Ok(Token::OpenCurlyBracket),
            "}" => Ok(Token::CloseCurlyBracket),
            "[" => Ok(Token::OpenSquareBracket),
            "]" => Ok(Token::CloseSquareBracket),
            "true" => Ok(Token::BooleanLiteral(true)),
            "false" => Ok(Token::BooleanLiteral(false)),
            _ => Err(DaycareError::UnrecognizedToken),
        }
    }
}

pub fn lex(string: String) -> Result<Vec<Token>, DaycareError> {
    let mut peekable = string.chars().peekable();
    let mut tokens = Vec::<Token>::new();

    toss_whitespace(&mut peekable);

    loop {
        match peekable.peek() {
            Some(it) => match it {
                '{' => {
                    peekable.next();
                    tokens.push(Token::OpenCurlyBracket)
                }
                '}' => {
                    peekable.next();
                    tokens.push(Token::CloseCurlyBracket);
                }
                '[' => {
                    peekable.next();
                    tokens.push(Token::OpenSquareBracket)
                }
                ']' => {
                    peekable.next();
                    tokens.push(Token::CloseSquareBracket)
                }
                _ => tokens.push(
                    lex_word(&mut peekable)
                        .or_else(|_| lex_number(&mut peekable))
                        .or_else(|_| lex_string(&mut peekable))?,
                ),
            },
            None => break,
        }

        toss_whitespace(&mut peekable);
    }

    Ok(tokens)
}

fn lex_word(peekable: &mut Peekable<Chars>) -> Result<Token, DaycareError> {
    let peeked = peekable.peek().ok_or(DaycareError::Internal)?;
    if !(peeked.is_alphabetic() || *peeked == '-') {
        return Err(DaycareError::Internal);
    }

    let mut buffer = String::new();

    loop {
        println!("{:?}", peekable.peek());

        match peekable.peek() {
            Some(it) =>
                if it.is_alphabetic() || *it == '-' {
                    println!("Temp: {:?}", it);
                    buffer.push(*it);
                    peekable.next();
                } else {
                    break;
                },
            None => break,
        }
    }

    Token::from_str(buffer.as_str()).or(Ok(Token::Identifier(buffer)))
}

fn lex_number(peekable: &mut Peekable<Chars>) -> Result<Token, DaycareError> {
    let peeked = peekable.peek().ok_or(DaycareError::Internal)?;
    if !(peeked.is_digit(10) || *peeked == '-') {
        return Err(DaycareError::Internal);
    }

    let mut buffer = String::new();

    loop {
        match peekable.peek() {
            Some(it) =>
                if it.is_digit(10)
                    || (*it == '-' && buffer.len() == 0)
                    || (*it == '.' && !buffer.contains('.'))
                {
                    buffer.push(*it);
                    peekable.next();
                } else {
                    break;
                },
            None => break,
        }
    }

    f32::from_str(buffer.as_str())
        .and_then(|it| Ok(Token::NumberLiteral(it)))
        .or(Err(DaycareError::Internal))
}

fn lex_string(peekable: &mut Peekable<Chars>) -> Result<Token, DaycareError> {
    let peeked = peekable.peek().ok_or(DaycareError::Internal)?;
    if *peeked != '"' {
        return Err(DaycareError::Internal);
    }

    let mut buffer = String::new();
    peekable.next();

    loop {
        match peekable.next() {
            Some(it) =>
                if it != '"' {
                    buffer.push(it)
                } else {
                    break;
                },
            None => return Err(DaycareError::Internal),
        }
    }

    Ok(Token::StringLiteral(buffer))
}

fn toss_whitespace(peekable: &mut Peekable<Chars>) {
    loop {
        match peekable.peek() {
            Some(it) =>
                if it.is_whitespace() {
                    println!("{:?}", it);
                    peekable.next();
                } else {
                    break;
                },
            None => break,
        };
    }
}
