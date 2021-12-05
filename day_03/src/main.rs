fn part_1(input: &[&[u8]]) -> u32 {
	let line_count = input.len();

	let gamma_string: String = (0..input[0].len())
		.map(|col| {
			let iter = input.iter().map(move |line| line[col]);
			let count = iter.filter(|&char| char == b'1').count();
			if count > line_count / 2 {
				'1'
			} else {
				'0'
			}
		})
		.collect();

	let gamma = u32::from_str_radix(&gamma_string, 2).unwrap();
	let epsilon: u32 = !gamma << 20 >> 20;

	gamma * epsilon
}

fn find_rating(mut input: Vec<&[u8]>, most_common: bool) -> u32 {
	for i in 0..input[0].len() {
		let one_count = input.iter().filter(|line| line[i] == b'1').count();

		let bit = match (most_common, one_count * 2 >= input.len()) {
			(true, true) | (false, false) => b'1',
			_ => b'0',
		};

		input.retain(|line| line[i] == bit);

		if input.len() == 1 {
			let string = std::str::from_utf8(input[0]).unwrap();
			return u32::from_str_radix(string, 2).unwrap();
		}
	}

	panic!("never got to length of 1");
}

fn part_2(input: &[&[u8]]) -> u32 {
	let oxygen = find_rating(input.to_vec(), true);
	let carbon = find_rating(input.to_vec(), false);

	oxygen * carbon
}

fn main() {
	let input: Vec<&[u8]> = include_str!("../input")
		.lines()
		.map(|line| line.as_bytes())
		.collect();

	println!("Part 1: {}", part_1(&input));
	println!("Part 2: {}", part_2(&input));
}
