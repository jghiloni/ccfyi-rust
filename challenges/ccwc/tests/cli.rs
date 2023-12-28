use assert_cmd::prelude::*;
use std::process::{Command, Stdio};

#[test]
fn test_count_bytes() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ccwc")?;
    cmd.arg("-c").arg("testdata/test.txt");
    cmd.assert().success().stdout(predicates::str::contains("  342190 testdata/test.txt"));
    Ok(())
}

#[test]
fn test_count_lines() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ccwc")?;
    cmd.arg("-l").arg("testdata/test.txt");
    cmd.assert().success().stdout(predicates::str::contains("    7145 testdata/test.txt"));
    Ok(())
}

#[test]
fn test_count_words() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ccwc")?;
    cmd.arg("-w").arg("testdata/test.txt");
    cmd.assert().success().stdout(predicates::str::contains("   58164 testdata/test.txt"));
    Ok(())
}

#[test]
fn test_count_chars() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ccwc")?;
    cmd.arg("-m").arg("testdata/test.txt");
    cmd.assert().success().stdout(predicates::str::contains("  339292 testdata/test.txt"));
    Ok(())
}

#[test]
fn test_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let testfile = std::fs::File::open("testdata/test.txt")?;

    let mut cmd = Command::cargo_bin("ccwc")?;
    cmd.stdin(Stdio::from(testfile));
    cmd.assert().success().stdout(predicates::str::contains("    7145    58164   342190 -"));

    Ok(())
}