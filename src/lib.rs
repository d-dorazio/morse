#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Morse {
    symbols: u8,
    len: u8,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MorseSymbol {
    Dot = 0,
    Dash = 1,
}

pub const ASCII_DOT: &str = ".";
pub const UNICODE_DOT: &str = "\u{25cf}";
pub const ASCII_DASH: &str = "---";
pub const UNICODE_DASH: &str = "\u{2586}\u{2586}\u{2586}";
pub const SEPARATOR: char = ' ';

pub const ALPHABET: &[(u8, Morse)] = &[
    //
    // letters
    //
    (b'A', Morse::new().dot().dash()),
    (b'B', Morse::new().dash().dot().dot().dot()),
    (b'C', Morse::new().dash().dot().dash().dot()),
    (b'D', Morse::new().dash().dot().dot()),
    (b'E', Morse::new().dot()),
    (b'F', Morse::new().dot().dot().dash().dot()),
    (b'G', Morse::new().dash().dash().dot()),
    (b'H', Morse::new().dot().dot().dot().dot()),
    (b'I', Morse::new().dot().dot()),
    (b'J', Morse::new().dot().dash().dash().dash()),
    (b'K', Morse::new().dash().dot().dash()),
    (b'L', Morse::new().dot().dash().dot().dash()),
    (b'M', Morse::new().dash().dash()),
    (b'N', Morse::new().dash().dot()),
    (b'O', Morse::new().dash().dash().dash()),
    (b'P', Morse::new().dot().dash().dash().dot()),
    (b'Q', Morse::new().dash().dash().dot().dash()),
    (b'R', Morse::new().dot().dash().dot()),
    (b'S', Morse::new().dot().dot().dot()),
    (b'T', Morse::new().dash()),
    (b'U', Morse::new().dot().dot().dash()),
    (b'V', Morse::new().dot().dot().dot().dash()),
    (b'W', Morse::new().dot().dash().dash()),
    (b'X', Morse::new().dash().dot().dot().dash()),
    (b'Y', Morse::new().dash().dot().dash().dash()),
    (b'Z', Morse::new().dash().dash().dot().dot()),
    //
    // numbers
    //
    (b'1', Morse::new().dot().dash().dash().dash().dash()),
    (b'2', Morse::new().dot().dot().dash().dash().dash()),
    (b'3', Morse::new().dot().dot().dot().dash().dash()),
    (b'4', Morse::new().dot().dot().dot().dot().dash()),
    (b'5', Morse::new().dot().dot().dot().dot().dot()),
    (b'6', Morse::new().dash().dot().dot().dot().dot()),
    (b'7', Morse::new().dash().dash().dot().dot().dot()),
    (b'8', Morse::new().dash().dash().dash().dot().dot()),
    (b'9', Morse::new().dash().dash().dash().dash().dot()),
    (b'0', Morse::new().dash().dash().dash().dash().dash()),
];

impl Morse {
    pub const fn new() -> Self {
        Morse { symbols: 0, len: 0 }
    }

    pub fn iter(&self) -> impl Iterator<Item = MorseSymbol> {
        let mut sym = self.clone();

        std::iter::from_fn(move || {
            if sym.len == 0 {
                return None;
            }

            let v = sym.symbols & 1;
            sym.symbols >>= 1;
            sym.len -= 1;

            Some(match v {
                0 => MorseSymbol::Dot,
                1 => MorseSymbol::Dash,
                _ => unreachable!(),
            })
        })
    }

    const fn dash(self) -> Self {
        self.and(MorseSymbol::Dash)
    }

    const fn dot(self) -> Self {
        self.and(MorseSymbol::Dot)
    }

    const fn and(self, s: MorseSymbol) -> Self {
        Morse {
            symbols: self.symbols | ((s as u8) << self.len),
            len: self.len + 1,
        }
    }
}

pub mod decoder;
pub mod encoder;

pub use decoder::MorseDecoder;
pub use encoder::{MorseEncoder, MorseEncoding};

#[cfg(test)]
mod tests {
    use super::{MorseDecoder, MorseEncoder, MorseEncoding};

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn prop_decode_encoded(input in "[A-Z0-9]+( [A-Z0-9]+)") {
            let encoder = MorseEncoder::new();
            let decoder = MorseDecoder::new();

            let encoded = encoder.encode(&input, MorseEncoding::Ascii).unwrap();
            let decoded = decoder.decode(&encoded).unwrap();

            prop_assert_eq!(input, decoded);
        }
    }
}
