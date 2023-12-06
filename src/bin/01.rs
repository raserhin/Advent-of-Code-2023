///
///  Day 01
///

#[aoc::main(01)]
fn main() -> () {}

const DIGITS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn part1(input: &str) -> u32 {
    let mut p1: u32 = 0;

    input.split('\n').for_each(|line| {
        let mut iter = line.as_bytes().iter().filter(|s| s.is_ascii_digit());
        let last = iter.next_back().unwrap() - b'0';
        p1 += (last + iter.next().map(|&a| a - b'0').unwrap_or(last) * 10) as u32;
    });
    p1
}

fn part2(input: &str) -> u32 {
    let mut result: u32 = 0;

    input.lines().for_each(|line| {
        let first_digit = (0..line.len())
            .find_map(|i| DIGITS.iter().position(|&dig| line[i..].starts_with(dig)))
            .map(|a| a % 9 + 1)
            .expect("Find a number");

        let last_digit = (0..line.len() + 1)
            .rev()
            .find_map(|i| DIGITS.iter().position(|&dig| line[..i].ends_with(dig)))
            .map(|a| a % 9 + 1)
            .expect("Find a number");

        result += first_digit as u32 * 10 + last_digit as u32;
    });
    result
}
