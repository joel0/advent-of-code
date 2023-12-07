use std::io::BufRead;

use anyhow::{anyhow, Result, Context};

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines();
    
    let races = read_races(&mut line_iter)?;

    let mut multiplied_times = 1;
    for race in races {
        let win_range = find_number_of_winning_hold_times(&race)
            .with_context(|| format!("Error with race {race:?}"))?;
        println!("Winning range: {}", win_range);
        multiplied_times *= win_range;
    }
    println!("Multiplied winning time possibilities: {}", multiplied_times);

    Ok(())
}

/// Read the races from the file.
fn read_races(line_iter: &mut dyn Iterator<Item = std::io::Result<String>>) -> Result<Vec<Race>> {
    let time_line = line_iter
        .next()
        .ok_or_else(|| anyhow!("The file is missing the 'time' line"))?.unwrap();
    let time_line = trim_line_prefix(&time_line, "Time: ")?.trim();
    let distance_line = line_iter
        .next()
        .ok_or_else(|| anyhow!("The file is missing the 'distance' line"))?.unwrap();
    let distance_line = trim_line_prefix(&distance_line, "Distance: ")?.trim();

    let times: Vec<u32> = time_line.split_ascii_whitespace().map(|t| t.parse::<u32>()).collect::<Result<_, _>>()?;
    let distances: Vec<u32> = distance_line.split_ascii_whitespace().map(|d| d.parse()).collect::<Result<_, _>>()?;

    if times.len() != distances.len() {
        return Err(anyhow!(
            "Number of times ({}) must match the number of distances ({})",
            times.len(),
            distances.len()
        ));
    }

    let mut races = Vec::with_capacity(times.len());
    for i in 0..times.len() {
        races.push(Race::new(times[i], distances[i]));
    }
    Ok(races)
}

fn trim_line_prefix<'a>(line: &'a str, prefix: &str) -> Result<&'a str> {
    if line.starts_with(prefix) {
        Ok(&line[prefix.len()..])
    } else {
        Err(anyhow!("The line '{}' does not start with the prefix '{}'", line, prefix))
    }
}

/// Finds the range of button hold times possible to win the race.
fn find_number_of_winning_hold_times(race: &Race) -> Result<u32> {
    let min_win_hold_time = find_minimum_winning_race(race)?;
    let max_win_hold_time = find_maximum_winning_race(race)?;

    Ok(max_win_hold_time - min_win_hold_time + 1)
}

/// Finds the minimum button hold time to win the race.
fn find_minimum_winning_race(race: &Race) -> Result<u32> {
    let mut time_iter = (1..race.time - 1).into_iter();
    find_first_winning_race_iter(race, &mut time_iter)
}

/// Finds the maxmimum button hold time to win the race.
fn find_maximum_winning_race(race: &Race) -> Result<u32> {
    let mut time_iter = (1..race.time - 1).rev().into_iter();
    find_first_winning_race_iter(race, &mut time_iter)
}

/// Find the button hold time for the first winning race in the time iterator.
fn find_first_winning_race_iter(race: &Race, time_iter: &mut dyn Iterator<Item = u32>) -> Result<u32> {
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
    time: u32,
    record_distance: u32,
}

impl Race {
    fn new(time: u32, distance: u32) -> Self {
        Self { time, record_distance: distance }
    }

    /// Calculate the distance traveled for the amount of time holding the button.
    fn calculate_distance(&self, button_hold_time: u32) -> u32 {
        assert!(button_hold_time <= self.time);
        let travel_time = self.time - button_hold_time;

        // The button_hold_time is the speed.
        travel_time * button_hold_time
    }
}
