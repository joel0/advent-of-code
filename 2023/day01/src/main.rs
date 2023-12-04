use std::io::{self, BufRead};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut sum = 0;
    for line in stdin.lock().lines() {
        sum += parse_calibration_line(&line.unwrap())?;
    }

    println!("Calibration value: {sum}");
    Ok(())
}

/// Parse the "calibration value" out of a line. The calibration value is a two
/// digit number consisting of the first and last digits in the line.
fn parse_calibration_line(line: &str) -> Result<u32> {
    let first_digit = find_first_digit(line).ok_or(anyhow!("Line '{line}' contains no digits"))?;
    let last_digit = find_last_digit(line).unwrap();

    Ok(first_digit * 10 + last_digit)
}

/// Find the first digit in the line.
fn find_first_digit(line: &str) -> Option<u32> {
    line
        .chars()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
}

/// Find the last digit in the line.
fn find_last_digit(line: &str) -> Option<u32> {
    line
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
}
