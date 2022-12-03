use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn find_max(calories: Vec<u64>) {
    let top1_calories = calories.get(0);
    let top2_calories = calories.get(1);
    let top3_calories = calories.get(2);
    match top1_calories {
        Some(max_calories) => println!("Elf with max calories has {} calories", max_calories),
        None => println!("Uhm, dunno"),
    }

    match (top1_calories, top2_calories, top3_calories) {
        (Some(top1), Some(top2), Some(top3)) => {
            println!("Top 3 have {} in total", top1 + top2 + top3)
        }
        (_, _, _) => println!("Not enough elves to determine top 3"),
    }
}

fn process(file: File) {
    let lines = BufReader::new(file).lines();
    let mut elf_calories: Vec<u64> = Vec::new();
    let mut current_calories = 0;
    for line in lines {
        if let Ok(content) = line {
            match content.parse::<u64>() {
                Err(_) => {
                    elf_calories.push(current_calories);
                    current_calories = 0;
                }
                Ok(num) => {
                    current_calories = current_calories + num;
                }
            }
        }
    }
    elf_calories.push(current_calories);
    elf_calories.sort();
    elf_calories.reverse();
    find_max(elf_calories);
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
