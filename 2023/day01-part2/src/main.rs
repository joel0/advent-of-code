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

/// Find the first digit in the line, either numeric or spelled out.
fn find_first_digit(line: &str) -> Option<u32> {
    for i in 0..line.len() {
        if let Some(digit) = parse_starts_with_digit(&line[i..]) {
            return Some(digit);
        }
    }

    None
}

/// Find the last digit in the line, either numeric or spelled out.
fn find_last_digit(line: &str) -> Option<u32> {
    for i in (0..line.len()).rev() {
        if let Some(digit) = parse_starts_with_digit(&line[i..]) {
            return Some(digit);
        }
    }

    None
}

/// Parse either a numeric or spelled out digit from the start of the string.
fn parse_starts_with_digit(s: &str) -> Option<u32> {
    const DIGITS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    assert!(!s.is_empty(), "parse_starts_with_digit called with empty string");
    let c = s.chars().next().unwrap();
    if c.is_ascii_digit() {
        return c.to_digit(10);
    }

    for (i, digit) in DIGITS.iter().enumerate() {
        if s.starts_with(digit) {
            return Some(i as u32);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_first_num() {
        assert_eq!(find_first_digit("69"), Some(6));
        assert_eq!(find_first_digit("foo4"), Some(4));
        assert_eq!(find_first_digit(""), None);
        assert_eq!(find_first_digit("foo"), None);
    }

    #[test]
    fn test_parse_first_str() {
        assert_eq!(find_first_digit("sixnine"), Some(6));
        assert_eq!(find_first_digit("foofour"), Some(4));
        assert_eq!(find_first_digit("four20"), Some(4));
    }

    #[test]
    fn test_parse_last_num() {
        assert_eq!(find_last_digit("69"), Some(9));
        assert_eq!(find_last_digit("foo4"), Some(4));
        assert_eq!(find_last_digit(""), None);
        assert_eq!(find_last_digit("foo"), None);
    }

    #[test]
    fn test_parse_last_str() {
        assert_eq!(find_last_digit("sixnine"), Some(9));
        assert_eq!(find_last_digit("foofour"), Some(4));
        assert_eq!(find_last_digit("42zero"), Some(0));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_calibration_line("sixnine").unwrap(), 69);
        assert_eq!(parse_calibration_line("foofour").unwrap(), 44);
        assert_eq!(parse_calibration_line("42zero").unwrap(), 40);
    }
}
