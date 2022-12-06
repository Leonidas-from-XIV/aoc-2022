use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashSet;
use std::collections::VecDeque;

const MARKER_LENGTH : usize = 14;

fn qualifies(v: &VecDeque<char>) -> bool {
    v.len() == MARKER_LENGTH
}

fn unique(v: &VecDeque<char>) -> bool {
    let hs: HashSet<&char> = v.into_iter().collect();
    hs.len() == MARKER_LENGTH
}

fn marker_location(s: String) -> usize {
    let mut lookbehind: VecDeque<char> = VecDeque::new();
    for (index, new_char) in s.chars().enumerate() {
        lookbehind.push_back(new_char);
        if qualifies(&lookbehind) {
            if unique(&lookbehind) {
                return index + 1;
            } else {
                lookbehind.pop_front();
            }
        }
    }
    0
}

fn process(file: File) {
    let lines = BufReader::new(file).lines();
    for line in lines {
        if let Ok(content) = line {
            let location = marker_location(content);
            println!("Marker location is at {}", location);
        }
    }
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

