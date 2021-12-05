#[derive(Clone, Copy, Debug)]
struct Coords(isize, isize);

impl std::ops::Sub for Coords {
	type Output = Coords;

	fn sub(self, rhs: Self) -> Self::Output {
		Coords(self.0 - rhs.0, self.1 - rhs.1)
	}
}

#[derive(Clone, Copy, Debug)]
struct Line(Coords, Coords);

impl Line {
	fn is_diagonal(&self) -> bool {
		let Line(from, to) = self;
		from.0 != to.0 && from.1 != to.1
	}

	fn area(&self) -> impl Iterator<Item = Coords> + '_ {
		let Coords(x, y) = self.0;
		let Coords(dx, dy) = self.1 - self.0;
		let (xstep, ystep) = (dx.signum(), dy.signum());
		let d = std::cmp::max(dx.abs(), dy.abs());

		(0..=d).map(move |offset| Coords(x + offset * xstep, y + offset * ystep))
	}
}

fn part_1<'a>(lines: impl Iterator<Item = &'a Line>) -> usize {
	part_2(lines.filter(|line| !line.is_diagonal()))
}

fn part_2<'a>(lines: impl Iterator<Item = &'a Line>) -> usize {
	let mut state = vec![vec![0u8; 1000]; 1000];

	lines
		.flat_map(|line| line.area())
		.for_each(|coords| state[coords.0 as usize][coords.1 as usize] += 1);

	state.iter().flatten().filter(|&&x| x >= 2).count()
}

fn main() {
	let lines: Vec<Line> = include_str!("../input")
		.lines()
		.map(|line| {
			let (x1, y1, x2, y2) = serde_scan::scan!("{},{} -> {},{}" <- line).unwrap();
			Line(Coords(x1, y1), Coords(x2, y2))
		})
		.collect();

	println!("Part 1: {}", part_1(lines.iter()));
	println!("Part 2: {}", part_2(lines.iter()));
}
