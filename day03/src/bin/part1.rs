use std::collections::BTreeMap;

use itertools::Itertools;

enum Value {
    Empty,
    Number(u32),
    Symbol(char),
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, character)| {
                (
                    (y, x),
                    match character {
                        '.' => Value::Empty,
                        c if c.is_ascii_digit() => {
                            Value::Number(c.to_digit(10).expect("Should be number"))
                        }
                        c => Value::Symbol(c),
                    },
                )
            })
        })
        .collect::<BTreeMap<(usize, usize), Value>>();

    // Try this again, tracking the coordinates of each thing in something like a map
    // How do you keep the digits combined?

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_input = include_str!("./test_input.txt");
        let result = part1(test_input);
        assert_eq!(result, 4361);
    }
}

// Add up all the part numbers in the engine schematic
// Any number adjacent to a part number, even diagonally, is a part number to be included in the sum
// Periods are not a symbol

// For each item that is not a period or a number, check all eight directions and store any numbers in a list if they aren't already
