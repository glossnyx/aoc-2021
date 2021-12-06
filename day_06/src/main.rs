fn simulate_fishies(fishies: &[usize]) -> impl Iterator<Item = u128> {
	let mut initial_state = [0u128; 9];

	for &fishy in fishies {
		initial_state[fishy] += 1;
	}

	(0..).scan(initial_state, |state, _| {
		state.rotate_left(1);
		state[6] += state[8];
		Some(state.iter().copied().sum())
	})
}

fn main() {
	let fishies: Vec<usize> = include_str!("../input")
		.trim()
		.split(",")
		.map(str::parse)
		.collect::<Result<_, _>>()
		.expect("invalid input");

	let mut simulation = simulate_fishies(&fishies);

	println!("Part 1: {}", simulation.nth(80 - 1).unwrap());
	println!("Part 2: {}", simulation.nth(256 - 80 - 1).unwrap());
}
