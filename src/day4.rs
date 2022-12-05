use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashSet;

fn parse_start_end(s: &str) -> (u64, u64) {
    let mut splitter = s.splitn(2, '-');
    let start = splitter.next().unwrap();
    let end = splitter.next().unwrap();
    let start_num = start.parse::<u64>().unwrap();
    let end_num = end.parse::<u64>().unwrap();
    (start_num, end_num)
}

fn range_to_set((start, end): (u64, u64)) -> HashSet<u64> {
    (start..=end).into_iter().collect()
}

fn _are_subsets(left: HashSet<u64>, right: HashSet<u64>) -> bool {
    let left_is_subset = left.is_subset(&right);
    let right_is_subset = right.is_subset(&left);
    left_is_subset || right_is_subset
}

fn have_subsets(left: HashSet<u64>, right: HashSet<u64>) -> bool {
    !(left.is_disjoint(&right))
}

fn process(file: File) {
    let lines = BufReader::new(file).lines();
    let mut total : u64 = 0;
    for line in lines {
        if let Ok(content) = line {
            let mut splitter = content.splitn(2, ',');
            let elf1 = splitter.next().unwrap();
            let elf1_range = parse_start_end(elf1);
            let elf1_set = range_to_set(elf1_range);
            let elf2 = splitter.next().unwrap();
            let elf2_range = parse_start_end(elf2);
            let elf2_set = range_to_set(elf2_range);
            // let total_overlap = _are_subsets(elf1_set, elf2_set);
            let total_overlap = have_subsets(elf1_set, elf2_set);
            total += match total_overlap { true => 1, false => 0};
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
