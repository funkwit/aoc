use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
fn main() {
	let f = File::open("input.txt").unwrap();
    let mut file = BufReader::new(&f);
    let mut sum = 0;
    for line in file.lines() {
        let l = line.unwrap();
    	let mut min = 9999999;
    	let mut max = 0;
    	let mut split = l.split('\t');
		for s in split {
			let n = s.parse::<i32>().unwrap();
			if n < min {
				min = n;
			}
			if n > max {
				max = n;
			}
    		println!("{}", n)
		}    	
        println!("{} min {} max {}", l, min, max); 
        sum = sum + (max - min);
	}
	println!("{}", sum);
}
