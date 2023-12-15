use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    // Declare a hash map that relates integer values to the words for them
    let numbers_as_words: HashMap<&str, i32> = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into();

    // Splint string by newlines into an iterator
    let parts = input.split("\n");

    // Find numbers in each entry and load them into an interator
    let mut calibration_values: Vec<i32> = Vec::new();
    for part in parts {
        // Replace all number as word instances with its i32 representation
        // Number and index for order
        let mut nums: Vec<i32> = Vec::new();

        let chars: Vec<&str> = part.split("").collect();

        for i in 0..chars.len() {
            match chars[i].parse::<i32>() {
                Ok(x) => nums.push(x), //number
                _ => {
                    // letter
                    // from this index check if this letter is the start of a number word
                    for word in numbers_as_words.iter() {
                        let length: usize = word.0.len();
                        let start = i;
                        let end = i + length;
                        if end < chars.len() {
                            let word_slice = &chars[start..end];
                            if word.0.to_string() == word_slice.join("") {
                                nums.push(word.1.clone())
                            }
                        }
                    }
                }
            }
        }

        if nums.len() > 1 {
            // If there are multiple numbers get the first and last
            match (nums.first(), nums.last()) {
                (Some(tens), Some(ones)) => {
                    // Use the first number for the tens digit and the last number for the ones
                    let final_number = tens * 10 + ones;
                    calibration_values.push(final_number)
                }
                _ => (),
            }
        } else if nums.len() == 1 {
            // Use the number twice
            match nums.first() {
                Some(num) => {
                    let final_number = num * 10 + num;
                    calibration_values.push(final_number)
                }
                _ => (),
            }
        }
    }
    // Return the sum of all the numbers
    return calibration_values.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let result = part2(test_input);
        assert_eq!(result, 281);
    }
}
