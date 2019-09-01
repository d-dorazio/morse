use std::io;
use std::io::BufRead;

use morse::{MorseEncoder, MorseEncoding};

fn main() -> std::io::Result<()> {
    let encoder = MorseEncoder::new();

    let input = io::BufReader::new(io::stdin());
    for line in input.lines() {
        let line = line?;

        let encoded = match encoder.encode(&line, MorseEncoding::Unicode) {
            None => {
                println!("cannot encode this line because of not recognized characters");
                continue;
            }
            Some(e) => e,
        };

        println!("{}", encoded);
    }

    Ok(())
}
