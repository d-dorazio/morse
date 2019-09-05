use crate::{Symbol, ASCII_DASH, ASCII_DOT, BYTE_TO_MORSE_MAP, UNICODE_DASH, UNICODE_DOT};

pub trait Encoder {
    fn dash(&mut self);
    fn dot(&mut self);
    fn space(&mut self);
    fn pop(&mut self);
}

pub fn encode(encoder: &mut impl Encoder, corpus: &str) -> Option<()> {
    for word in corpus.split_whitespace() {
        for b in word.as_bytes() {
            let symbols = BYTE_TO_MORSE_MAP.get(&b.to_ascii_uppercase())?;

            for s in symbols.iter() {
                match s {
                    Symbol::Dash => encoder.dash(),
                    Symbol::Dot => encoder.dot(),
                }

                encoder.space();
            }

            // the letter separator is made of 3 spaces, but we've already inserted 1 as part
            // of the last symbol, only 2 left to insert
            encoder.space();
            encoder.space();
        }

        // the word separator is 7 units, but we already inserted 3 as part of the last
        // word, only 4 left to insert
        for _ in 0..4 {
            encoder.space();
        }
    }

    // remove word whitespaces
    for _ in 0..7 {
        encoder.pop();
    }

    Some(())
}

pub struct TextEncoder {
    dot: &'static str,
    dash: &'static str,
    space: &'static str,

    encoded_text: String,
}

impl TextEncoder {
    pub fn ascii() -> Self {
        TextEncoder {
            dot: ASCII_DOT,
            dash: ASCII_DASH,
            space: " ",
            encoded_text: String::new(),
        }
    }

    pub fn unicode() -> Self {
        TextEncoder {
            dot: UNICODE_DOT,
            dash: UNICODE_DASH,
            space: " ",
            encoded_text: String::new(),
        }
    }

    pub fn encoded(&self) -> &str {
        &self.encoded_text
    }

    pub fn clear(&mut self) {
        self.encoded_text.clear()
    }
}

impl Encoder for TextEncoder {
    fn dash(&mut self) {
        self.encoded_text.push_str(self.dash);
    }

    fn dot(&mut self) {
        self.encoded_text.push_str(self.dot);
    }

    fn space(&mut self) {
        self.encoded_text.push_str(self.space);
    }

    fn pop(&mut self) {
        self.encoded_text.pop();
    }
}
