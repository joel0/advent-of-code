use std::collections::HashMap;
use std::ops::Range;
use std::io::{self, BufRead};

use anyhow::{anyhow, Result, Context};

struct Mappings (HashMap<String, Map>);

struct Map {
    source: String,
    destination: String,
    mappings: Vec<Mapping>,
}

#[derive(Debug, PartialEq)]
struct Mapping {
    source: Range<u64>,
    destination_start: u64,
}

impl Mappings {
    /// Traverse all the maps to find the location for the given seed.
    fn lookup_seed_location(&self, seed: u64) -> Result<u64> {
        let mut key = "seed".to_string();
        let mut value = seed;
        loop {
            let map = self.0.get(&key)
                .ok_or_else(|| anyhow!("No map for source '{key}'"))?;
            key = map.destination.to_owned();
            value = map.lookup(value);
            if key == "location" {
                return Ok(value);
            }
        }
    }
}

impl Map {
    fn lookup(&self, source: u64) -> u64 {
        let mapping = self.mappings.iter()
            .find(|mapping| mapping.source.contains(&source));
        match mapping {
            Some(mapping) => mapping.destination_start + (source - mapping.source.start),
            None => source,
        }
    }
}

impl Mapping {
    fn from_str(s: &str) -> Result<Mapping> {
        let parts: Vec<&str> = s.split_ascii_whitespace().collect();
        if parts.len() != 3 {
            return Err(anyhow!("Mapping must have 3 parts"));
        }

        let destination_start: u64 = parts[0]
            .parse()
            .with_context(|| format!("Mapping destination '{}' must be a number", parts[0]))?;
        let source_start: u64 = parts[1]
            .parse()
            .with_context(|| format!("Mapping source start '{}' must be a number", parts[1]))?;
        let length: u64 = parts[2]
            .parse()
            .with_context(|| format!("Mapping length '{}' must be a number", parts[2]))?;

        Ok(Mapping {
            source: source_start..source_start + length,
            destination_start,
        })
    }
}

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let mut line_iter = stdin.lock().lines();

    let seeds = read_seeds(&line_iter.next().unwrap().unwrap())
        .context("Error reading seeds")?;
    let line = line_iter.next().unwrap().unwrap();
    assert_eq!(line, "", "Expected blank line after seeds");

    let maps = read_all_maps(&mut line_iter)?;

    let smallest = find_seed_with_smallest_location(seeds, &maps)?;
    println!("Seed with smallest location: {}", smallest);
    println!("Smallest location: {}", maps.lookup_seed_location(smallest).unwrap());

    Ok(())
}

/// From all the given seeds, lookup the locations to find the one with the smallest location.
fn find_seed_with_smallest_location(seeds: Vec<u64>, maps: &Mappings) -> Result<u64> {
    if seeds.is_empty() {
        return Err(anyhow!("No seeds"));
    }
    let location = maps.lookup_seed_location(seeds[0])?;
    let mut smallest = (seeds[0], location);
    for seed in seeds.iter().skip(1) {
        let location = maps.lookup_seed_location(*seed)?;
        if location < smallest.1 {
            smallest = (*seed, location);
        }
    }

    Ok(smallest.0)
}

/// Read a line of the form "seeds: 1 2 3" and return a vector of the seeds.
fn read_seeds(line: &str) -> Result<Vec<u64>> {
    if !line.starts_with("seeds: ") {
        return Err(anyhow!("Line must start with 'seeds: '"));
    }
    let line = &line["seeds: ".len()..];

    let mut seeds = Vec::new();
    for seed in line.split_ascii_whitespace() {
        let seed = seed.parse()
            .with_context(|| format!("Seed '{}' must be a number", seed))?;
        seeds.push(seed);
    }

    Ok(seeds)
}

/// Read all maps in the file.
fn read_all_maps(line_iter: &mut dyn Iterator<Item = io::Result<String>>) -> Result<Mappings> {
    let mut maps = HashMap::new();
    while let Some(map) = read_map(line_iter)? {
        maps.insert(map.source.to_owned(), map);
    }

    Ok(Mappings(maps))
}

/// Reads the map header and mappings until EOF or a blank line.
fn read_map(line_iter: &mut dyn Iterator<Item = io::Result<String>>) -> Result<Option<Map>> {
    let Some(header) = line_iter.next() else {
        return Ok(None);
    };
    let header = header.unwrap();
    let (source, destination) = parse_map_header(&header)
        .with_context(|| format!("Error parsing map header '{header}'"))?;

    let mut mappings = Vec::new();
    for line in line_iter {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        
        let mapping = Mapping::from_str(&line)
            .with_context(|| format!("Error parsing mapping '{line}' for '{header}'"))?;
        mappings.push(mapping);
    }

    Ok(Some(Map {
        source,
        destination,
        mappings,
    }))
}

