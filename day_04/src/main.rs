use std::{collections::HashMap, num::ParseIntError, str::FromStr};

#[derive(Clone, Copy, Debug)]
struct Coordinates(pub usize, pub usize);

struct BoardState(u32);

impl BoardState {
	pub fn is_enabled(&self, coords: Coordinates) -> bool {
		self.0 & 1 << Self::position_at(coords) != 0
	}

	pub fn enable(&mut self, coords: Coordinates) {
		self.0 |= 1 << Self::position_at(coords);
	}

	pub fn has_won(&self) -> bool {
		const HORIZ_MASK: u32 = 0b11111;
		const VERT_MASK: u32 = 0b00001_00001_00001_00001_00001;

		let won_horizontally = (0..5).any(|y| self.0 >> y * 5 & HORIZ_MASK == HORIZ_MASK);
		let won_vertically = (0..5).any(|x| self.0 >> x & VERT_MASK == VERT_MASK);

		won_horizontally || won_vertically
	}

	fn position_at(Coordinates(x, y): Coordinates) -> usize {
		y * 5 + x
	}
}

struct Board {
	state: BoardState,
	values: HashMap<u8, Coordinates>,
}

impl FromStr for Board {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut board = Self {
			values: HashMap::new(),
			state: BoardState(0),
		};

		for (y, line) in s.lines().enumerate() {
			for (x, value) in line.split_whitespace().enumerate() {
				board.values.insert(value.parse()?, Coordinates(x, y));
			}
		}

		Ok(board)
	}
}

fn calculate_score(board: &Board, winning_number: u32) -> u32 {
	let unmarked_sum: u32 = board
		.values
		.iter()
		.filter(|(_, coords)| !board.state.is_enabled(**coords))
		.map(|(value, _)| *value as u32)
		.sum();

	unmarked_sum * winning_number
}

fn play(order: &[u8], mut boards: Vec<Board>) -> impl Iterator<Item = u32> + '_ {
	order.iter().flat_map(move |number| {
		boards
			.iter_mut()
			.filter(|board| !board.state.has_won())
			.filter_map(|board| {
				if let Some(coords) = board.values.get(number) {
					board.state.enable(*coords);
				}

				match board.state.has_won() {
					true => Some(calculate_score(board, *number as u32)),
					false => None,
				}
			})
			.collect::<Vec<_>>()
	})
}

fn main() {
	let mut input = include_str!("../input").split("\n\n");

	let order = input
		.next()
		.unwrap()
		.split(",")
		.map(str::parse)
		.collect::<Result<Vec<_>, _>>()
		.expect("invalid order");

	let boards: Vec<Board> = input
		.map(Board::from_str)
		.collect::<Result<Vec<_>, _>>()
		.expect("invalid board");

	let mut winners = play(&order, boards);

	println!("Part 1: {}", winners.next().unwrap());
	println!("Part 2: {}", winners.last().unwrap());
}
