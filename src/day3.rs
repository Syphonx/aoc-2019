/*
    --- Day 3: Crossed Wires ---
*/

pub enum WireDirection {
    Invalid,
    Up,
    Right,
    Down,
    Left,
}

pub struct Point {
    x: i32,
    y: i32,
}

pub struct Wire {
    direction: WireDirection,
    length: i32,
    index: i32,
    start: Point,
    end: Point,
}

pub fn char_to_direction(input: &char) -> WireDirection {
    match input {
        'U' => WireDirection::Up,
        'D' => WireDirection::Down,
        'L' => WireDirection::Left,
        'R' => WireDirection::Right,
        _ => WireDirection::Invalid,
    }
}

pub fn enum_to_str(input: &WireDirection) -> &'static str {
    match input {
        WireDirection::Up => "Up",
        WireDirection::Down => "Down",
        WireDirection::Left => "Left",
        WireDirection::Right => "Right",
        WireDirection::Invalid => "Invalid",
    }
}

pub fn direction_to_move(input: &Wire) -> (i32, i32) {
    match input.direction {
        WireDirection::Up => (0, input.length),
        WireDirection::Down => (0, -input.length),
        WireDirection::Left => (-input.length, 0),
        WireDirection::Right => (input.length, 0),
        WireDirection::Invalid => (0, 0),
    }
}

#[aoc_generator(day3)]
pub fn input_generator(_input: &str) -> Vec<Wire> {
    let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
    let mut wires: Vec<Wire> = Vec::new();
    let mut marker: (i32, i32) = (0, 0);
    for (index, line) in input.lines().enumerate() {
        for wire in line.trim().split(',') {
            let (dir, len) = wire.trim().split_at(1);
            let mut new_wire = Wire {
                direction: char_to_direction(&dir.chars().next().unwrap()),
                length: len.parse::<i32>().expect("Expected integer"),
                index: index as i32,
                start: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 0 },
            };
            new_wire.start = Point {
                x: marker.0,
                y: marker.1,
            };
            new_wire.end = Point {
                x: marker.0 + direction_to_move(&new_wire).0,
                y: marker.1 + direction_to_move(&new_wire).1,
            };
            marker.0 = new_wire.end.x;
            marker.1 = new_wire.end.y;
            wires.push(new_wire);
        }
    }
    return wires;
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[Wire]) -> i32 {
    let intersections: Vec<Wire> = Vec::new();
    for wire in input {
        for other_wire in input {
            if (wire.index == other_wire.index) {
                continue;
            }
        }
        println!(
            "Wire: ({}, {}) -> ({}, {}) = {}",
            wire.start.x,
            wire.start.y,
            wire.end.x,
            wire.end.y,
            enum_to_str(&wire.direction),
            wire.length
        );
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {}
}
