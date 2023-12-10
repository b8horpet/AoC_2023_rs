use std::error;
use std::fs::File;
use std::io::prelude::*;

type Seeds = Vec<i64>;
type Mapping = (std::ops::Range<i64>, i64);
type Map = Vec<Mapping>;
type Maps = Vec<Map>;
type Almanac = (Seeds, Maps);

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let almanac = parse_almanac(&contents)?;
    eprint!("seeds:");
    for seed in &almanac.0 {
        eprint!(" {seed}");
    }
    eprintln!();
    for (i,map) in almanac.1.iter().enumerate() {
        eprintln!("map #{i}:");
        for m in map {
            eprintln!("[{}-{}) => [{}-{})", m.0.start, m.0.end, m.0.start+m.1, m.0.end+m.1);
        }
        eprintln!();
    }
    let closest = almanac.0.iter()
        .map(|seed| map_seed_to_location(*seed, &almanac.1))
        .min().ok_or_else(
            || std::io::Error::new(std::io::ErrorKind::Other, "min failed?")
        )?;
    println!("{closest}");
    Ok(())
}

fn parse_almanac(almanac: &str) -> Result<Almanac, Box<dyn error::Error>> {
    let mut seeds = Seeds::new();
    let mut maps: Maps = vec![];
    for l in almanac.lines() {
        if l.len()>6 && &l[0..7] == "seeds: " { // initial seeds
            assert!(seeds.is_empty());
            let sv = l.split(':').collect::<Vec<&str>>();
            assert!(sv.len() == 2);
            seeds = sv[1].split(' ')
                .filter(|s| !s.is_empty())
                .map(str::parse::<i64>)
                .collect::<Result<Vec<i64>,_>>()?;
        } else
        if l.len() > 0 && l.chars().last().unwrap() == ':' { // mapping name
            maps.push(vec![]);
        }
        else if !l.is_empty() { // regular line of a mapping
            let mv = l.split(' ')
                .map(str::parse::<i64>)
                .collect::<Result<Vec<i64>,_>>()?;
            assert!(mv.len() == 3);
            assert!(!maps.is_empty());
            maps.last_mut().unwrap().push((mv[1]..mv[1]+mv[2], mv[0]-mv[1]));
        }
    }
    Ok((seeds, maps))
}

fn map_seed_to_location(seed: i64, maps: &Maps) -> i64 {
    let mut target : i64 = seed;
    for map in maps {
        if let Some(mapping) = map.iter().find(|m| m.0.contains(&target)) {
            target += mapping.1;
        }
    }
    target
}