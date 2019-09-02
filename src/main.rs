use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

use morse::{MorseEncoder, MorseEncoding};

use structopt::StructOpt;

/// Morse is a simple application to convert text into morse code.
#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Encoding of the output representation of the morse code. Can be either "ascii" or
    /// "unicode".
    #[structopt(short, long, parse(try_from_str = encoding_from_str), default_value = "ascii")]
    encoding: MorseEncoding,

    /// The input text to encode.
    #[structopt(name = "FILE", parse(from_os_str))]
    input: Option<PathBuf>,
}

fn encoding_from_str(input: &str) -> Result<MorseEncoding, String> {
    match input {
        "a" | "ascii" => Ok(MorseEncoding::Ascii),
        "u" | "unicode" => Ok(MorseEncoding::Unicode),
        enc => Err(format!("unrecognized encoding {}", enc)),
    }
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    match opt.input {
        None => run(&mut io::stdin(), opt.encoding),
        Some(path) => run(&mut File::open(path)?, opt.encoding),
    }
}

fn run(input: &mut impl io::Read, encoding: MorseEncoding) -> io::Result<()> {
    let encoder = MorseEncoder::new();

    let input = io::BufReader::new(input);

    for line in input.lines() {
        let line = line?;

        let encoded = match encoder.encode(&line, encoding) {
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
