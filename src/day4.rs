/*
	--- Day 4: Secure Container ---
*/

pub struct Range {
	from: i32,
	to: i32,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Range {
	let input: Vec<&str> = input.trim().split('-').collect();
	let range = Range {
		from: input
			.get(0)
			.unwrap()
			.parse()
			.expect("Expected integer parameter"),
		to: input
			.get(1)
			.unwrap()
			.parse()
			.expect("Expected integer parameter"),
	};
	return range;
}

struct Digits {
	n: usize,
	divisor: usize,
}

impl Digits {
	fn new(n: usize) -> Self {
		let mut divisor = 1;
		while n >= divisor * 10 {
			divisor *= 10;
		}

		Digits { n, divisor }
	}
}

impl Iterator for Digits {
	type Item = usize;
	fn next(&mut self) -> Option<Self::Item> {
		if self.divisor == 0 {
			None
		} else {
			let v = Some(self.n / self.divisor);
			self.n %= self.divisor;
			self.divisor /= 10;
			v
		}
	}
}

pub fn has_adjacent_digits(digits: &Vec<usize>) -> bool {
	let mut adjacent = false;
	for i in 0..digits.len() - 1 {
		if digits.get(i) == digits.get(i + 1) {
			adjacent = true;
			break;
		}
	}
	return adjacent;
}

// Hacked together from: https://gist.github.com/vlmonk/8f466c8b2d99c4bc0be3b355b783c44a because i got stuck.
pub fn has_adjacent_digits_limit(digits: &Vec<usize>) -> bool {
	(0..5).any(|i| match i {
		0 => (digits[0] == digits[1]) && (digits[0] != digits[2]),
		4 => (digits[4] == digits[5]) && (digits[4] != digits[3]),
		n => {
			(digits[n] == digits[n + 1])
				&& (digits[n] != digits[n - 1])
				&& (digits[n] != digits[n + 2])
		}
	})
}

pub fn digits_dont_decrease(digits: &Vec<usize>) -> bool {
	let mut dont_decrease = true;
	for i in 0..digits.len() - 1 {
		if digits.get(i) > digits.get(i + 1) {
			dont_decrease = false;
			break;
		}
	}
	return dont_decrease;
}

pub fn validate_value(value: i32) -> bool {
	let digits: Vec<_> = Digits::new(value as usize).collect();
	return digits_dont_decrease(&digits) && has_adjacent_digits(&digits);
}

pub fn validate_value_part_2(value: i32) -> bool {
	let digits: Vec<_> = Digits::new(value as usize).collect();
	return digits_dont_decrease(&digits) && has_adjacent_digits_limit(&digits);
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Range) -> i32 {
	let mut num = 0;
	for value in input.from..input.to {
		if validate_value(value) {
			num = num + 1;
		}
	}
	return num;
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Range) -> i32 {
	let mut num = 0;
	for value in input.from..input.to {
		if validate_value_part_2(value) {
			num = num + 1;
		}
	}
	return num;
}
