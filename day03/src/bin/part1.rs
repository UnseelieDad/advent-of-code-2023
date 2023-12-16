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

    let mut numbers: Vec<Vec<((usize, usize), u32)>> = vec![];

    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers.iter_mut().last().expect("Should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![((*x, *y), *num)]);
                            }
                        }
                        None => unimplemented!("Shouldn't happen"),
                    }
                }
                None => numbers.push(vec![((*x, *y), *num)]),
            }
        }
    }

    for num_list in numbers {
        let postions = [
            (1, 0),
            (1, -1),
            (1, 1),
            (0, 1),
            (0, -1),
            (-1, -1),
            (-1, 1),
            (-1, 0),
        ];
        let num_positions: Vec<(i32, i32)> = num_list
            .iter()
            .map(|((y, x), _)| (*x as i32, *y as i32))
            .collect();
        let pos_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                postions.iter().map(|outer_pos| {
                    // outer_pos.x + pos.x, outer_pos.y + pos.y
                    (outer_pos.0 + pos.1 as i32, outer_pos.1 + pos.0 as i32)
                })
            })
            .unique()
            .filter(|num| !num_positions.contains(num))
            .collect();
    }

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
