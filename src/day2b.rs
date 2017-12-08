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
    	let mut split: Vec<&str> = l.split('\t').collect();
    	for i in 0 .. split.len() {
    		for j in 0 .. split.len() {
    			if i != j {
	    			let num1 = split[i].parse::<i32>().unwrap();
	    			let num2 = split[j].parse::<i32>().unwrap();
	    			if num1 % num2 == 0 {
	    				sum = sum + (num1 / num2);
	    				println!("Found pair {} / {}: adding {}", num1, num2, num1 / num2);
	    			}

    			}
    		}
    	}
	}
	println!("{}", sum);
}
