use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn sort(s: &str) -> String {
	let mut chars = s.chars().collect::<Vec<char>>();
	chars.sort();
	chars.into_iter().collect::<String>()
}

fn main() {
	let f = File::open("input/day4").unwrap();
	let file = BufReader::new(&f);
	let mut valid_count = 0;
	for line in file.lines() {
		let mut line = line.unwrap();
		println!("Line: {}", line);
		let mut parts = line.split_whitespace();
		let mut word;
  		let mut words = HashSet::new();
  		let mut valid = true;
		for p in parts {
			// For part a, remove sort
			word = sort(p);

			if words.contains(&word) {
				println!("Duplicate word {}", word);
				valid = false;
			}
			words.insert(word);
		}
		if valid {
			valid_count = valid_count + 1;
		}
	}
	println!("Valid lines: {}", valid_count);
}