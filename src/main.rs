mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

aoc_main::main! {
    year 2022;
    day1 : generator => part1, part2;
    day2 : generator => part1?, part2?;
    day3 : generator => part1?, part2?;
    day4 : generator => part1;
    day5 : generator => part1, part2;
    day6 : generator => part1?, part2?;
    day7 : generator => part1?, part2?;
}
