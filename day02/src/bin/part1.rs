use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_until};
use nom::character::complete::{char, digit1, multispace0, multispace1};
use nom::character::streaming::space1;
use nom::multi::{many_m_n, many_till, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

struct Game {
    id: i32,
    highest_red: i32,
    highest_blue: i32,
    highest_green: i32,
}

impl Game {
    fn check_valid(&self) -> bool {
        return self.highest_red <= 12 && self.highest_blue <= 14 && self.highest_green <= 13;
    }
}

fn parse_game_id(input: &str) -> IResult<&str, &str> {
    // Return the number from the part of the strin that reads "Game 1: " etc
    delimited(tag("Game "), digit1, tag(": "))(input)
}

fn parse_game_sets(input: &str) -> IResult<&str, Vec<Vec<(&str, &str)>>> {
    // split list of colors and numbers by "; ", then ", ", and then split the number and its color into pairs
    separated_list1(
        tag("; "),
        separated_list1(
            tag(", "),
            separated_pair(digit1, space1, alt((tag("red"), tag("green"), tag("blue")))),
        ),
    )(input)
}

fn parse_line_to_game_data(line: &str) -> Game {
    // Parse string into operable data
    let (remainder, game_id_str) = parse_game_id(line).unwrap();
    let (_remainder, game_set) = parse_game_sets(remainder).unwrap();

    // flatten to 1 dimensional vector and convert cube numbers to i32
    let flat_data = game_set
        .into_iter()
        .flatten()
        .map(|(n, c)| (n.parse::<i32>().ok().unwrap(), c));

    // Iterate over each number color pair and find the highest number for each color in the list
    let mut highest_red = 0;
    let mut highest_blue = 0;
    let mut highest_green = 0;

    for color in flat_data {
        match color.1 {
            "red" => {
                if color.0 > highest_red {
                    highest_red = color.0;
                }
            }
            "blue" => {
                if color.0 > highest_blue {
                    highest_blue = color.0;
                }
            }
            "green" => {
                if color.0 > highest_green {
                    highest_green = color.0;
                }
            }
            _ => (),
        }
    }

    return Game {
        id: game_id_str.parse::<i32>().ok().unwrap(),
        highest_red,
        highest_blue,
        highest_green,
    };
}

fn part1(input: &str) -> i32 {
    let lines = input.lines();

    return lines
        .map(|line| parse_line_to_game_data(line))
        .filter(|game| game.check_valid())
        .map(|game| game.id)
        .sum::<i32>();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_input = include_str!("./test_input.txt");
        let result = part1(test_input);
        assert_eq!(result, 8);
    }
}

// Which games would have been possible if there are 12 red cubes, 13 green cubes, and 14 blue cubes
// Sum the ids of games where it would be possible
// Only need to check the highest number in a game set

// Split by colon, grab number, that's the game id, discard rest of that portion
// For remaining input group by color, grab highest nummber
