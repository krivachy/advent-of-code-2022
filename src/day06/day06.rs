use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("src/day06/input.txt").expect("failed to open input.txt");
    let mut last_four_chars = input.chars().take(4).collect_vec();
    let mut count = 4;
    while !vector_contains_unique_chars(&last_four_chars) {
        last_four_chars.push(input.chars().nth(count).unwrap());
        last_four_chars.remove(0);
        count += 1;
    }
    println!("Part 1: {}", count);
    let mut last_fourteen_chars = input.chars().take(14).collect_vec();
    let mut count2 = 14;
    while !vector_contains_unique_chars(&last_fourteen_chars) {
        last_fourteen_chars.push(input.chars().nth(count2).unwrap());
        last_fourteen_chars.remove(0);
        count2 += 1;
    }
    println!("Part 2: {}", count2);
}

fn vector_contains_unique_chars(v: &Vec<char>) -> bool {
    let mut set = std::collections::HashSet::new();
    for c in v {
        if set.contains(c) {
            return false;
        }
        set.insert(c);
    }
    true
}