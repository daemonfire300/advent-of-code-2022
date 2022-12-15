use std::{env::args, fs::read_to_string};

fn main() {
    problem1();
    problem2();
}

fn problem1() {
    let path = args().nth(1).unwrap_or("input.txt".to_string());
    let contents = read_to_string(path.as_str()).unwrap();
    let final_score = contents
        .lines()
        .map(|e| {
            if let Some(split) = e.split_once(" ") {
                let score = to_score(split.1, split.0);
                return score;
            } else {
                return 0;
            }
        })
        .reduce(|acc, e| return acc + e);
    println!("{:?}", final_score);
}

fn problem2() {
    let path = args().nth(1).unwrap_or("input.txt".to_string());
    let contents = read_to_string(path.as_str()).unwrap();
    let final_score = contents
        .lines()
        .map(|e| e.split_once(" "))
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .map(|split| {
            let score = to_score(convert_strategy_to_action(split.0, split.1), split.0);
            return score;
        })
        .reduce(|acc, e| return acc + e);
    println!("{:?}", final_score);
}

fn convert_strategy_to_action<'a, 'b>(
    opponent_letter: &'a str,
    strategy_letter: &'a str,
) -> &'b str {
    let idx: i8 = letter_to_idx(opponent_letter)
        .try_into()
        .unwrap_or(ACTION_SET.len().try_into().unwrap_or(2));
    let idx_move: i8;
    if is_rock(strategy_letter) {
        // lose
        idx_move = idx - 1;
    } else if is_paper(strategy_letter) {
        // draw
        idx_move = idx;
    } else {
        // win
        idx_move = idx + 1;
    }
    let chosen_letter = ACTION_SET[wrap_move(idx_move)];
    return chosen_letter;
}

fn wrap_move(idx_move: i8) -> usize {
    if idx_move < 0 {
        return 2;
    }
    if idx_move > 2 {
        return 0;
    }
    return idx_move.try_into().unwrap_or_default();
}

fn letter_to_idx(letter: &str) -> u8 {
    match letter {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!("undefined letter"),
    }
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

const ACTION_SET: [&str; 3] = ["A", "B", "C"];

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
    panic!("should not reach this code")
}
