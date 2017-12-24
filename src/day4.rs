use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
	let f = File::open("day4input.txt").unwrap();
    let file = BufReader::new(&f);
    let mut valid_count = 0;
    for line in file.lines() {
    	let mut line = line.unwrap();
    	println!("Line: {}", line);
    	let mut parts = line.split_whitespace();
  	    let mut words = HashSet::new();
  	    let mut valid = true;
		for p in parts {
			if words.contains(p) {
				println!("Duplicate word {}", p);
				valid = false;
			}
			words.insert(p);
		}
		if valid {
			valid_count = valid_count + 1;
		}
	}
	println!("Valid lines: {}", valid_count);
}