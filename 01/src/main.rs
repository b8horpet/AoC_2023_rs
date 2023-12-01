use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let digits : HashMap<i32, Vec<&str>> = HashMap::from([
        (0, vec!["0", "zero"]),
        (1, vec!["1", "one"]),
        (2, vec!["2", "two"]),
        (3, vec!["3", "three"]),
        (4, vec!["4", "four"]),
        (5, vec!["5", "five"]),
        (6, vec!["6", "six"]),
        (7, vec!["7", "seven"]),
        (8, vec!["8", "eight"]),
        (9, vec!["9", "nine"]),
    ]);
    let args: Vec<String> = env::args().collect();
    let filename = {
        if args.len() > 1 { &args[1][..] }
        else { "input.txt" }
    };
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    let mut result1 = 0_usize;
    let mut result2 = 0_usize;
    file.read_to_string(&mut contents)?;
    for l in contents.lines() {
        let mut first_val1 = (-1,l.len());
        let mut last_val1 = (-1, 0_usize);
        let mut first_val2 = (-1,l.len());
        let mut last_val2 = (-1, 0_usize);
        for (v,ds) in &digits {
            assert!(!ds.is_empty());
            // star1
            let d1 = ds[0];
            if let Some(p) = l.find(d1) {
                if p <= first_val1.1 {
                    first_val1 = (*v,p);
                }
            }
            if let Some(p) = l.rfind(d1) {
                    if p >= last_val1.1 {
                    last_val1 = (*v,p);
                }
            }
            for d in ds {
                if let Some(p) = l.find(d) {
                    if p <= first_val2.1 {
                        first_val2 = (*v,p);
                    }
                }
                if let Some(p) = l.rfind(d) {
                        if p >= last_val2.1 {
                        last_val2 = (*v,p);
                    }
                }
            }
        }
        assert!((0..=9).contains(&first_val1.0));
        assert!((0..=9).contains(&last_val1.0));
        assert!((0..=9).contains(&first_val2.0));
        assert!((0..=9).contains(&last_val2.0));
        result1 += (first_val1.0 * 10 + last_val1.0) as usize;
        result2 += (first_val2.0 * 10 + last_val2.0) as usize;
    
    }
    println!("star1: {result1}");
    println!("star2: {result2}");
    Ok(())
}
