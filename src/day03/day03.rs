use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("src/day03/input.txt").expect("failed to open input.txt");
    let result: u32 = contents.lines().map(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        let left_set: HashSet<char> = left.chars().collect();
        let right_set: HashSet<char> = right.chars().collect();
        let common_item_type = left_set.intersection(&right_set);
        let priority: u32 = common_item_type.map(|c| priority(*c as u32)).sum();
        priority
    }).sum();
    println!("Part 1: {}", result);
    let result2: u32 = contents.lines().chunks(3).into_iter().map(|group| {
        let hash_sets: Vec<HashSet<char>> = group.map(|g| g.chars().collect()).collect();
        let intersection = hash_sets
            .iter()
            .skip(1)
            .fold(hash_sets[0].clone(), |acc, next| acc.intersection(next).cloned().collect());
        let priority: u32 = intersection.into_iter().map(|c| priority(c as u32)).sum();
        priority
    }).sum();
    println!("Part 2: {}", result2);
}

fn priority(digit: u32) -> u32 {
    if digit >= 'a' as u32 {
        digit - ('a' as u32) + 1
    } else {
        digit - ('A' as u32) + 27
    }
}