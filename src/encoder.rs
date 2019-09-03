use std::collections::HashMap;
use std::iter;

use crate::{
    Morse, MorseSymbol, ALPHABET, ASCII_DASH, ASCII_DOT, SEPARATOR, UNICODE_DASH, UNICODE_DOT,
};

pub struct MorseEncoder {
    encoding_map: HashMap<u8, Morse>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorseEncoding {
    Ascii,
    Unicode,
}

impl MorseEncoder {
    pub fn new() -> Self {
        MorseEncoder {
            encoding_map: ALPHABET.iter().cloned().collect(),
        }
    }

    pub fn encode(&self, corpus: &str, mode: MorseEncoding) -> Option<String> {
        let mut morsed = String::new();

        let (dash, dot) = match mode {
            MorseEncoding::Ascii => (ASCII_DASH, ASCII_DOT),
            MorseEncoding::Unicode => (UNICODE_DASH, UNICODE_DOT),
        };
        let dash = format!("{}{}", dash, SEPARATOR);
        let dot = format!("{}{}", dot, SEPARATOR);

        for word in corpus.split_whitespace() {
            let encoded_word = word.as_bytes().iter().map(|b| {
                let symbols = self.encoding_map.get(&b.to_ascii_uppercase())?;

                let letters = symbols.iter().flat_map(|s| match s {
                    MorseSymbol::Dash => dash.chars(),
                    MorseSymbol::Dot => dot.chars(),
                });

                // the letter separator is made of 3 spaces, but we've already inserted 1 as part
                // of the last symbol, only 2 left to insert
                let letters = letters.chain(iter::repeat(SEPARATOR).take(2));

                Some(letters)
            });

            for symbols in encoded_word {
                morsed.extend(symbols?);
            }

            // the word separator is 7 units, but we already inserted 3 as part of the last
            // word, only 4 left to insert
            morsed.extend(iter::repeat(SEPARATOR).take(4));
        }

        // remove word whitespaces
        for _ in 0..7 {
            morsed.pop();
        }

        Some(morsed)
    }
}
