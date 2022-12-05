use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn parse_stack_line(l: String) -> Vec<Option<char>> {
    let mut collection = Vec::<Option<char>>::new();

    for chunk in l.chars().collect::<Vec<char>>().chunks(4) {
        let mut iter = chunk.iter();
        let _ = iter.next();
        let maybe_char = iter.next().unwrap();
        let v = match maybe_char {
            ' ' => None,
            char => Some(char)
        };
        collection.push(v.copied());
    }
    collection
}

struct Move {amount: u64, from: usize, to: usize}

fn parse_move(l: String) -> Move {
    let mut pieces = l.split_whitespace();
    let _move = pieces.next();
    let amount : u64 = pieces.next().unwrap().parse().unwrap();
    let _from = pieces.next();
    let from : usize = pieces.next().unwrap().parse().unwrap();
    let _to = pieces.next();
    let to : usize = pieces.next().unwrap().parse().unwrap();

    Move { amount, from, to }
}

fn construct_stacks(stack_lines: Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    let mut stacks : Vec<Vec<char>> = Vec::new();
    for _ in &stack_lines[0] {
        let stack: Vec<char> = Vec::new();
        stacks.push(stack);
    }
    for line in stack_lines {
        for (index, stack_value) in line.iter().enumerate() {
            match stack_value {
                Some(v) => {
                    stacks[index].push(*v);
                },
                None => ()
            }
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    stacks
}

fn perform_move(stacks: &mut Vec<Vec<char>>, mov: &Move) {
    let stack = &mut stacks[mov.from - 1];
    let mut cache : Vec<char> = Vec::new();
    for _ in 0..mov.amount {
        let v = stack.pop().unwrap();
        cache.push(v);
    }
    // println!("Attempting to move {:?}", cache);

    let stack = &mut stacks[mov.to - 1];
    for v in cache {
        stack.push(v);
    }
}

fn peek_tops(stacks: &Vec<Vec<char>>) -> Vec<char> {
    stacks.iter().map(|stack|
                      match stack.last() {
                          None => '?',
                          Some(c) => *c,
                      }).collect()
}

fn peek_tops_id(stacks: &Vec<Vec<char>>) -> String {
    let tops = peek_tops(stacks);
    tops.iter().cloned().collect()
}

fn process(file: File) {
    let lines = BufReader::new(file).lines();
    let mut stack_lines: Vec<Vec<Option<char>>> = Vec::new();
    let mut moves : Vec<Move> = Vec::new();
    for line in lines {
        if let Ok(content) = line {
            if content.is_empty() {
                continue;
            }
            else if &content[0..1] == "[" {
                let l = parse_stack_line(content);
                stack_lines.push(l);
            }
            else if &content[0..1] == " " {
                continue;
            }
            else {
                let m = parse_move(content);
                moves.push(m);
            }
        }
    }
    // stack_lines.reverse();
    let mut stacks = construct_stacks(stack_lines);
    // println!("Initial tops: {}", peek_tops_id(&stacks));

    for mov in moves {
        perform_move(&mut stacks, &mov);
        // println!("Intermediate tops: {}", peek_tops_id(&stacks));
    }
    println!("Final tops: {}", peek_tops_id(&stacks));
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

