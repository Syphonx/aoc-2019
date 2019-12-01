/*
    --- Day 1: The Tyranny of the Rocket Equation ---
*/

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            let x: i32 = x.trim().parse().expect("Invalid argument");
            (x / 3) - 2
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            let mass: i32 = x.trim().parse().expect("Invalid argument");
            let mut fuel: i32 = (mass / 3) - 2;
            let mut sum: i32 = fuel;
            loop {
                fuel = (fuel / 3) - 2;
                if fuel > 0 {
                    sum += fuel;
                } else {
                    break;
                }
            }
            return sum;
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(solve_part1("12"), 2);
        assert_eq!(solve_part1("14"), 2);
        assert_eq!(solve_part1("1969"), 654);
        assert_eq!(solve_part1("100756"), 33583);
    }
    #[test]
    fn part_2() {
        assert_eq!(solve_part2("14"), 2);
        assert_eq!(solve_part2("1969"), 966);
        assert_eq!(solve_part2("100756"), 50346);
    }
}
