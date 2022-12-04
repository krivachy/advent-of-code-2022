use std::fs;

use regex::Regex;

fn main() {
    let contents = fs::read_to_string("src/day04/input.txt").expect("failed to open input.txt");
    let result: u32 = contents.lines().map(|line_str| {
        let line = parse_input_line(line_str);
        if range_within_range(&line.left, &line.right) || range_within_range(&line.right, &line.left) {
            1
        } else {
            0
        }
    }).sum();
    println!("Part 1: {}", result);
    let result: u32 = contents.lines().map(|line_str| {
        let line = parse_input_line(line_str);
        if range_overlaps_range(&line.left, &line.right) || range_overlaps_range(&line.right, &line.left) {
            1
        } else {
            0
        }
    }).sum();
    println!("Part 2: {}", result);
}

struct Range {
    start: u32,
    end: u32,
}

struct Line {
    left: Range,
    right: Range,
}

fn parse_input_line(str: &str) -> Line {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut groups = Vec::new();
    for cap in re.captures_iter(str) {
        groups.push(cap[1].to_string());
        groups.push(cap[2].to_string());
        groups.push(cap[3].to_string());
        groups.push(cap[4].to_string());
    }
    Line {
        left: Range {
            start: groups[0].parse().unwrap(),
            end: groups[1].parse().unwrap(),
        },
        right: Range {
            start: groups[2].parse().unwrap(),
            end: groups[3].parse().unwrap(),
        },
    }
}

fn range_within_range(range: &Range, range2: &Range) -> bool {
    range.start >= range2.start && range.end <= range2.end
}

fn range_overlaps_range(range: &Range, range2: &Range) -> bool {
    range.start <= range2.start && range.end >= range2.start
}