use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissor
}

enum Outcome {
    Lose,
    Draw,
    Win
}

fn score(opponent: Choice, me: Choice) -> u64 {
    let shape_score = match me {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissor => 3
    };
    let outcome_score = match (opponent, me) {
        (Choice::Rock, Choice::Paper) | (Choice::Paper, Choice::Scissor) | (Choice::Scissor, Choice::Rock) => 6,
        (Choice::Rock, Choice::Rock) | (Choice::Paper, Choice::Paper) | (Choice::Scissor, Choice::Scissor) =>  3,
        (Choice::Rock, Choice::Scissor) | (Choice::Paper, Choice::Rock) | (Choice::Scissor, Choice::Paper) =>  0,
    };
    shape_score + outcome_score
}

fn determine_choice(opponent: Choice, outcome: Outcome) -> Choice {
    match (opponent, outcome) {
        (Choice::Rock, Outcome::Draw) | (Choice::Paper, Outcome::Lose) | (Choice::Scissor, Outcome::Win) => Choice::Rock,
        (Choice::Rock, Outcome::Win) | (Choice::Paper, Outcome::Draw) | (Choice::Scissor, Outcome::Lose) => Choice::Paper,
        (Choice::Rock, Outcome::Lose) | (Choice::Paper, Outcome::Win) | (Choice::Scissor, Outcome::Draw) => Choice::Scissor,
    }
}

fn parse_to_choice(s: &str) -> Choice {
    match s {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissor,
        _ => panic!("I'm not paid enough to do this")
    }
}

fn parse_to_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("I'm not paid enough to do this")
    }
}

fn process(file: File) {
    let lines = BufReader::new(file).lines();
    let mut total : u64 = 0;
    for line in lines {
        if let Ok(content) = line {
            let mut splitter = content.splitn(2, ' ');
            let opponent = splitter.next().unwrap();
            let opponent_choice = parse_to_choice(opponent);
            let outcome = splitter.next().unwrap();
            let expected_outcome = parse_to_outcome(outcome);
            let my_choice = determine_choice(opponent_choice, expected_outcome);
            total = total + score(opponent_choice, my_choice);
        }
    }
    println!("Total score is {}", total);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1);
    match filename {
        None => println!("No filename given, bugger off"),
        Some(filename) => {
            let path = Path::new(filename);

            match File::open(&path) {
                Err(why) => println!("couldn't open file: {}", why),
                Ok(file) => process(file),
            }
        }
    }

}
