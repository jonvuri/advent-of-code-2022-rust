use aoc_parse::{parser, prelude::*};

use crate::utils;

#[derive(Debug)]
struct Move {
    quantity: usize,
    source: usize,
    target: usize,
}

#[derive(Debug)]
pub struct Input {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

pub fn generator(input: &str) -> Input {
    let stack_parser = parser!(
        rows:lines(
            repeat_sep(
                {
                    "   " => None,
                    "[" ch:alpha "]" => Some(ch),
                },
                " "
            )
        )
        =>
        {
            let length = rows[0].len();
            let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(64); length];
            for row in rows.iter().rev() {
                for (i, ch) in row.iter().enumerate() {
                    if let Some(character) = ch {
                        stacks[i].push(*character);
                    }
                }
            }
            stacks
        }
    );

    let move_parser = parser!(
        "move " quantity:usize " from " source:usize " to " target:usize
            =>
            Move { quantity, source: source - 1, target: target - 1 }
    );

    parser!(
        stacks:stack_parser
        line(repeat_sep(" " digit " ", " "))
        line("")
        moves:lines(move_parser)
        => {
            Input { stacks, moves }
        }
    )
    .parse(&utils::newline_end(input))
    .unwrap()
}

pub fn part1(input: &Input) -> String {
    let mut stacks = input.stacks.clone();

    for Move {
        quantity,
        source,
        target,
    } in input.moves.iter()
    {
        // Move <quantity> items from source to target one by one
        for _ in 0..*quantity {
            let ch = stacks[*source].pop().unwrap();
            stacks[*target].push(ch)
        }
    }

    stacks
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .collect()
}

pub fn part2(input: &Input) -> String {
    let mut stacks = input.stacks.clone();

    for Move {
        quantity,
        source,
        target,
    } in input.moves.iter()
    {
        // Move <quantity> items from source to target all at once
        let length = stacks[*source].len();
        let mut pile = stacks[*source].split_off(length - quantity);
        stacks[*target].append(&mut pile);
    }

    stacks
        .into_iter()
        .map(|stack| stack[stack.len() - 1])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{generator, part1, part2};

    static EXAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part1(&input), "CMZ");
    }

    #[test]
    fn part2_example() {
        let input = generator(EXAMPLE_INPUT);
        assert_eq!(part2(&input), "MCD");
    }
}
