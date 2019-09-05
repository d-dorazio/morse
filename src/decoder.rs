use std::fmt::{self, Display};

use itertools::Itertools;

use crate::{Letter, Symbol, ASCII_DASH, ASCII_DOT, MORSE_TO_BYTE_MAP, UNICODE_DASH, UNICODE_DOT};

#[derive(Debug)]
pub enum MorseDecoderError {
    UnknownSymbol(String),
    UnexpectedToken(String),
    UnknownMorse(Letter),
    EOF,
}

pub fn decode(input: &str) -> Result<String, MorseDecoderError> {
    use MorseDecoderError::*;

    let mut out = String::new();

    let tokens = input.chars().group_by(|e| *e);
    let mut tokens = tokens.into_iter();

    let mut next_token = || {
        let (_tok_id, mut group) = tokens.next().ok_or(EOF)?;
        let tok = group.by_ref().take(7).collect::<String>();

        // must be empty at this point since the biggest token is the word separator which is 7
        // characters
        if group.next().is_some() {
            return Err(UnknownSymbol(tok));
        }

        Ok(tok)
    };
    let convert_morse = |m| {
        MORSE_TO_BYTE_MAP
            .get(&m)
            .ok_or_else(|| UnknownMorse(m))
            .map(|b| char::from(*b))
    };

    let mut current = Letter::new();
    loop {
        let tok = next_token()?;

        if tok == UNICODE_DASH || tok == ASCII_DASH {
            current = current.dash();
        } else if tok == UNICODE_DOT || tok == ASCII_DOT {
            current = current.dot();
        } else {
            return Err(UnexpectedToken(tok));
        }

        // eat whitespace
        match next_token() {
            Ok(tok) => {
                // another symbol pending
                if tok == " " {
                    continue;
                }

                // end of symbol or word
                if tok == "   " || tok == "       " {
                    out.push(convert_morse(current)?);

                    // if end of word add a space
                    if tok == "       " {
                        out.push(' ');
                    }

                    current = Letter::new();
                    continue;
                }

                return Err(UnexpectedToken(tok));
            }
            Err(EOF) => {
                out.push(convert_morse(current)?);
                out.push(' ');

                break;
            }
            Err(e) => return Err(e),
        }
    }

    // remove last whitespace
    out.pop();

    Ok(out)
}

impl std::error::Error for MorseDecoderError {}
impl Display for MorseDecoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MorseDecoderError::EOF => write!(f, "unexpected end of input"),
            MorseDecoderError::UnknownSymbol(s) => write!(f, "unknown symbol {}", s),
            MorseDecoderError::UnexpectedToken(t) => write!(f, "unexpected token {}", t),
            MorseDecoderError::UnknownMorse(m) => {
                let repr = m
                    .iter()
                    .map(|s| match s {
                        Symbol::Dash => '-',
                        Symbol::Dot => '.',
                    })
                    .collect::<String>();

                write!(f, "unknown morse {}", repr)
            }
        }
    }
}
