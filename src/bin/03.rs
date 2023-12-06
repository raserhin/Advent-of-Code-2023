const fn get_width() -> usize {
    let mut i = 0;
    loop {
        if INPUT.as_bytes()[i] == b'\n' {
            break i + 1;
        }
        i += 1;
    }
}
const WIDTH: usize = get_width(); // count for \n at end of first line
const LINES: usize = WIDTH - 1; // Assuming square input
#[aoc::main(03)]
fn main() -> () {}

fn contains_symbol(input: &str) -> bool {
    input.contains(|ch: char| ch != '.' && ch != '\n' && !ch.is_ascii_digit())
}
fn contains_number(input: &str) -> bool {
    input.contains(|ch: char| ch.is_ascii_digit())
}

fn expand_index(mut start_pos: usize, input: &str) -> u32 {
    let mut end_pos = start_pos + 1;
    while input[start_pos..end_pos + 1]
        .as_bytes()
        .iter()
        .all(|a| a.is_ascii_digit())
    {
        end_pos += 1;
    }
    while start_pos > 0
        && input[start_pos - 1..end_pos]
            .as_bytes()
            .iter()
            .all(|a| a.is_ascii_digit())
    {
        start_pos -= 1;
    }
    input[start_pos..end_pos].parse::<u32>().unwrap()
}

fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;
    let mut in_number = false;
    let mut start_pos = 0;
    for (i, value) in input.as_bytes().iter().enumerate() {
        match value {
            (b'0'..=b'9') => {
                if !in_number {
                    in_number = true;
                    start_pos = i;
                }
            }
            _ => {
                // Number has finished, look for symbols to see if needs to be added
                if in_number {
                    in_number = false;

                    // The value is a symbol itself, just sum and continue
                    if value != &b'.' && value != &b'\n' {
                        result += input[start_pos..i].parse::<u32>().unwrap();
                    } else {
                        let mut symbol_around = false;
                        // index before if not first char
                        symbol_around = symbol_around
                            || start_pos > 0 && contains_symbol(&input[start_pos - 1..start_pos]);
                        // row above if past first line
                        symbol_around = symbol_around
                            || start_pos > WIDTH
                                && contains_symbol(&input[start_pos - WIDTH - 1..i - WIDTH + 1]);
                        // row belof if not last line
                        symbol_around = symbol_around
                            || start_pos / WIDTH < LINES
                                && contains_symbol(&input[start_pos + WIDTH - 1..i + WIDTH + 1]);

                        if symbol_around {
                            result += input[start_pos..i].parse::<u32>().unwrap();
                        }
                    }
                }
            }
        }
    }
    result
}

fn part2(input: &str) -> u32 {
    let mut result: u32 = 0;
    for i in input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_i, &val)| val == b'*')
        .map(|a| a.0)
    {
        let mut temp_result = 1;
        let mut numbers_surrounding = 0;
        // index before if not first char
        let number_before = i > 0 && contains_number(&input[i - 1..i]);
        // index after
        let number_after = i < WIDTH * LINES - 1 && contains_number(&input[i + 1..i + 2]);
        // row above if past first line
        let number_above = i > WIDTH && contains_number(&input[i - WIDTH - 1..i - WIDTH + 2]);
        // row below if not last line
        let number_below =
            i / WIDTH < LINES && contains_number(&input[i + WIDTH - 1..i + WIDTH + 2]);

        if number_before as u8 + number_above as u8 + number_after as u8 + number_below as u8 > 2 {
            continue;
        }

        // dbg!(i / WIDTH+ 1, i % WIDTH +1);

        if number_before {
            numbers_surrounding += 1;
            temp_result *= expand_index(i - 1, input)
        }
        if number_after {
            numbers_surrounding += 1;
            temp_result *= expand_index(i + 1, input)
        }
        if numbers_surrounding < 2 && number_above {
            if input[i - WIDTH..i - WIDTH + 1].contains('.')
                && input[i - WIDTH - 1..i - WIDTH + 2]
                    .chars()
                    .filter(|ch| ch.is_ascii_digit())
                    .count()
                    == 2
            {
                numbers_surrounding += 2;
                temp_result *=
                    expand_index(i - WIDTH - 1, input) * expand_index(i - WIDTH + 1, input)
            } else {
                numbers_surrounding += 1;
                // From left to right
                let mut start_pos = i - WIDTH - 1;
                start_pos += input[start_pos..start_pos + 3]
                    .as_bytes()
                    .iter()
                    .position(|a| a.is_ascii_digit())
                    .unwrap();
                temp_result *= expand_index(start_pos, input)
            }
        }
        if numbers_surrounding < 2 && number_below {
            if input[i + WIDTH..i + WIDTH + 1].contains('.')
                && input[i + WIDTH - 1..i + WIDTH + 2]
                    .chars()
                    .filter(|ch| ch.is_ascii_digit())
                    .count()
                    == 2
            {
                numbers_surrounding += 2;
                temp_result *=
                    expand_index(i + WIDTH - 1, input) * expand_index(i + WIDTH + 1, input)
            } else {
                numbers_surrounding += 1;
                // From left to right
                let mut start_pos = i + WIDTH - 1;
                start_pos += input[start_pos..start_pos + 3]
                    .as_bytes()
                    .iter()
                    .position(|a| a.is_ascii_digit())
                    .unwrap();

                temp_result *= expand_index(start_pos, input);
            }
        }

        if numbers_surrounding == 2 {
            result += temp_result;
        }
    }
    result
}
