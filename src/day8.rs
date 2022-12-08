use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::cmp::Ordering;

fn print_vis(x: &Vec<Vec<bool>>) {
    for line in x {
        let s : String = line.iter().map(|v| match v { false => "0", true => "1" }).collect();
        println!("{}", s);
    }
}

fn print(x: &Vec<Vec<u32>>) {
    for line in x {
        let s : String = line.iter().map(|v| match v { 0 => "0", 1 => "1", 2 => "2", 3 => "3", 4 => "4", 5 => "5", 6 => "6", 7 => "7", 8 => "8", 9 => "9", _ => "?"}).collect();
        println!("{}", s);
    }
}

fn horizontal_of(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> (Vec<&u32>, Vec<&u32>) {
    let line = forest.get(x).unwrap();
    let left : Vec<&u32> = line[0..y].iter().rev().collect();
    let right : Vec<&u32> = line[y+1..].iter().collect();
    (left, right)
}

fn vertical_of(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> (Vec<&u32>, Vec<&u32>) {
    let mut above : Vec<&u32> = Vec::new();
    let mut below : Vec<&u32> = Vec::new();
    for (i, line) in forest.iter().enumerate() {
        let value = line.get(y).unwrap();
        match i.cmp(&x) {
            Ordering::Less => {
                above.push(value);
            }
            Ordering::Equal => (),
            Ordering::Greater => {
                below.push(value);
            },
        };
    }
    above.reverse();
    (above, below)
}

fn visible_from_direction(tree: u32, direction: &Vec<&u32>) -> bool {
    direction.iter().all(|enemy| *enemy < &tree)
}

fn visible(tree: u32, left: &Vec<&u32>, right: &Vec<&u32>, top: &Vec<&u32>, bottom: &Vec<&u32>) -> bool {
    visible_from_direction(tree, left) || visible_from_direction(tree, right) || visible_from_direction(tree, top) || visible_from_direction(tree, bottom)
}

fn view_distance_from_direction(tree: u32, direction: &Vec<&u32>) -> u64 {
    let mut distance : u64 = 0;
    for e in direction {
        distance += 1;
        if *e >= &tree {
            break;
        }
    }
    distance
}

fn calc_vis(forest: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut vis : Vec<Vec<bool>> = Vec::new();
    for (x, line) in forest.iter().enumerate() {
        let mut vis_line : Vec<bool> = Vec::new();
        for (y, tree) in line.iter().enumerate() {
            let (left, right) = horizontal_of(x, y, forest);
            let (top, bottom) = vertical_of(x, y, forest);
            let vis = visible(*tree, &left, &right, &top, &bottom);
            // println!("x {} y {} tree {} left {:?} right {:?} top {:?} bottom {:?} visible {}", x, y, tree, left, right, top, bottom, vis);
            vis_line.push(vis);
        }
        vis.push(vis_line);
    }
    vis
}

fn calc_score(forest: &Vec<Vec<u32>>) -> Vec<Vec<u64>> {
    let mut vis : Vec<Vec<u64>> = Vec::new();
    for (x, line) in forest.iter().enumerate() {
        let mut vis_line : Vec<u64> = Vec::new();
        for (y, tree) in line.iter().enumerate() {
            let (left, right) = horizontal_of(x, y, forest);
            let (top, bottom) = vertical_of(x, y, forest);
            let sleft = view_distance_from_direction(*tree, &left);
            let sright = view_distance_from_direction(*tree, &right);
            let stop = view_distance_from_direction(*tree, &top);
            let sbottom = view_distance_from_direction(*tree, &bottom);
            let score = sleft * sright * stop * sbottom;
            // println!("x {} y {} tree {} left {:?} right {:?} top {:?} bottom {:?} score ({} * {} * {} * {}) = {}", x, y, tree, left, right, top, bottom, sleft, sright, stop, sbottom, score);
            vis_line.push(score);
        }
        vis.push(vis_line);
    }
    vis
}

fn process(file: File) {
    let lines = BufReader::new(file).lines();
    let mut forest : Vec<Vec<u32>> = Vec::new();
    for line in lines {
        if let Ok(content) = line {
            let tree_line : Vec<u32> = content.chars().map(|c| c.to_digit(10).unwrap()).collect();
            forest.push(tree_line);
        }
    }
    print(&forest);
    let vis = calc_vis(&forest);
    let total : u64 = vis.iter().map(|l| l.iter().map(|e| match e { true => 1, false => 0}).sum::<u64>()).sum();
    print_vis(&vis);
    println!("Visible trees: {}", total);
    let scores = calc_score(&forest);
    let max_score : u64 = *scores.iter().map(|l| l.iter().max().unwrap()).max().unwrap();
    println!("Max score: {}", max_score);
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

