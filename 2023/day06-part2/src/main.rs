use std::io::BufRead;

use anyhow::{anyhow, Result, Context};

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines();
    
    let race = read_race(&mut line_iter)?;

    let win_range = find_number_of_winning_hold_times(&race)
        .with_context(|| format!("Error with race {race:?}"))?;
    println!("Winning range: {}", win_range);

    Ok(())
}

/// Read the race from the file, ignoring whitespace between numbers.
fn read_race(line_iter: &mut dyn Iterator<Item = std::io::Result<String>>) -> Result<Race> {
    let time_line = line_iter
        .next()
        .ok_or_else(|| anyhow!("The file is missing the 'time' line"))?.unwrap();
    let time_line = trim_line_prefix(&time_line, "Time: ")?.trim();
    let distance_line = line_iter
        .next()
        .ok_or_else(|| anyhow!("The file is missing the 'distance' line"))?.unwrap();
    let distance_line = trim_line_prefix(&distance_line, "Distance: ")?.trim();

    let time: u64 = time_line.replace(" ", "").parse().context("Error parsing time")?;
    let distance: u64 = distance_line.replace(" ", "").parse().context("Error parsing distance")?;

    Ok(Race::new(time, distance))
}

fn trim_line_prefix<'a>(line: &'a str, prefix: &str) -> Result<&'a str> {
    if line.starts_with(prefix) {
        Ok(&line[prefix.len()..])
    } else {
        Err(anyhow!("The line '{}' does not start with the prefix '{}'", line, prefix))
    }
}

/// Finds the range of button hold times possible to win the race.
fn find_number_of_winning_hold_times(race: &Race) -> Result<u64> {
    let min_win_hold_time = find_minimum_winning_race(race)?;
    let max_win_hold_time = find_maximum_winning_race(race)?;

    Ok(max_win_hold_time - min_win_hold_time + 1)
}

/// Finds the minimum button hold time to win the race.
fn find_minimum_winning_race(race: &Race) -> Result<u64> {
    let mut time_iter = (1..race.time - 1).into_iter();
    find_first_winning_race_iter(race, &mut time_iter)
}

/// Finds the maxmimum button hold time to win the race.
fn find_maximum_winning_race(race: &Race) -> Result<u64> {
    let mut time_iter = (1..race.time - 1).rev().into_iter();
    find_first_winning_race_iter(race, &mut time_iter)
}

/// Find the button hold time for the first winning race in the time iterator.
fn find_first_winning_race_iter(race: &Race, time_iter: &mut dyn Iterator<Item = u64>) -> Result<u64> {
    let winning_distance = race.record_distance;
    for button_hold_time in time_iter {
        let distance = race.calculate_distance(button_hold_time);
        if distance > winning_distance {
            return Ok(button_hold_time);
        }
    }
    
    Err(anyhow!("There's no way to win this race. {race:?}"))
}

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, record_distance: distance }
    }

    /// Calculate the distance traveled for the amount of time holding the button.
    fn calculate_distance(&self, button_hold_time: u64) -> u64 {
        assert!(button_hold_time <= self.time);
        let travel_time = self.time - button_hold_time;

        // The button_hold_time is the speed.
        travel_time * button_hold_time
    }
}
