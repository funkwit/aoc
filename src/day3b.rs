use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

enum Direction {
	Up,
	Left,
	Down,
	Right
}

fn main() {
	let target = 347991;
    let mut last_val = 0;

    let mut x = 0;
    let mut y = 0;
    let mut grid: HashMap<Point, i32> = HashMap::new();
    let mut next_direction = Direction::Right;

    while last_val < target {
    	println!("Looking for neighbours of {}, {}", x, y);

    	let mut val = 0;
    	for x_delta in -1..2 {
    		for y_delta in -1..2 {
    			println!("Checking at {}, {}", x + x_delta, y + y_delta);

    			val = val + match grid.get(&Point{x: x + x_delta, y: y + y_delta}) {
    				Some(&number) => number,
    				_ => 0,
				}
    		}
    	}
    	if val == 0 {
    		// First square
    		val = 1;
    	}
    	println!("Inserting value {}", val);
    	grid.insert(Point{x: x, y: y}, val);

    	match next_direction {
    		Direction::Up => {
    			y = y + 1;
    			if !grid.contains_key(&Point{x: x - 1, y: y}) {
    				next_direction = Direction::Left;
    			}
    		},
    		Direction::Left => {
    			x = x - 1;
    			if !grid.contains_key(&Point{x: x, y: y - 1}) {
    				next_direction = Direction::Down;
    			}
    		},
    		Direction::Down =>{
    			y = y - 1;
    			if !grid.contains_key(&Point{x: x + 1, y: y}) {
    				next_direction = Direction::Right;
    			}
    		},
    		Direction::Right => {
    			x = x + 1;
    			if !grid.contains_key(&Point{x: x, y: y + 1}) {
    				next_direction = Direction::Up;
    			}
    		},
    	}

    	last_val = val;
    }
}