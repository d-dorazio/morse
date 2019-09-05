use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Letter {
    symbols: u8,
    len: u8,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Symbol {
    Dot = 0,
    Dash = 1,
}

pub const ASCII_DOT: &str = ".";
pub const UNICODE_DOT: &str = "\u{25cf}";
pub const ASCII_DASH: &str = "---";
pub const UNICODE_DASH: &str = "\u{2586}\u{2586}\u{2586}";
pub const SEPARATOR: char = ' ';

pub const ALPHABET: &[(u8, Letter)] = &[
    //
    // letters
    //
    (b'A', Letter::new().dot().dash()),
    (b'B', Letter::new().dash().dot().dot().dot()),
    (b'C', Letter::new().dash().dot().dash().dot()),
    (b'D', Letter::new().dash().dot().dot()),
    (b'E', Letter::new().dot()),
    (b'F', Letter::new().dot().dot().dash().dot()),
    (b'G', Letter::new().dash().dash().dot()),
    (b'H', Letter::new().dot().dot().dot().dot()),
    (b'I', Letter::new().dot().dot()),
    (b'J', Letter::new().dot().dash().dash().dash()),
    (b'K', Letter::new().dash().dot().dash()),
    (b'L', Letter::new().dot().dash().dot().dash()),
    (b'M', Letter::new().dash().dash()),
    (b'N', Letter::new().dash().dot()),
    (b'O', Letter::new().dash().dash().dash()),
    (b'P', Letter::new().dot().dash().dash().dot()),
    (b'Q', Letter::new().dash().dash().dot().dash()),
    (b'R', Letter::new().dot().dash().dot()),
    (b'S', Letter::new().dot().dot().dot()),
    (b'T', Letter::new().dash()),
    (b'U', Letter::new().dot().dot().dash()),
    (b'V', Letter::new().dot().dot().dot().dash()),
    (b'W', Letter::new().dot().dash().dash()),
    (b'X', Letter::new().dash().dot().dot().dash()),
    (b'Y', Letter::new().dash().dot().dash().dash()),
    (b'Z', Letter::new().dash().dash().dot().dot()),
    //
    // numbers
    //
    (b'1', Letter::new().dot().dash().dash().dash().dash()),
    (b'2', Letter::new().dot().dot().dash().dash().dash()),
    (b'3', Letter::new().dot().dot().dot().dash().dash()),
    (b'4', Letter::new().dot().dot().dot().dot().dash()),
    (b'5', Letter::new().dot().dot().dot().dot().dot()),
    (b'6', Letter::new().dash().dot().dot().dot().dot()),
    (b'7', Letter::new().dash().dash().dot().dot().dot()),
    (b'8', Letter::new().dash().dash().dash().dot().dot()),
    (b'9', Letter::new().dash().dash().dash().dash().dot()),
    (b'0', Letter::new().dash().dash().dash().dash().dash()),
];

lazy_static! {
    pub static ref BYTE_TO_MORSE_MAP: HashMap<u8, Letter> = ALPHABET.iter().cloned().collect();
    pub static ref MORSE_TO_BYTE_MAP: HashMap<Letter, u8> =
        ALPHABET.iter().map(|(b, l)| (l.clone(), *b)).collect();
}

impl Letter {
    pub const fn new() -> Self {
        Letter { symbols: 0, len: 0 }
    }

    pub fn iter(&self) -> impl Iterator<Item = Symbol> {
        let mut sym = self.clone();

        std::iter::from_fn(move || {
            if sym.len == 0 {
                return None;
            }

            let v = sym.symbols & 1;
            sym.symbols >>= 1;
            sym.len -= 1;

            Some(match v {
                0 => Symbol::Dot,
                1 => Symbol::Dash,
                _ => unreachable!(),
            })
        })
    }

    const fn dash(self) -> Self {
        self.and(Symbol::Dash)
    }

    const fn dot(self) -> Self {
        self.and(Symbol::Dot)
    }

    const fn and(self, s: Symbol) -> Self {
        Letter {
            symbols: self.symbols | ((s as u8) << self.len),
            len: self.len + 1,
        }
    }
}

pub mod decoder;
pub mod encoder;

pub use decoder::decode;
pub use encoder::{encode, TextEncoder};

#[cfg(test)]
mod tests {
    use crate::decoder::decode;
    use crate::encoder::{encode, TextEncoder};

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_decode_encoded(input in "[A-Z0-9]+( [A-Z0-9]+)") {
            let mut encoder = TextEncoder::ascii();

            encode(&mut encoder, &input).unwrap();
            let decoded = decode(&encoder.encoded()).unwrap();

            prop_assert_eq!(input, decoded);
        }
    }
}
