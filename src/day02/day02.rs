use std::fs;

fn main() {
    let contents = fs::read_to_string("src/day02/input.txt").expect("failed to open input.txt");
    let result: u32 = contents.lines().map(|line| {
        let mut parts = line.split(' ');
        let opponent_choice = parts.next().unwrap();
        let your_choice = parts.next().unwrap();
        shape_score(your_choice) + round_score(opponent_choice, your_choice)
    }).sum();
    println!("Answer 1: {}", result);
    let result2: u32 = contents.lines().map(|line| {
        let mut parts = line.split(' ');
        let opponent_choice = parts.next().unwrap();
        let strategy = parts.next().unwrap();
        let your_choice = strategy_choice(opponent_choice, strategy);
        shape_score(&your_choice) + round_score(opponent_choice, &your_choice)
    }).sum();
    println!("Answer 2: {}", result2);
}

const LETTER_A: u32 = 'A' as u32;
const LETTER_X: u32 = 'X' as u32;

fn shape_score(choice: &str) -> u32 {
    let choice_int = choice.chars().next().unwrap() as u32;
    choice_int - LETTER_X + 1
}

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSS: u32 = 0;
const SCORE: [[u32; 3]; 3] = [
    // X    Y    Z
    [DRAW, WIN, LOSS],   // "A"
    [LOSS, DRAW, WIN],   // "B"
    [WIN, LOSS, DRAW],   // "C"
];


fn round_score(opponent_choice: &str, your_choice: &str) -> u32 {
    let opponent_int: u32 = opponent_choice.chars().next().unwrap() as u32;
    let your_int: u32 = your_choice.chars().next().unwrap() as u32;
    SCORE[usize::try_from(opponent_int - LETTER_A).unwrap()][usize::try_from(your_int - LETTER_X).unwrap()]
}

fn strategy_choice(opponent_choice: &str, strategy: &str) -> String {
    let result = match strategy {
        "X" => LOSS,
        "Y" => DRAW,
        "Z" => WIN,
        _ => LOSS
    };
    let opponent_int: u32 = opponent_choice.chars().next().unwrap() as u32;
    let results = SCORE[usize::try_from(opponent_int - LETTER_A).unwrap()];
    let index: u32 = results.iter().position(|r| r.eq(&result)).unwrap() as u32;
    char::from_u32(index + LETTER_X).unwrap().to_string()
}