///
///  Day 02
///
const R: u32 = 12;
const G: u32 = 13;
const B: u32 = 14;

#[aoc::main(02)]
fn main(input: &str) -> (u32, u32) {}

fn part1(input: &str) -> u32 {
    let mut result = 0;
    input.lines().enumerate().for_each(|(i, line)| {
        let game_id = i as u32 + 1;
        let mut valid = true;
        let content = line.split(':').last().unwrap();
        content.split(';').for_each(|set| {
            if !valid {
                return;
            }
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;
            set.split(',').for_each(|pair| {
                let mut it = pair.split_ascii_whitespace();
                let num: u32 = it.next().unwrap().parse().unwrap();
                let color: &str = it.next().unwrap();
                match color.as_bytes()[0] {
                    b'r' => r += num,
                    b'g' => g += num,
                    b'b' => b += num,
                    _ => panic!("No color"),
                }
            });

            if R < r || G < g || B < b {
                valid = false;
            }
        });
        if valid {
            result += game_id;
        }
    });
    result
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    input.lines().for_each(|game| {
        let mut max_r: u32 = 0;
        let mut max_g: u32 = 0;
        let mut max_b: u32 = 0;
        let content = game.split(':').last().unwrap();
        content.split(';').for_each(|set| {
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;
            set.split(',').for_each(|pair| {
                let mut it = pair.split_ascii_whitespace();
                let num: u32 = it.next().unwrap().parse().unwrap();
                let color: &str = it.next().unwrap();
                match color.as_bytes()[0] {
                    b'r' => r += num,
                    b'g' => g += num,
                    b'b' => b += num,
                    _ => panic!("No color"),
                }
            });

            max_r = max_r.max(r);
            max_g = max_g.max(g);
            max_b = max_b.max(b);
        });
        result += max_r * max_g * max_b;
    });
    result
}
