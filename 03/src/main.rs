use std::fs::File;
use std::io::prelude::*;
use std::cmp;
use std::cmp::Ordering;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut nums = vec![];
    let mut syms = vec![];
    let mut stars = vec![];
    for (y, l) in contents.lines().enumerate() {
        nums.push(vec![]);
        syms.push(vec![]);
        let mut num : (Option<usize>, usize) = (None, 0_usize);
        for (x, c) in l.chars().enumerate() {
            assert!('\n' != c);
            if ('0'..='9').contains(&c) {
                if num.0.is_none() {
                    num.0=Some(x);
                }
                num.1 = num.1 * 10 + ((c as usize)-('0' as usize));
            } else {
                if let Some(begin) = num.0 {
                    let n = num.1;
                    let end = x-1;
                    eprintln!("parsed {n} ({begin}-{end})");
                    nums.last_mut().unwrap().push(((begin,end),n));
                    num = (None, 0_usize);
                }
                if c != '.' {
                    eprintln!("encountered a symbol '{c}' at ({x},{y})");
                    syms.last_mut().unwrap().push(x);
                    if c == '*' {
                        stars.push((y,x));
                    }
                }
            }
        }
        if let Some(begin) = num.0 {
            let n = num.1;
            let end = l.len()-1;
            eprintln!("parsed {n} ({begin}-{end})");
            nums.last_mut().unwrap().push(((begin,end),n));
        }
    }
    let mut sum = 0_usize;
    for (ni,nl) in nums.iter().enumerate() {
        for n in nl {
            let mut found = false;
            for si in cmp::max(ni, 1_usize)-1_usize .. cmp::min(ni+2, syms.len()) {
                if syms[si].binary_search_by(|s| num_order(&n.0, s)).is_ok() {
                    found = true;
                    break;
                }
            }
            if found {
                sum += n.1;
            } else {
                eprintln!("{} not counted on line {} {}-{}", n.1, ni, n.0.0, n.0.1);
            }
        }
    }
    println!("{sum}");
    sum = 0_usize;
    for s in stars {
        let mut found = vec![];
        let sx = s.1;
        for ni in cmp::max(s.0,1_usize)-1_usize .. cmp::min(s.0+2_usize, nums.len()) {
            if let Ok(n) = nums[ni].binary_search_by(|num| num_order(&num.0, &sx).reverse()) {
                found.push(nums[ni][n]);
                assert!(!nums[ni].is_empty());
                if n > 0 && num_contains(&nums[ni][n-1].0, &sx)
                {
                    found.push(nums[ni][n-1]);
                }
                if n < nums[ni].len()-1_usize && num_contains(&nums[ni][n+1].0, &sx)
                {
                    found.push(nums[ni][n+1]);
                }
            }
        }
        if found.len() == 2 {
            sum += found[0].1*found[1].1;
            eprintln!("star {},{} is surrounded by {} and {}", s.0, s.1, found[0].1, found[1].1);
        } else {
            eprintln!("star {},{} is surrounded by {} numbers", s.0, s.1, found.len());
        }
    }
    println!("{sum}");
    Ok(())
}

fn num_contains(num: &(usize,usize), pos: &usize) -> bool {
    let nb = cmp::max(num.0,1_usize)-1_usize;
    let ne = num.1+1_usize;
    return (nb ..= ne).contains(pos);
}

fn num_order(num: &(usize,usize), pos: &usize) -> Ordering {
    let nb = cmp::max(num.0,1_usize)-1_usize;
    let ne = num.1+1_usize;
    if (nb ..= ne).contains(pos) {
        Ordering::Equal
    } else {
        if pos < &nb {
            Ordering::Less
        } else if &ne < pos {
            Ordering::Greater
        } else {
            unreachable!();
        }
    }
}
