use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = {
        if args.len() > 1 { &args[1][..] }
        else { "input.txt" }
    };
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut first_num = -1;
    let mut last_num = -1;
    let mut result = 0;
    for c in contents.chars() {
        match c {
            '\n' => {
                assert!((0..=9).contains(&first_num));
                assert!((0..=9).contains(&last_num));
                result += first_num * 10 + last_num;
                first_num = -1;
                last_num = -1;
            }
            '0' ..= '9' => {
                last_num = (c as i32) - ('0' as i32);
                if first_num == -1 { first_num = last_num; }
            }
            _ => {}
        }
    }
    println!("{result}");
    Ok(())
}
