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

fn hashset_from_content(content: Vec<char>) -> HashSet<char> {
    HashSet::from_iter(content.iter().cloned())
}

struct Group3 {
    inner: std::io::Lines<BufReader<File>>
}

struct Group3Iter {
    inner: Group3
}

impl Group3 {
    fn iter(self) -> Group3Iter {
        Group3Iter {inner: self}
    }
}

impl Iterator for Group3Iter {
    // type Item = &'a It;
    type Item = (HashSet<char>, HashSet<char>, HashSet<char>);

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.inner.inner.next();
        let b = self.inner.inner.next();
        let c = self.inner.inner.next();
        match (a, b, c) {
            (Some(Ok(a)), Some(Ok(b)), Some (Ok(c))) => {
                let a_h = hashset_from_content(a.chars().collect());
                let b_h = hashset_from_content(b.chars().collect());
                let c_h = hashset_from_content(c.chars().collect());
                Some((a_h, b_h, c_h))
                 }
            (_, _, _) => None
        }
    }
}

fn group3(file: File) {
    let grouped : Group3 = Group3 {inner: BufReader::new(file).lines()};
    // let mut iter = e.iter();
    // iter.next().unwrap()
    let mut total : u64 = 0;
    for (a, b, c) in grouped.iter() {
        let inter = a.intersection(&b);
        for common in inter {
            match c.contains(&common) {
                false => (),
                true => {
                    total += priority(*common);
                },
            }
        }
    }
    println!("Total 3 groups: {}", total);

}

fn _process(file: File) {
    let lines = BufReader::new(file).lines();
    let mut total : u64 = 0;
    for line in lines {
        if let Ok(content) = line {
            let capacity = content.len();
            let part1 : Vec<char> = content[0..(capacity/2)].chars().collect();
            let part2 : Vec<char> = content[(capacity/2)..capacity].chars().collect();
            // let compartment1: HashSet<char> = HashSet::from_iter(part1.iter().cloned());
            let compartment1: HashSet<char> = hashset_from_content(part1);
            let compartment2: HashSet<char> = hashset_from_content(part2);
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
                // Ok(file) => _process(file),
                Ok(file) => group3(file),
            }
        }
    }
}

