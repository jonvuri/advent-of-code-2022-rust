use aoc_parse::{parser, prelude::*};

use crate::utils;

pub fn generator(input: &str) -> Vec<char> {
    parser!(line(alpha+))
        .parse(&utils::newline_end(input))
        .unwrap()
}

fn first_continuous_distinct(input: &Vec<char>, length: usize) -> Result<usize, String> {
    // Hash of the _next_ index after the last occurrence of each character
    let mut character_hash = [0; 26];

    // Rolling index _after_ the first character of last found duplicate pair
    let mut distinct_start: usize = 0;

    for (index, character) in input.iter().enumerate() {
        let character_index = *character as usize - 'a' as usize;

        if character_index >= 26 {
            return Err(format!("Invalid character: {}", character));
        }

        let window_start = index.saturating_sub(length);

        if index >= length && window_start >= distinct_start {
            // Success, we've gone <length> characters without seeing a duplicate
            return Ok(index);
        } else if character_hash[character_index] > window_start {
            // We've seen this character before inside the window
            // (Found a duplicate pair)

            // This condition could fail if there is another duplicate pair
            // contained in this one
            if character_hash[character_index] > distinct_start {
                distinct_start = character_hash[character_index];
            }
        }

        // Record this as last seen index of character
        character_hash[character_index] = index + 1;
    }

    Err(format!(
        "No sequence of {} continuous distinct characters found",
        length
    ))
}

pub fn part1(input: &Vec<char>) -> Result<usize, String> {
    if input.len() < 4 {
        return Err("Input too short".to_string());
    }

    first_continuous_distinct(input, 4)
}

pub fn part2(input: &Vec<char>) -> Result<usize, String> {
    if input.len() < 14 {
        return Err("Input too short".to_string());
    }

    first_continuous_distinct(input, 14)
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    static EXAMPLE_INPUT: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part1(&input), Ok(11));
    }

    #[test]
    fn part2_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part2(&input), Ok(26));
    }
}
