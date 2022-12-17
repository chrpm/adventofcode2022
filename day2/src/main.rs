use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut total_score = 0;
    let mut total_score_2 = 0;
    for line in lines {
        let split: Vec<&str> = line.split(" ").collect();
        total_score += score_round_first(split[0], split[1]);
        total_score_2 += score_round_second(split[0], split[1]);
    }
    println!("first {} second {}", total_score, total_score_2)
}

// The winner of the whole tournament is the player with the highest score.
// Your total score is the sum of your scores for each round.
// The score for a single round is the score for the shape you selected
// (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome
// of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

// x rock
// y paper
// z scissors

// a rock
// b paper
// c scissors

fn score_round_first(you: &str, me: &str) -> i32 {
    let mut points = 0;
    if me == "X" {
        points += 1;
        if you == "A" {
            // rock
            points += 3;
        } else if you == "B" {
            // paper
            points += 0;
        } else if you == "C" {
            // scissors
            points += 6;
        }
    } else if me == "Y" {
        points += 2;
        if you == "A" {
            // rock
            points += 6;
        } else if you == "B" {
            // paper
            points += 3;
        } else if you == "C" {
            // scissors
            points += 0;
        }
    } else if me == "Z" {
        points += 3;
        if you == "A" {
            // rock
            points += 0;
        } else if you == "B" {
            // paper
            points += 6;
        } else if you == "C" {
            // scissors
            points += 3;
        }
    }

    return points;
}
// (1 for Rock, 2 for Paper, and 3 for Scissors)
fn score_round_second(you: &str, me: &str) -> i32 {
    let mut points = 0;
    if me == "X" { // lose
        points += 0;
        if you == "A" {
            // rock
            points += 3;
        } else if you == "B" {
            // paper
            points += 1;
        } else if you == "C" {
            // scissors
            points += 2;
        }
    } else if me == "Y" { // tie
        points += 3;
        if you == "A" {
            // rock
            points += 1;
        } else if you == "B" {
            // paper
            points += 2;
        } else if you == "C" {
            // scissors
            points += 3;
        }
    } else if me == "Z" { // win
        points += 6;
        if you == "A" {
            // rock
            points += 2;
        } else if you == "B" {
            // paper
            points += 3;
        } else if you == "C" {
            // scissors
            points += 1;
        }
    }

    return points;
}