#[derive(serde::Deserialize)]
enum Command {
	#[serde(rename = "forward")]
	Forward(i32),
	#[serde(rename = "up")]
	Up(i32),
	#[serde(rename = "down")]
	Down(i32),
}

fn part_1(commands: &[Command]) -> i32 {
	let (mut horizontal, mut depth) = (0, 0);

	for command in commands {
		match command {
			Command::Down(amount) => depth += amount,
			Command::Up(amount) => depth -= amount,
			Command::Forward(amount) => horizontal += amount,
		};
	}

	horizontal * depth
}

fn part_2(commands: &[Command]) -> i32 {
	let (mut horizontal, mut depth, mut aim) = (0, 0, 0);

	for command in commands {
		match command {
			Command::Down(amount) => aim += amount,
			Command::Up(amount) => aim -= amount,
			Command::Forward(amount) => {
				horizontal += amount;
				depth += aim * amount;
			}
		};
	}

	horizontal * depth
}

fn main() {
	let commands: Vec<Command> = include_str!("../input")
		.lines()
		.map(|line| serde_scan::from_str(line).unwrap())
		.collect();

	println!("Part 1: {}", part_1(&commands));
	println!("Part 2: {}", part_2(&commands));
}
