use std::vec;

use itertools::Itertools;

#[derive(Debug)]
struct Mapping {
    start: u32,
    end: u32,
    leap: i32,
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Mapping {
    fn init(line: &str) -> Self {
        let (dest, source, span) = line
            .split_ascii_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect_tuple()
            .unwrap();
        Self {
            start: source,
            end: source + span - 1,
            leap: (dest - source) as i32,
        }
    }

    fn map(&self, num: u32) -> Option<u32> {
        if num <= self.end && num >= self.start {
            Some(num + self.leap as u32)
        } else {
            None
        }
    }
}

impl Range {
    fn map(self, mappings: &Vec<Mapping>) -> Vec<Range> {
        let mut result = vec![];
        let mapping = mappings.iter().find(|mapping| self.overlap(mapping));
        if let Some(mapping) = mapping {
            let ranges = self.map_overlapped_range(mapping);
            result.push(ranges.0);
            if let Some(value) = ranges.1 {
                result.append(&mut value.map(mappings));
            }
            result
        } else {
            vec![self]
        }
    }

    fn overlap(&self, mapping: &Mapping) -> bool {
        (self.end <= mapping.end && self.end >= mapping.start)
            || (self.start >= mapping.start && self.start <= mapping.end)
    }
    fn fully_contained_in_mapping(&self, mapping: &Mapping) -> bool {
        (self.end <= mapping.end && self.end >= mapping.start)
            && (self.start >= mapping.start && self.start <= mapping.end)
    }

    fn map_overlapped_range(self, mapping: &Mapping) -> (Range, Option<Range>) {
        let mut result_range: Range = Range { start: 0, end: 0 };
        let mut leftover_range: Option<Range> = None;
        if self.fully_contained_in_mapping(mapping) {
            result_range = Range {
                start: self.start + mapping.leap as u32,
                end: self.end + mapping.leap as u32,
            };
        } else if self.end <= mapping.end && self.start < mapping.start {
            result_range = Range {
                start: mapping.start + mapping.leap as u32,
                end: self.end + mapping.leap as u32,
            };
            leftover_range = Some(Range {
                start: self.start,
                end: mapping.start - 1,
            });
        } else if self.start >= mapping.start && self.end > mapping.end {
            result_range = Range {
                start: self.start + mapping.leap as u32,
                end: mapping.end + mapping.leap as u32,
            };
            leftover_range = Some(Range {
                start: mapping.end + 1,
                end: self.end,
            });
        }
        (result_range, leftover_range)
    }
}

#[aoc::main(05)]
fn main() -> () {}

fn part1(input: &str) -> u32 {
    let (seeds_line, input) = input.split_once("\n\n").unwrap();
    let mut seeds: Vec<u32> = seeds_line["seeds: ".len()..]
        .split_ascii_whitespace()
        .filter_map(|a| a.parse::<u32>().ok())
        .collect();
    for section in input.split("\n\n") {
        let (_header, mapping_lines) = section.split_once('\n').unwrap();
        let mappings: Vec<Mapping> = mapping_lines.split('\n').map(Mapping::init).collect();

        for seed in seeds.iter_mut() {
            let value = mappings.iter().find_map(|mapping| mapping.map(*seed));
            if let Some(new_value) = value {
                *seed = new_value;
            }
        }
    }
    seeds
        .into_iter()
        .min()
        .expect("Some value should have been found")
}

fn part2(input: &str) -> u32 {
    let (seeds_line, input) = input.split_once("\n\n").unwrap();
    let mut seeds: Vec<Range> = vec![];
    let mut it = seeds_line["seeds: ".len()..]
        .split_ascii_whitespace()
        .filter_map(|a| a.parse::<u32>().ok());
    while let Some(left) = it.next() {
        let span = it.next().expect("Not pair values");
        seeds.push(Range {
            start: left,
            end: left + span - 1,
        });
    }
    for section in input.split("\n\n") {
        let mut temp_values = vec![];
        let (_header, mapping_lines) = section.split_once('\n').unwrap();
        let mappings: Vec<Mapping> = mapping_lines.split('\n').map(Mapping::init).collect();

        for seed in seeds.into_iter() {
            temp_values.push(seed.map(&mappings));
        }

        seeds = temp_values.into_iter().flatten().collect();
    }

    seeds
        .into_iter()
        .map(|a| a.start)
        .min()
        .expect("Some value should have been found")
}
