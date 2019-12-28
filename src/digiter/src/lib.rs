pub struct Digits {
	n: usize,
	divisor: usize,
}

impl Digits {
	pub fn new(n: usize) -> Self {
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

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let digits: Vec<_> = Digits::new(12346789).collect();
		assert_eq!(digits, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
	}
}
