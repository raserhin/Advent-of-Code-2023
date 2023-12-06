use itertools::Itertools;

#[warn(dead_code)]
const fn get_num_lines() ->u32 {
    let mut i = 0;
    let mut lines = 0;
    while i < INPUT.as_bytes().len(){
        if(INPUT.as_bytes()[i] == b'\n'){

        }
        i+=1;
    }
    return lines;

}

fn parse(num: &str) -> u8{
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
    input
    .lines()
    .enumerate()
    .map(|(card_id, line)| {
        let card_id = card_id +1;
        let (_card, content) = line.split_once(':').expect("at least one :");
        let (winning, owned) = content.split_once('|').expect("Should have |");
        let mut win_values = [0u8; 10];
        winning
            .split_ascii_whitespace()
            .map(|a| a.parse::<u8>().expect("expected number"))
            .enumerate()
            .for_each(|(i, v)| win_values[i] = v);
        match owned
            .split_ascii_whitespace()
            .map(|num| num.parse::<u8>().unwrap())
            .filter(|val| win_values.contains(val))
            .count()
        {
            0 => 0,
            val => 2u32.pow(val as u32 - 1),
        }
    })
    .sum()
}
