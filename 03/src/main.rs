use std::fs::File;
use std::io::prelude::*;
use std::cmp;
use std::cmp::Ordering;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    file.read_to_string(&mut contents)?;
    let mut nums = vec![];
    let mut syms = vec![];
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
            let nb = cmp::max(n.0.0,1_usize)-1_usize;
            let ne = n.0.1+1_usize;
            let mut found = false;
            for si in cmp::max(ni, 1_usize)-1 .. cmp::min(ni+2, syms.len()) {
                if syms[si].binary_search_by(
                    |s| if (&nb ..= &ne).contains(&s) {Ordering::Equal} else {
                        if s < &nb {Ordering::Less}
                        else if &ne < s {Ordering::Greater}
                             else {unreachable!()}
                    } 
                ).is_ok() {
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
    Ok(())
}
