use clap::Parser;
use counters::{count_bytes, count_lines};
use std::fs::File;
use std::io::{self, stdin, BufRead, BufReader, Error, ErrorKind};

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

    /// The file to count. Pass "-" or omit to use standard in
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
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "one of [-c] must be specified!"))
    };

    println!("{:>8} {}", result.unwrap(), stream_source);
    Ok(())

}
