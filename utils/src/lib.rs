use std::str::FromStr;

pub fn nums<F: FromStr>(input: &str) -> Vec<F> {
	input.lines().filter_map(|line| line.parse().ok()).collect()
}
