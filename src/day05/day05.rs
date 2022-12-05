use std::fs;

use regex::Regex;

fn main() {
    let contents = fs::read_to_string("src/day05/input.txt").expect("failed to open input.txt");
    // Input table:
    //     [H]         [D]     [P]
    // [W] [B]         [C] [Z] [D]
    // [T] [J]     [T] [J] [D] [J]
    // [H] [Z]     [H] [H] [W] [S]     [M]
    // [P] [F] [R] [P] [Z] [F] [W]     [F]
    // [J] [V] [T] [N] [F] [G] [Z] [S] [S]
    // [C] [R] [P] [S] [V] [M] [V] [D] [Z]
    // [F] [G] [H] [Z] [N] [P] [M] [N] [D]
    //  1   2   3   4   5   6   7   8   9
    let initial_containers: [Vec<char>; 9] = [
        vec!['F', 'C', 'J', 'P', 'H', 'T', 'W'],
        vec!['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'],
        vec!['H', 'P', 'T', 'R'],
        vec!['Z', 'S', 'N', 'P', 'H', 'T'],
        vec!['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'],
        vec!['P', 'M', 'G', 'F', 'W', 'D', 'Z'],
        vec!['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'],
        vec!['N', 'D', 'S'],
        vec!['D', 'Z', 'S', 'F', 'M'],
    ];

    let result = contents.lines().skip(10).fold(initial_containers.clone(), |mut acc, line_str| {
        let line = parse_line(&line_str);
        move_containers_with_command(&mut acc, &line);
        acc
    });

    // Print out the joined together last character in each container array:
    println!("Part 1: {}", result.iter().map(|container| container.last().unwrap()).collect::<String>());

    // Skip first 10 lines of input and fold over the rest of the lines:
    let result = contents.lines().skip(10).fold(initial_containers.clone(), |mut acc, line_str| {
        let line = parse_line(&line_str);
        slice_containers_with_command(&mut acc, &line);
        acc
    });
    // Print out the joined together last character in each container array:
    println!("Part 2: {}", result.iter().map(|container| container.last().unwrap()).collect::<String>());
}

struct Command {
    count: u32,
    from: usize,
    to: usize,
}

// Use regex to parse a line like:
// move 2 from 8 to 2
// into a the Move struct
fn parse_line(line: &str) -> Command {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut groups = Vec::new();
    for cap in re.captures_iter(line) {
        groups.push(cap[1].to_string());
        groups.push(cap[2].to_string());
        groups.push(cap[3].to_string());
    }
    Command {
        count: groups[0].parse().unwrap(),
        from: groups[1].parse().unwrap(),
        to: groups[2].parse().unwrap(),
    }
}

fn move_containers_with_command(containers: &mut [Vec<char>], command: &Command) {
    let from = command.from - 1;
    let to = command.to - 1;
    let mut count = command.count;
    while count > 0 {
        let container = containers[from].pop().unwrap();
        containers[to].push(container);
        count -= 1;
    }
}

// Function to slice count number of containers from the end of "from" array to the end of "to" array:
fn slice_containers_with_command(containers: &mut [Vec<char>], command: &Command) {
    let from = command.from - 1;
    let to = command.to - 1;
    let count = command.count;
    let slice = containers[from].split_off(containers[from].len() - count as usize);
    containers[to].extend(slice);
}