/// Parse a map header of the form "source-to-destination map:".
/// Returns (source, destination).
fn parse_map_header(line: &str) -> Result<(String, String)> {
    if !line.ends_with(" map:") {
        return Err(anyhow!("Map header must end with ' map:'"));
    }
    let line = &line[..line.len() - " map:".len()];
    let parts: Vec<&str> = line.split('-').collect();
    if parts.len() != 3 || parts[1] != "to" {
        return Err(anyhow!("Map header must be in the format 'source-to-destination map:'"));
    }

    Ok((parts[0].to_string(), parts[2].to_string()))
}

#[cfg(test)]
mod test {
    use std::{vec, io::{BufReader, Cursor}};

    use super::*;

    #[test]
    fn test_seeds() {
        assert_eq!(read_seeds("seeds: 1 2 3").unwrap(), vec![1, 2, 3]);
        assert_eq!(read_seeds("seeds: 1").unwrap(), vec![1]);
        assert_eq!(read_seeds("seeds: ").unwrap(), vec![]);
        assert!(read_seeds("seeds: a").is_err());
        assert!(read_seeds("seeds: 1 a").is_err());
    }

    #[test]
    fn test_read_map() {
        let mut line_iter = vec![
            Ok("seed-to-soil map:".to_string()),
            Ok("50 98 2".to_string()),
            Ok("52 50 48".to_string()),
            Ok("".to_string()),
        ]
            .into_iter();
        let map = read_map(&mut line_iter).unwrap().unwrap();
        assert_eq!(map.source, "seed".to_string());
        assert_eq!(map.destination, "soil".to_string());
        assert_eq!(map.mappings, vec![
            Mapping {
                source: 98..100,
                destination_start: 50,
            },
            Mapping {
                source: 50..98,
                destination_start: 52,
            },
        ]);
    }

    #[test]
    fn test_read_all_maps() {
        let mut line_iter = vec![
            Ok("seed-to-soil map:".to_string()),
            Ok("50 98 2".to_string()),
            Ok("52 50 48".to_string()),
            Ok("".to_string()),
            Ok("soil-to-fertilizer map:".to_string()),
            Ok("0 15 37".to_string()),
            Ok("37 52 2".to_string()),
            Ok("39 0 15".to_string()),
            Ok("".to_string()),
        ]
            .into_iter();
        let maps = read_all_maps(&mut line_iter).unwrap();
        assert_eq!(maps.0.len(), 2);
        let seed_soil_map = maps.0.get("seed").unwrap();
        assert_eq!(seed_soil_map.source, "seed".to_string());
        assert_eq!(seed_soil_map.destination, "soil".to_string());
        assert_eq!(seed_soil_map.mappings, vec![
            Mapping {
                source: 98..100,
                destination_start: 50,
            },
            Mapping {
                source: 50..98,
                destination_start: 52,
            },
        ]);
        let soil_fertilizer_map = maps.0.get("soil").unwrap();
        assert_eq!(soil_fertilizer_map.source, "soil".to_string());
        assert_eq!(soil_fertilizer_map.destination, "fertilizer".to_string());
        assert_eq!(soil_fertilizer_map.mappings, vec![
            Mapping {
                source: 15..52,
                destination_start: 0,
            },
            Mapping {
                source: 52..54,
                destination_start: 37,
            },
            Mapping {
                source: 0..15,
                destination_start: 39,
            },
        ]);
    }

    #[test]
    fn test_map_lookup() {
        let map = Map {
            source: "seed".to_string(),
            destination: "soil".to_string(),
            mappings: vec![
                Mapping {
                    source: 98..100,
                    destination_start: 50,
                },
                Mapping {
                    source: 50..98,
                    destination_start: 52,
                },
            ],
        };
        assert_eq!(map.lookup(79), 81);
        assert_eq!(map.lookup(14), 14);
        assert_eq!(map.lookup(55), 57);
        assert_eq!(map.lookup(13), 13);
    }

    #[test]
    fn test_lookup_seed_location() {
        let text = r#"seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;
        let cursor = Cursor::new(text);
        let mut line_iter = BufReader::new(cursor).lines();
        let maps = read_all_maps(&mut line_iter).unwrap();
        assert_eq!(maps.lookup_seed_location(79).unwrap(), 82);
        assert_eq!(maps.lookup_seed_location(14).unwrap(), 43);
        assert_eq!(maps.lookup_seed_location(55).unwrap(), 86);
        assert_eq!(maps.lookup_seed_location(13).unwrap(), 35);
    }
}
