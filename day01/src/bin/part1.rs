fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    // Splint string by newlines into an iterator
    let parts = input.split("\n");

    // Find numbers in each entry and load them into an interator
    let mut calibration_values: Vec<i32> = Vec::new();
    for part in parts {
        // split string into characters and parse into integers. Filter out the letters.
        let nums: Vec<i32> = part
            .split("")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

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
        let test_input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let result = part1(test_input);
        assert_eq!(result, 142);
    }
}

// For each line get the first and last digit appearing to find the number. If there's only one number then use that number twice. Add each number together at the end.
