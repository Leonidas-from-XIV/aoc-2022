use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Move {
    direction: Direction,
    distance: u64
}

struct Move1(Direction);

fn parse_direction(s: &str) -> Direction {
    if s == "U" { Direction::Up }
    else if s == "D" { Direction::Down }
    else if s == "L" { Direction::Left }
    else if s == "R" { Direction::Right }
    else {
        panic!("I'm having the worst day");
    }
}

fn parse_distance(s: &str) -> u64 {
    s.parse().unwrap()
}

type Loc = (i64, i64);

fn new_tail_pos(tail_pos: Loc, head_pos: Loc) -> Loc {
    let (hx, hy) = head_pos;
    let (tx, ty) = tail_pos;

    if (hx - tx).abs() <= 1 && (hy - ty).abs() <= 1 {
        // stay
        tail_pos
    } else {
        let mut rx = tx;
        let mut ry = ty;
        if hx > tx { rx += 1 }
        if hx < tx { rx -= 1 }
        if hy > ty { ry += 1 }
        if hy < ty { ry -= 1 }
        (rx, ry)
    }
}

fn new_head_pos(head_pos: Loc, dir: Move1) -> Loc {
    let (x, y) = head_pos;
    match dir.0 {
        Direction::Left => (x-1, y),
        Direction::Right => (x+1, y),
        Direction::Up => (x, y+1),
        Direction::Down => (x, y-1),
    }
}

fn process(file: File) {
    let lines = BufReader::new(file).lines();
    let mut moves : Vec<Move> = Vec::new();
    for line in lines {
        if let Ok(content) = line {
            let mut splitter = content.splitn(2, " ");
            let direction = parse_direction(splitter.next().unwrap());
            let distance = parse_distance(splitter.next().unwrap());
            let mov = Move { direction, distance };
            moves.push(mov);
        }
    }
    let mut move1s : Vec<Move1> = Vec::new();
    for mov in &moves {
        for _ in 0..(mov.distance) {
            move1s.push(Move1(mov.direction));
        }
    }
    let mut head_pos : Loc = (0, 0);
    let mut tail_pos : Loc = (0, 0);
    let mut tail_positions : Vec<Loc> = Vec::new();
    tail_positions.push(tail_pos);

    for mov1 in move1s {
        head_pos = new_head_pos(head_pos, mov1);
        tail_pos = new_tail_pos(tail_pos, head_pos);
        // println!("Head pos {:?} => Tail pos {:?}", head_pos, tail_pos);
        tail_positions.push(tail_pos);
    }
   
    let tail_pos_set : HashSet<Loc> = HashSet::from_iter(tail_positions.iter().cloned());
    println!("Unique tail positions: {}", tail_pos_set.len());
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

