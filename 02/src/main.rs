use std::fs::File;
use std::io::prelude::*;

type Triplet = (usize,usize,usize);
type Game = (usize, Vec<Triplet>);

fn main() -> std::io::Result<()> {
	let mut file = File::open("input.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let mut sum = 0_usize;
	let truth : Triplet = (12_usize, 13_usize, 14_usize);
	for line in contents.lines() {
		let g = parse_line(line);
		if is_valid_match(&g, &truth) {
			sum += g.0;
		}
	}
	println!("{sum}");
	Ok(())
}

fn parse_line(line: &str) -> Game {
	assert!(&line[0..5] == "Game ");
	let parts : Vec<&str> = line.split(':').collect();
	assert!(parts.len() == 2);
	let id = parts[0][5..].parse::<usize>().unwrap();
	let draws: Vec<&str> = parts[1].split(';').collect();
	assert!(!draws.is_empty());
	(id,draws.iter().map(|txt: &&str|parse_triplet(txt)).collect())
}

// triplet order is R G B
fn parse_triplet(txt: &str) -> Triplet {
	let pieces: Vec<&str> = txt.split(',').collect();
	assert!(pieces.len() <= 3);
	let mut triplet = [0_usize, 0_usize, 0_usize];
	for p in pieces {
		let info: Vec<&str> = p.split(' ').collect();
		assert!(info.len()==3);
		let amount = info[1].parse::<usize>().unwrap();
		let idx: usize = match info[2] {
			"red" => 0_usize,
			"green" => 1_usize,
			"blue" => 2_usize,
			_ => panic!(),
		};
		assert!(amount > 0);
		assert!(triplet[idx] == 0);
		triplet[idx]=amount;
	}
	(triplet[0], triplet[1], triplet[2])
}

fn is_valid_match(game: &Game, truth: &Triplet) -> bool {
	for draw in &game.1 {
		if draw.0 > truth.0 || draw.1 > truth.1 || draw.2 > truth.2 {
			return false;
		}
	}
	true
}
