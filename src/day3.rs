fn main() {
	let target = 347991;
	let mut min;
	let mut max = 1;
	let mut level = 0;
	let mut delta_to_first_midpoint = 0; 
	let mut first_midpoint = 1;
	let mut midpoint_spacing = 0;
	let mut level_count = 8;

	while max < target {
		level = level + 1;
		min = max + 1;
		max = max + level_count;
		level_count = level_count + 8;
		first_midpoint = min + delta_to_first_midpoint;
		delta_to_first_midpoint = delta_to_first_midpoint + 1;
		midpoint_spacing = midpoint_spacing + 2;
		println!("Entering level {}, {}-{}, first midpoint @ {} and then every {}", level, min, max, first_midpoint, midpoint_spacing);
	}
	let nearest_midpoint = ((((target - first_midpoint) + midpoint_spacing/2) / midpoint_spacing) * midpoint_spacing) + first_midpoint;
	let to_middle = target - nearest_midpoint;
	let moves = to_middle + level;
	println!("{} to middle, {} back to 1, {} total", to_middle, level, moves);
}