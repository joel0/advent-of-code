use std::collections::HashSet;
use std::io::{self, BufRead};

use anyhow::{anyhow, Result, Context};

fn main() -> Result<()> {
    let stdin = io::stdin();

    let card_iter = stdin.lock().lines();
    let total_cards = process_all_cards(&mut card_iter.into_iter())?;
    println!("Number of scratchcards: {total_cards}");

    Ok(())
}

fn process_all_cards(card_iter: &mut dyn Iterator<Item = std::io::Result<String>>) -> Result<u32> {
    let mut card_counts: Vec<u32> = vec![1];
    for (i, line) in card_iter.enumerate() {
        assert!(i <= card_counts.len(), "Card count must not go beyond the list by more than 1");
        if i >= card_counts.len() {
            card_counts.push(1);
        }
        let matches = process_card(&line.unwrap())?;
        // Win one copy of the next `matches` cards for each of the current card copy.
        for j in 0..matches {
            if i + j + 1 >= card_counts.len() {
                card_counts.push(1);
            }
            card_counts[i + j + 1] += card_counts[i];
        }
    }
    let total_cards = card_counts.iter().sum();

    Ok(total_cards)
}

/// Process the card string and return the number of matches.
fn process_card(line: &str) -> Result<usize> {
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
    Ok(matches)
}

/// Parse a string of whitespace separated numbers into a vector of numbers.
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

/// Parse a string of whitespace separated numbers into a set of numbers.
fn parse_number_set(numbers: &str) -> Result<HashSet<u32>> {
    parse_number_list(numbers).map(HashSet::from_iter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_individual_cards() {
        assert_eq!(process_card("Card   1: 1 2 3 | 4 5 6").unwrap(), 0);
        assert_eq!(process_card("Card   2: 1 2 3 | 1 5 6").unwrap(), 1);
        assert_eq!(process_card("Card   3: 1 2 3 | 1 1 1").unwrap(), 3);
        assert_eq!(process_card("Card   4: 1 1 1 | 1 2 3").unwrap(), 1);
    }

    #[test]
    fn test_cards1() {
        let mut card_iter = vec![
            Ok("Card   1: 1 2 3 | 1 2 3".to_string()), // 3 matches => 1 card
            Ok("Card   2: 1 2 3 | 1 5 6".to_string()), // 1 match   => 2 card
            Ok("Card   3: 1 2 3 | 4 5 6".to_string()), // 0 matches => 4 cards
            Ok("Card   4: 1 1 1 | 3 4 5".to_string()), // 0 matches => 2 card
            Ok("Card   5: 1 1 1 | 3 4 5".to_string()), // 0 matches => 1 card
        ]
        .into_iter();
        assert_eq!(process_all_cards(&mut card_iter).unwrap(), 10);
    }

    #[test]
    fn test_cards2() {
        let mut card_iter = vec![
            Ok("Card   1: 1 2 3 4 | 1 2 3 4".to_string()), // 4 matches
            Ok("Card   2: 1 2 3 4 | 1 2 5 6".to_string()), // 2 matches
            Ok("Card   3: 1 2 3 4 | 1 2 5 6".to_string()), // 2 matches
            Ok("Card   4: 1 2 3 4 | 1 5 6 7".to_string()), // 1 match
            Ok("Card   5: 1 2 3 4 | 5 6 7 8".to_string()), // 0 matches
            Ok("Card   6: 1 2 3 4 | 5 6 7 8".to_string()), // 0 matches
        ]
        .into_iter();
        assert_eq!(process_all_cards(&mut card_iter).unwrap(), 30);
    }

    #[test]
    fn test_cards_edgecase() {
        let mut card_iter = vec![
            Ok("Card   1: 1 2 3 | 4 5 6".to_string()), // 0 matches => 1 card
        ]
        .into_iter();
        assert_eq!(process_all_cards(&mut card_iter).unwrap(), 1);
    }
}
