use std::io::{BufRead, BufReader, Read, Result};

pub(crate) fn count_bytes(raw: &mut dyn BufRead) -> Result<usize> {
    let mut reader = BufReader::new(raw);

    let mut buffer = [0; 1048576]; // 1048576 = 1 Mibibyte
    let mut count = 0;

    let mut read_result = reader.read(&mut buffer[..])?;
    while read_result > 0 {
        count += read_result;
        read_result = reader.read(&mut buffer[..])?;
    }

    Ok(count)
}

pub(crate) fn count_lines(raw: &mut dyn BufRead) -> Result<usize> {
    let mut reader = BufReader::new(raw);
    let mut count: usize = 0;

    let mut line = String::new();
    while let Ok(1..) = reader.read_line(&mut line) {
        count += 1;
        line.clear();
    }

    Ok(count)
}
