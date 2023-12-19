use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

type Seeds = Vec<i64>;
type Mapping = (std::ops::Range<i64>, i64);
type Map = Vec<Mapping>;
type Maps = Vec<Map>;
type Almanac = (Seeds, Maps);

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut almanac = parse_almanac(&contents)?;
    let closest = almanac.0.iter()
        .map(|seed| map_seed_to_location(*seed, &almanac.1))
        .min().ok_or_else(
            || std::io::Error::new(std::io::ErrorKind::Other, "min failed?")
        )?;
    println!("{closest}");
    let mut merged = almanac.1
        .drain(..)
        .fold(Map::new(), merge_ranges);
    merged.sort_by_cached_key(|m| m.0.start+m.1);
    let seed_ranges : Vec<std::ops::Range<i64>> = almanac.0.chunks(2)
        .map(|pair| (pair[0] .. pair[0]+pair[1])).collect();
    'merged: for m in merged {
        for s in &seed_ranges {
            if overlaps(&m.0, s) {
                // eprint!("{} -> ", std::cmp::max(m.0.start, s.start));
                println!("{}", std::cmp::max(m.0.start, s.start)+m.1);
                break 'merged;
            }
        }
    }
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
        } else if !l.is_empty() && l.ends_with(':') { // mapping name
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

fn merge_ranges(mut lhs: Map, mut rhs: Map) -> Map {
    let mut result : Map = vec![];
    lhs.sort_by_cached_key(|m| m.0.start+m.1);
    rhs.sort_by_cached_key(|m| m.0.start);
    let mut l = lhs.pop();
    let mut r = rhs.pop();
    loop {
        if l.is_none() && r.is_none() {
            break;
        }
        if l.is_none() {
            result.push(r.unwrap());
            r=rhs.pop();
            continue;
        } else if r.is_none() {
            result.push(l.unwrap());
            l=lhs.pop();
            continue;
        }
        let mut lu = l.clone().unwrap();
        let mut ru = r.clone().unwrap();
        let lr = right_range(&lu);
        let rr = left_range(&ru);
        if overlaps(&lr, &rr) {
            let slice = slice_with(&lr, &rr);
            for p in slice.iter() {
                if p.0.is_some() && p.1.is_some() {
                    let ln = p.0.unwrap();
                    let rn = p.1.unwrap();
                    assert!(ln == rn);
                    let merged = (lu.0.end-ln..lu.0.end, lu.1+ru.1);
                    result.push(merged);
                    lu.0.end-=ln;
                    ru.0.end-=rn;
                } else if let Some(ln) = p.0 {
                    let shrinked = (lu.0.end-ln..lu.0.end, lu.1);
                    result.push(shrinked);
                    lu.0.end-=ln;
                } else if let Some(rn) = p.1 {
                    let shrinked = (ru.0.end-rn..ru.0.end, ru.1);
                    result.push(shrinked);
                    ru.0.end-=rn;
                } else {
                    unreachable!();
                }
            }
            if lu.0.is_empty() {
                l = lhs.pop();
            } else {
                l = Some((lu.0, lu.1));
            }
            if ru.0.is_empty() {
                r = rhs.pop();
            } else {
                r = Some((ru.0, ru.1));
            }
        } else if lr.start > rr.start {
            result.push(lu);
            l=lhs.pop();
        } else {
            result.push(ru);
            r=rhs.pop();
        }
    }
    result
}

fn left_range(m: &Mapping) -> std::ops::Range<i64> {
    m.0.clone()
}

fn right_range(m: &Mapping) -> std::ops::Range<i64> {
    (m.0.start+m.1)..(m.0.end+m.1)
}

fn overlaps(l: &std::ops::Range<i64>, r: &std::ops::Range<i64>) -> bool {
    l.start <= r.end && r.start <= l.end
}

type Offset = Option<i64>;
type OffPair = (Offset, Offset);

fn slice_with(left: &std::ops::Range<i64>, right: &std::ops::Range<i64>) ->
Vec<OffPair> {
    match &left.start.cmp(&right.start) {
        Ordering::Greater => match &left.end.cmp(&right.end) {
            // -> |      -
            // |  | <-   -
            // -> |  |   -
            //    | <-   -
            Ordering::Greater => vec![
                (Some(left.end  - right.end ), None                          ),
                (Some(right.end - left.start), Some(right.end  - left.start)),
                // (None,                         Some(left.start - right.start)),
            ],
            // -> | <-   -
            // |  |  |
            // -> |  |   -
            //    | <-   -
            Ordering::Equal => vec![
                (Some(left.end - left.start), Some(right.end  - left.start )),
                // (None,                        Some(left.start - right.start)),
            ],
            //    | <-   -
            // -> |  |   -
            // |  |  |
            // -> |  |   -
            //    | <-   -
            Ordering::Less => vec![
                (None,                        Some(right.end  - left.end   )),
                (Some(left.end - left.start), Some(left.end   - left.start )),
                // (None,                        Some(left.start - right.start)),
            ],
        }
        Ordering::Equal => match &left.end.cmp(&right.end) {
            // -> |      -
            // |  | <-   -
            // |  |  |
            // -> | <-   -
            Ordering::Greater => vec![
                (Some(left.end  - right.end ), None                         ),
                (Some(right.end - left.start), Some(right.end - right.start)),
            ],
            // -> | <-   -
            // |  |  |
            // -> | <-   -
            Ordering::Equal => vec![
                (Some(left.end - left.start), Some(right.end - right.start)),
            ],
            //    | <-   -
            // -> |  |   -
            // |  |  |
            // -> | <-   -
            Ordering::Less => vec![
                (None,                        Some(right.end - left.end   )),
                (Some(left.end - left.start), Some(left.end  - right.start)),
            ]
        }
        Ordering::Less => match &left.end.cmp(&right.end) {
            // -> |      -
            // |  | <-   -
            // |  |  |
            // |  | <-   -
            // -> |      -
            Ordering::Greater => vec![
                (Some(left.end    - right.end  ), None                         ),
                (Some(right.end   - right.start), Some(right.end - right.start)),
                // (Some(right.start - left.start ), None                         ),
            ],
            // -> | <-   -
            // |  |  |
            // |  | <-   -
            // -> |      -
            Ordering::Equal => vec![
                (Some(left.end    - right.start), Some(right.end - right.start)),
                // (Some(right.start - left.start ), None                         ),
            ],
            //    | <-   -
            // -> |  |   -
            // |  | <-   -
            // -> |      -
            Ordering::Less => vec![
                (None,                            Some(right.end - left.end   )),
                (Some(left.end    - right.start), Some(left.end  - right.start)),
                // (Some(right.start - left.start ), None                         ),
            ],
        }
    }
}
