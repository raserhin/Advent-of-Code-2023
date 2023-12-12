use itertools::Itertools;

#[aoc::main(06)]
fn main() -> () {}

// d = tp * (tt  -  tp )
fn distance_time_pressed(time_pressed: u32, total_time: u32) -> u64 {
    time_pressed as u64 * (total_time as u64 - time_pressed as u64)
}

fn time_pressed_to_distance(distance: u64, total_tiume: u32) -> u32 {
    let a = 1;
    let b = -(total_tiume as i64);
    let c = distance as i64;
    let discriminant = b * b - 4 * a * c;

    let sqrt_discriminant = (discriminant as f64).sqrt() as i64;
    ((-b - sqrt_discriminant) / (2 * a)) as u32
}

fn part1(input: &str) -> u32 {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let (_time, times_txt) = time_line.split_once(':').unwrap();
    let times: Vec<u32> = times_txt
        .split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let (_distance, distance_txt) = distance_line.split_once(':').unwrap();
    let distances: Vec<u32> = distance_txt
        .split_ascii_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let mut result = 1;
    for (time, distance) in times.into_iter().zip(distances) {
        let max_time_pressed = time / 2; // Time to travel maximum distance
        let a = time_pressed_to_distance(distance as u64, time);
        let test = (distance as u64) < distance_time_pressed(a, time);
        let parity = time % 2 == 0;

        result *= (max_time_pressed - a + test as u32) * 2 - parity as u32;
    }
    result
}

fn part2(input: &str) -> u32 {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let (_time, times_txt) = time_line.split_once(':').unwrap();
    let (_distance, distance_txt) = distance_line.split_once(':').unwrap();

    let time: u64 = times_txt.split_ascii_whitespace().join("").parse().unwrap();

    let distance: u64 = distance_txt
        .split_ascii_whitespace()
        .join("")
        .parse()
        .unwrap();

    let max_time_pressed = time / 2; // Time to travel maximum distance
    let a = time_pressed_to_distance(distance, time as u32);
    let test = distance < distance_time_pressed(a, time as u32);
    let parity = time % 2 == 0;

    (max_time_pressed as u32 - a + test as u32) * 2 - parity as u32
}
