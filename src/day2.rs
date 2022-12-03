use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum Choice {
    Rock,
    Paper,
    Scissor
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

fn parse_to_choice(s: &str) -> Choice {
    match s {
        "A" | "X" => Choice::Rock,
        "B" | "Y" => Choice::Paper,
        "C" | "Z" => Choice::Scissor,
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
            let my = splitter.next().unwrap();
            let my_choice = parse_to_choice(my);
            let outcome = score(opponent_choice, my_choice);
            total = total + outcome;
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
