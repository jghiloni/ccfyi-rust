use clap::Parser;
use counters::*;
use std::fs::File;
use std::io::{self, stdin, BufRead, BufReader, Error, ErrorKind, Read, Cursor};


mod counters;

/// Count things!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CCWC {
    /// Count bytes
    #[arg(short = 'c', default_value_t = false)]
    count_bytes: bool,

    /// Count lines
    #[arg(short = 'l', default_value_t = false)]
    count_lines: bool,

    /// Count words
    #[arg(short = 'w', default_value_t = false)]
    count_words: bool,

    /// Count characters
    #[arg(short = 'm', default_value_t = false)]
    count_chars: bool,

    /// The file to count. Use stdin if omitted
    stream_source: Option<String>,
}

fn main() -> io::Result<()> {
    let cli = CCWC::parse();

    let stream_string = cli.stream_source.unwrap_or("-".to_string());
    let stream_source = stream_string.as_str();

    let stdin = stdin();
    let mut stdin_lock = stdin.lock();

    let mut file_buffer: BufReader<File>;

    let reader: &mut dyn BufRead = if stream_source == "-" {
        &mut stdin_lock
    } else {
        let f = File::open(stream_source);
        file_buffer = BufReader::new(f.unwrap());
        &mut file_buffer
    };

    let result = if cli.count_bytes {
        count_bytes(reader)
    } else if cli.count_lines {
        count_lines(reader)
    } else if cli.count_words {
        count_words(reader)
    } else if cli.count_chars {
        count_chars(reader)
    } else {
        Err(Error::new(ErrorKind::InvalidInput, ""))
    };

    match result {
        Ok(n) => {
            println!("{:>8} {}", n, stream_source);
            Ok(())
        },
        Err(_) => {
            let mut internal_buffer = Vec::<u8>::new();
            let mut buffer = BufReader::new(reader);
            let _ = buffer.read_to_end(&mut internal_buffer);

            let mut cursor = Cursor::new(internal_buffer);
            let lines = count_lines(&mut cursor);
            
            cursor.set_position(0);
            let words = count_words(&mut cursor);

            cursor.set_position(0);
            let bytes = count_bytes(&mut cursor);

            println!("{:>8} {:>8} {:>8} {}", lines.unwrap(), words.unwrap(), bytes.unwrap(), stream_source);
            Ok(())
        }
    }
}
