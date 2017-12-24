use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
	let f = File::open("input/day6").unwrap();
	let file = BufReader::new(&f);
	let line = file.lines().next().unwrap().unwrap();
	let mut vec = Vec::new();
	
	for num in line.split_whitespace() {
		let n = num.parse::<i32>().unwrap();
		vec.push(n);
	}

	println!("Bucket count {}", vec.len());
	let mut seen = HashSet::new();
	let mut dup = false;
	let mut cycles = 0;

	while !dup {
		println!("{:?}", vec);
		let mut max = -1;
		let mut index = 0;
		for i in 0 .. vec.len() {
			if vec[i] > max {
				max = vec[i];
				index = i;
			}
		}
		println!("Redistributing {} from bucket {}", max, index);
		vec[index] = 0;
		while max > 0 {
			max -= 1;
			index = index + 1;
			let mod_index = index % vec.len();
			vec[mod_index] += 1;
		}
		cycles += 1;

		// Using a debug-rendered vector as the key here is totally hacky. Loldontcare.
		let rendered = format!("{:?}", vec);
		if seen.contains(&rendered) {
			println!("Duplicate state: {}", rendered);
			dup = true;
		}
		seen.insert(rendered);

	}

	println!("Stopped after {} cycles", cycles);

}