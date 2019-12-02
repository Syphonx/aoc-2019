/*
    --- Day 2: 1202 Program Alarm ---
*/

pub fn mem_set(memory: &mut Vec<i32>, index: usize, value: i32) {
    let address = memory[index];
    memory[address as usize] = value;
}

pub fn mem_get(memory: &mut Vec<i32>, index: usize) -> i32 {
    let address = memory[index];
    return memory[address as usize];
}

pub fn opcode_add(memory: &mut Vec<i32>, index: usize) {
    // println!(
    //     "opcode_add {}+{}={}",
    //     mem_get(memory, index + 1),
    //     mem_get(memory, index + 2),
    //     mem_get(memory, index + 1) + mem_get(memory, index + 2)
    // );
    let value = mem_get(memory, index + 1) + mem_get(memory, index + 2);
    mem_set(memory, index + 3, value);
}

pub fn opcode_mul(memory: &mut Vec<i32>, index: usize) {
    // println!(
    //     "opcode_mul {}*{}={}",
    //     mem_get(memory, index + 1),
    //     mem_get(memory, index + 2),
    //     mem_get(memory, index + 1) * mem_get(memory, index + 2)
    // );
    let value = mem_get(memory, index + 1) * mem_get(memory, index + 2);
    mem_set(memory, index + 3, value);
}

pub fn run_intcode(memory: &mut Vec<i32>, noun: i32, verb: i32) -> i32 {
    let len = memory.len();
    memory[1] = noun;
    memory[2] = verb;
    for index in (0..len).step_by(4) {
        let opcode = memory[index];
        // println!("Before Memory: {:?}", memory);
        match opcode {
            1 => {
                opcode_add(memory, index);
            }
            2 => {
                opcode_mul(memory, index);
            }
            99 => {
                break;
            }
            _ => println!("Invalid opcode!"),
        }
        // println!("After Memory: {:?}", memory);
    }
    return memory[0];
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut memory: Vec<i32> = input
        .split_terminator(",")
        .map(|x| x.trim().parse::<i32>().expect("Expected integer"))
        .collect();
    return run_intcode(&mut memory, 12, 2);
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut memory: Vec<i32> = input
        .split_terminator(",")
        .map(|x| x.trim().parse::<i32>().expect("Expected integer"))
        .collect();
    let backup = memory.clone();
    for noun in 0..100 {
        for verb in 0..100 {
            // println!("{}, {}", noun, verb);
            let result = run_intcode(&mut memory, noun, verb);
            if result == 19690720 {
                return  100 * noun + verb;
            } else {
                memory = backup.clone();
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        assert_eq!(solve_part1("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
    }
}
