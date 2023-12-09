#[warn(dead_code)]
const fn get_num_lines() -> usize {
    let mut i = 0;
    let mut lines = 1;
    while i < INPUT.as_bytes().len() {
        if INPUT.as_bytes()[i] == b'\n' {
            lines += 1
        }
        i += 1;
    }
    lines
}

fn parse(num: &str) -> u8 {
    match num.len() {
        1 => num.as_bytes()[0] - b'0',
        2 => 10 * (num.as_bytes()[0] - b'0') + (num.as_bytes()[1] - b'0'),
        _ => panic!("Only 2-digit numbers are supported! Received: {num}"),
    }
}

#[aoc::main(04)]
fn main() -> () {}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_card, content) = line.split_once(':').expect("at least one :");
            let (winning, owned) = content.split_once('|').expect("Should have |");

            let win_bitmask = winning
                .split_ascii_whitespace()
                .map(parse)
                .fold(0u128, |bitmask, v| bitmask | (1 << v));
            match owned
                .split_ascii_whitespace()
                .map(parse)
                .filter(|val| win_bitmask & (1 << val) != 0)
                .count()
            {
                0 => 0,
                val => 2u32.pow(val as u32 - 1),
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    const L: usize = get_num_lines();
    let mut copies: [u32; L] = [1; L];
    dbg!(L);
    input.lines().enumerate().for_each(|(card_id, line)| {
        let (_card, content) = line.split_once(':').expect("at least one :");
        let (winning, owned) = content.split_once('|').expect("Should have |");
        let win_bitmask = winning
            .split_ascii_whitespace()
            .map(parse)
            .fold(0u128, |bitmask, v| bitmask | (1 << v));

        let matches = owned
            .split_ascii_whitespace()
            .map(parse)
            .filter(|val| win_bitmask & (1 << val) != 0)
            .count();

        let current_vale = copies[card_id];

        copies[card_id + 1..L.min(card_id + matches + 1)]
            .iter_mut()
            .for_each(|val| *val += current_vale);
    });

    copies.iter().sum()
}
