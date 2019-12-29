/*
	--- Intcode VM ---
*/

fn main() {
	println!(read_input(0))
}

fn read_input(day: usize) -> String {
	return std::fs::read_to_string(format!("../../input/{}/day{:0>2}.txt", day)).unwrap();
}
