use std::collections::HashSet;
use std::io::{self, BufRead};

use anyhow::{anyhow, Result, Context};

fn main() -> Result<()> {
    let stdin = io::stdin();

    let mut score: u32 = 0;
    for line in stdin.lock().lines() {
        score += process_card(&line.unwrap())?;
    }
    println!("Total score: {score}");

    Ok(())
}

fn process_card(line: &str) -> Result<u32> {
    if line.len() < 10 {
        return Err(anyhow!("Line '{line}' is missing the 'Card   #: ' prefix"));
    }
    // Trim off the "Card #: " prefix.
    let trimmed_line = &line[10..];

    let (winning_str, have_str) = trimmed_line
        .split_once('|')
        .ok_or_else(|| anyhow!("Line '{line}' is missing a '|'"))?;

    let winning = parse_number_set(winning_str)
        .with_context(|| format!("Line '{line}' winning numbers error"))?;
    let have = parse_number_list(have_str)
        .with_context(|| format!("Line '{line}' numbers you have error"))?;

    let mut matches: usize = 0;
    for number in have {
        if winning.contains(&number) {
            matches += 1;
        }
    }
    if matches == 0 {
        Ok(0)
    } else {
        Ok(1 << (matches - 1))
    }
}

fn parse_number_list(numbers: &str) -> Result<Vec<u32>> {
    let mut vec = Vec::new();
    for number in numbers.split_ascii_whitespace() {
        let number = number
            .parse()
            .with_context(|| format!("invalid number '{number}"))?;
        vec.push(number);
    }

    Ok(vec)
}

fn parse_number_set(numbers: &str) -> Result<HashSet<u32>> {
    parse_number_list(numbers).map(HashSet::from_iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cards() {
        assert_eq!(process_card("Card   1: 1 2 3 | 4 5 6").unwrap(), 0);
        assert_eq!(process_card("Card   2: 1 2 3 | 1 5 6").unwrap(), 1);
        assert_eq!(process_card("Card   3: 1 2 3 | 1 1 1").unwrap(), 4);
        assert_eq!(process_card("Card   4: 1 1 1 | 1 2 3").unwrap(), 1);
    }
}
