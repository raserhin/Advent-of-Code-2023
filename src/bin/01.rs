
///
///  Day 01
/// 
use regex::Regex;

#[aoc::main(01)]
fn main()->(){
  
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];


fn get_digit(digit: &str) -> u32 {
    if digit.len() == 1 {
        (digit.as_bytes()[0] - b'0') as u32
    } else {
        DIGITS.iter().position(|&elem| digit == elem).unwrap() as u32 % 9 + 1
    }
}

fn part1(input: &str) -> u32 {
    let mut p1: u32 = 0;

    input.split('\n').for_each(|line| {
        let mut iter = line
          .as_bytes()
          .iter()
          .filter(|s| s.is_ascii_digit());
        let last = iter.next_back().unwrap().to_owned();
        p1 += ((last - b'0') + iter.next().unwrap_or(&last) * 10) as u32;
    });
    p1
}

fn part2(input: &str) -> u32 {
    let mut result: u32 = 0;
    let re: Regex = Regex::new(r".*?(one|two|three|four|five|six|seven|eight|nine|[0-9])+?(?:.*(one|two|three|four|five|six|seven|eight|nine|[0-9]))?.*?").unwrap();

    input.lines().for_each(|line| {
        let captures = re.captures(line).unwrap();
        // Capture at index 0 is the whole string, not the capturing group, start 1
        let first_digit = captures.get(1).unwrap().as_str();
        let last_digit = captures.get(2)
        .map_or(first_digit, |a| a.as_str());

        result += get_digit(first_digit) * 10 + get_digit(last_digit);
    });
    result
}
