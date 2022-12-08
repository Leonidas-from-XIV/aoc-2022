use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::cmp::Ordering;

fn _visible(line: &Vec<u32>) -> Vec<bool> {
    let mut visible : Vec::<bool> = Vec::new();
    let mut biggest = line.get(0).unwrap();
    let halfway = line.len() / 2;

    visible.push(true);

    for tree in &line[1..halfway] {
        if tree > biggest {
            visible.push(true);
        } else {
            visible.push(false);
        }

        if tree > biggest {
            biggest = tree;
        }
    }

    let mut right_visible : Vec::<bool> = Vec::new();
    let mut biggest = line.last().unwrap();
    right_visible.push(true);

    for tree in line[halfway+1..].iter().rev() {
        println!("Tree: {}", tree);
        if tree > biggest {
            right_visible.push(true);
        } else {
            right_visible.push(false);
        }

        if tree > biggest {
            biggest = tree;
        }
    }
    right_visible.reverse();
    for v in right_visible {
        visible.push(v);
    }
    println!("XXX");
    
    visible
}

fn exchange(x: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    (0..x[0].len()).map(|i| x
        .iter()
        .map(|c| c[i])
        .collect()
      ).collect() 
}

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
    let left : Vec<&u32> = line[0..y].iter().collect();
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
    (above, below)
}

fn visible_from_direction(tree: u32, direction: &Vec<&u32>) -> bool {
    direction.iter().all(|enemy| *enemy < &tree)
}

fn visible(tree: u32, left: &Vec<&u32>, right: &Vec<&u32>, top: &Vec<&u32>, bottom: &Vec<&u32>) -> bool {
    visible_from_direction(tree, left) || visible_from_direction(tree, right) || visible_from_direction(tree, top) || visible_from_direction(tree, bottom)
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

