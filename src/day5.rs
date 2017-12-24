use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
	let part2 = true;
	let f = File::open("day5input.txt").unwrap();
	let file = BufReader::new(&f);
	let mut vec = Vec::new();
	for line in file.lines() {
		let n = line.unwrap().parse::<i32>().unwrap();
		vec.push(n);
	}
	println!("Line count {}", vec.len());

	let mut steps: i32 = 0;
	let mut index: i32 = 0;
	while index >= 0 && (index as usize) < vec.len() {
		steps = steps + 1;
		let jump = vec[index as usize];
		if jump >= 3 && part2 {
			vec[index as usize] = jump - 1;
		} else {
			vec[index as usize] = jump + 1;			
		}
		index = index + jump;
	}
	println!("Broke out in {} steps", steps);
}