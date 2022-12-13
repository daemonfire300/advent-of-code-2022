use std::{env::args, fs::read_to_string};

fn main() {
    let path = args().nth(1).unwrap_or("input.txt".to_string());
    let contents = read_to_string(path.as_str()).unwrap();
    contents.lines().reduce(|acc, e| {
        if let Some(split) = e.split_once(" ") {
        } else {
            return 0;
        }
    });
    println!("Hello, world!");
}

fn is_rock(letter: &str) -> bool {
    return letter == "A" || letter == "X";
}

fn is_paper(letter: &str) -> bool {
    return letter == "B" || letter == "Y";
}

fn is_scissor(letter: &str) -> bool {
    return letter == "C" || letter == "Z";
}

const ROCK_VALUE: u32 = 1;
const PAPER_VALUE: u32 = 2;
const SCISSOR_VALUE: u32 = 3;
const LOSS_VALUE: u32 = 0;
const DRAW_VALUE: u32 = 3;
const WIN_VALUE: u32 = 6;

fn to_score(first_letter: &str, second_letter: &str) -> u32 {
    if is_rock(first_letter) {
        if is_rock(second_letter) {
            return ROCK_VALUE + DRAW_VALUE;
        }
        if is_paper(second_letter) {
            return ROCK_VALUE + LOSS_VALUE;
        }
        if is_scissor(second_letter) {
            return ROCK_VALUE + WIN_VALUE;
        }
    }
    if is_paper(first_letter) {
        if is_rock(second_letter) {
            return PAPER_VALUE + WIN_VALUE;
        }
        if is_paper(second_letter) {
            return PAPER_VALUE + DRAW_VALUE;
        }
        if is_scissor(second_letter) {
            return PAPER_VALUE + LOSS_VALUE;
        }
    }
    if is_scissor(first_letter) {
        if is_rock(second_letter) {
            return SCISSOR_VALUE + LOSS_VALUE;
        }
        if is_paper(second_letter) {
            return SCISSOR_VALUE + WIN_VALUE;
        }
        if is_scissor(second_letter) {
            return SCISSOR_VALUE + DRAW_VALUE;
        }
    }
    return 0;
}
