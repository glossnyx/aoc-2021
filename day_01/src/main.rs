use itertools::Itertools;

const INPUT: &str = include_str!("../input");

fn part_1(numbers: impl Iterator<Item = usize>) -> usize {
	numbers.tuple_windows().filter(|(a, b)| b > a).count()
}

fn part_2(numbers: impl Iterator<Item = usize>) -> usize {
	numbers.tuple_windows().filter(|(a, _, _, d)| d > a).count()
}

fn main() {
	let numbers: Vec<usize> = utils::nums(INPUT);

	println!("Part 1: {}", part_1(numbers.iter().copied()));
	println!("Part 2: {}", part_2(numbers.iter().copied()));
}
