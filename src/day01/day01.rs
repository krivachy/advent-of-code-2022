extern crate core;

use std::fs;

struct Accumulator {
    top3: [i32; 3],
    current: i32,
}

fn main() {
    let contents = fs::read_to_string("src/day01/input.txt").expect("failed to open input.txt");
    let lines = contents.lines();
    let result = lines.fold(Accumulator {
        top3: [0, 0, 0],
        current: 0,
    }, |acc: Accumulator, next: &str| {
        if next == "" {
            let mut top4 = vec![acc.current];
            top4.extend_from_slice(&acc.top3);
            top4.sort();
            top4.reverse();
            top4.truncate(3);
            Accumulator {
                top3: <[i32; 3]>::try_from(top4).unwrap(),
                current: 0,
            }
        } else {
            Accumulator {
                top3: acc.top3,
                current: acc.current + next.parse::<i32>().unwrap(),
            }
        }
    });
    println!("Answer 1: {}", result.top3[0]);
    let sum: i32 = result.top3.iter().sum();
    println!("Answer 2: {}", sum);
}