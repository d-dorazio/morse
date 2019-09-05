use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

use structopt::StructOpt;

/// Morse is a simple application to convert text into morse code.
#[derive(Debug, StructOpt)]
pub enum Opt {
    #[structopt(name = "encode")]
    Encode {
        /// Encoding of the output representation of the morse code. Can be either "ascii" or
        /// "unicode".
        #[structopt(short, long, parse(try_from_str = encoding_from_str), default_value = "ascii")]
        encoding: Encoding,

        /// The input text to encode.
        #[structopt(name = "FILE", parse(from_os_str))]
        input: Option<PathBuf>,
    },

    #[structopt(name = "decode")]
    Decode {
        /// The input text to encode.
        #[structopt(name = "FILE", parse(from_os_str))]
        input: Option<PathBuf>,
    },
}

#[derive(Debug)]
pub enum Encoding {
    Ascii,
    Unicode,
}

fn encoding_from_str(input: &str) -> Result<Encoding, String> {
    match input {
        "a" | "ascii" => Ok(Encoding::Ascii),
        "u" | "unicode" => Ok(Encoding::Unicode),
        enc => Err(format!("unrecognized encoding {}", enc)),
    }
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Encode {
            encoding,
            input: None,
        } => encode(&mut io::stdin(), encoding),
        Opt::Encode {
            encoding,
            input: Some(input),
        } => encode(&mut File::open(input)?, encoding),
        Opt::Decode { input: None } => decode(&mut io::stdin()),
        Opt::Decode { input: Some(input) } => decode(&mut File::open(input)?),
    }
}

fn encode(input: &mut impl io::Read, encoding: Encoding) -> io::Result<()> {
    let mut encoder = match encoding {
        Encoding::Ascii => morse::TextEncoder::ascii(),
        Encoding::Unicode => morse::TextEncoder::unicode(),
    };

    let input = io::BufReader::new(input);

    for line in input.lines() {
        let line = line?;

        match morse::encode(&mut encoder, &line) {
            None => println!("cannot encode this line because of not recognized characters"),
            Some(()) => println!("{}", encoder.encoded()),
        };

        encoder.clear();
    }

    Ok(())
}

fn decode(input: &mut impl io::Read) -> io::Result<()> {
    let input = io::BufReader::new(input);

    for line in input.lines() {
        let line = line?;

        match morse::decode(&line) {
            Err(err) => println!("error: {}", err),
            Ok(decoded) => println!("{}", decoded),
        };
    }

    Ok(())
}
