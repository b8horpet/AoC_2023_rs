use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut sum = 0_usize;
    for l in contents.lines() {
        assert!(&l[0..5] == "Card ");
        let card: &str = {
            let v: Vec<&str> = l.split(':').collect();
            assert!(v.len()==2);
            v[1]
        };
        let nums: Vec<&str> = card.split('|').collect();
        assert!(nums.len() == 2);
        let nums : Vec<HashSet<usize>> = nums.iter().map(parse_nums).collect();
        eprint!("winning:");
        for n in &nums[0] {
            eprint!(" {n}");
        }
        eprint!(" | darw:");
        for n in &nums[1] {
            eprint!(" {n}");
        }
        let i : u32 = nums[0].intersection(&nums[1]).count().try_into().unwrap();
        eprint!(" => found {i} mathc(es)\n");
        if i>0u32 {
            sum += 1_usize.checked_shl(i-1u32).unwrap();
        }
    }
    println!("{sum}");
    Ok(())
}

fn parse_nums(s: &&str) -> HashSet<usize> {
    s.split(' ').filter(|x| !x.is_empty()).map(|x| x.parse::<usize>().unwrap()).collect()
}
