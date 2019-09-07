use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

use structopt::StructOpt;

/// Morse is a simple application to play around with morse code. It allows to encode, decode and
/// play morse code.
#[derive(Debug, StructOpt)]
pub enum Opt {
    /// Encode the input in morse code.
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

    /// Decode the morse code.
    #[structopt(name = "decode")]
    Decode {
        /// The input text to encode.
        #[structopt(name = "FILE", parse(from_os_str))]
        input: Option<PathBuf>,
    },

    /// Encode and play the input.
    #[structopt(name = "play")]
    Play {
        /// Speed of the audio, chain the option to go faster and faster like `-fff`.
        #[structopt(short = "f", long = "fast", parse(from_occurrences))]
        speed: u8,

        /// The frequency in Hz to beep at. For example, 440Hz is the frequency of the musical note
        /// A.
        #[structopt(long, default_value = "220")]
        frequency: f32,

        /// The input text to encode and play.
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
        Opt::Play {
            input: None,
            speed,
            frequency,
        } => play(&mut io::stdin(), speed, frequency),
        Opt::Play {
            input: Some(input),
            speed,
            frequency,
        } => play(&mut File::open(input)?, speed, frequency),
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

fn play(input: &mut impl io::Read, speed: u8, frequency: f32) -> io::Result<()> {
    let samples_per_symbol = morse::AudioSource::FRAME_RATE / 2_u32.pow(1 + u32::from(speed));
    let mut source = morse::AudioSource::new(frequency, samples_per_symbol);

    let input = io::BufReader::new(input);

    for line in input.lines() {
        let line = line?;

        if morse::encode(&mut source, &line).is_none() {
            println!("cannot encode this line because of not recognized characters");
        }
    }

    let device = rodio::default_output_device()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no audio output device found"))?;

    let sink = rodio::Sink::new(&device);
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
