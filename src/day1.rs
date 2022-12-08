use aoc_parse::{parser, prelude::*};


#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let p = parser!(sections(lines(u32)));

    // So ugly. The input does properly have an ending newline -
    // need to either fix cargo-aoc stripping that, or aoc_parse
    // not being able to handle no ending newline in lines()
    let input_str = input.to_string() + "\n";

    p.parse(&input_str).unwrap()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .map(|group| {
            group.iter().sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day1::input_generator;

    use super::part1;

    #[test]
    fn parses() {
        let input = "100\n200\n\n100\n100";
        assert_eq!(input_generator(&input), vec![vec![100, 200], vec![100, 100]]);
    }

    #[test]
    fn finds_max() {
        let input: Vec<Vec<u32>> = vec![vec![100, 200], vec![100, 100]];
        assert_eq!(part1(&input), 300);
    }
}
