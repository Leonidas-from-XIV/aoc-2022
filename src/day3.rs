use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashSet;

fn priority(c: char) -> u64 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("That's unamerican and unpartiotic"),
    }
}

fn process(file: File) {
    let lines = BufReader::new(file).lines();
    let mut total : u64 = 0;
    for line in lines {
        if let Ok(content) = line {
            let capacity = content.len();
            let part1 : Vec<char> = content[0..(capacity/2)].chars().collect();
            let part2 : Vec<char> = content[(capacity/2)..capacity].chars().collect();
            let compartment1: HashSet<char> = HashSet::from_iter(part1.iter().cloned());
            let compartment2: HashSet<char> = HashSet::from_iter(part2.iter().cloned());
            let common = compartment1.intersection(&compartment2);
            let mut common_prio : u64 = 0;
            for item in common {
                let prio = priority(*item);
                common_prio = common_prio + prio;
            }
            total = total + common_prio;
        }
    }
    println!("Total is {}", total);
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

