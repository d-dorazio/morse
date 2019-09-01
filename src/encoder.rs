use std::collections::HashMap;

use crate::{Morse, MorseSymbol, ALPHABET};

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
            MorseEncoding::Ascii => ("--- ", ". "),
            MorseEncoding::Unicode => ("\u{2586}\u{2586}\u{2586} ", "\u{25cf} "),
        };

        for word in corpus.split_whitespace() {
            let encoded_word = word.as_bytes().iter().map(|b| {
                let symbols = self.encoding_map.get(&b.to_ascii_uppercase())?;

                let letters = symbols
                    .iter()
                    .flat_map(|s| match s {
                        MorseSymbol::Dash => dash.chars(),
                        MorseSymbol::Dot => dot.chars(),
                    })
                    .chain("  ".chars());

                Some(letters)
            });

            for symbols in encoded_word {
                let symbols = symbols?;
                morsed.extend(symbols);
                morsed.push_str("       ");
            }
        }

        // remove final whitespaces
        for _ in 0..10 {
            morsed.pop();
        }

        Some(morsed)
    }
}
