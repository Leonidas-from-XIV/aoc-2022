use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use core::slice::Iter;

#[derive(Debug)]
enum Fs {
    FileEntry(String, u64),
    Dir(String, Vec<Fs>),
}

#[derive(Debug)]
enum Line {
    Cd(String),
    FileEntry(String, u64),
}

fn parse_command(l: String) -> Option<Line> {
    if l.starts_with("$ ls") {
        None
    } else if l.starts_with("$ cd") {
        let mut chunks = l.rsplitn(2, " ");
        let folder = chunks.next().unwrap().to_string();
        Some(Line::Cd(folder))
    } else if l.starts_with("dir") {
        None
    } else {
        let mut chunks = l.splitn(2, " ");
        let size : u64 = chunks.next().unwrap().parse().unwrap();
        let name = chunks.next().unwrap().to_string();
        Some(Line::FileEntry(name, size))
    }
}

fn fs_from_commands(cmds: Vec<Line>) -> Fs {

    fn aux(cwd : String, mut cmds: Iter<Line>) -> (Fs, Iter<Line>) {
        let mut current_entries : Vec<Fs> = Vec::new();

        loop {
            match cmds.next() {
                Some(Line::Cd(dir)) => {
                    if dir == ".." {
                        let fs = Fs::Dir(cwd, current_entries);
                        return (fs, cmds);
                    } else {
                        let (fs, cmds_new) = aux(dir.to_string(), cmds);
                        cmds = cmds_new;
                        current_entries.push(fs);
                    }
                },
                Some(Line::FileEntry(name, size)) => {
                    let fs = Fs::FileEntry(name.to_string(), *size);
                    current_entries.push(fs);
                }

                None => {
                    break;
                },
            }
        }
        let fs = Fs::Dir(cwd, current_entries);
        (fs, cmds)
    }

    let (fs, _) = aux("foo".to_string(), cmds.iter());
    match fs {
        Fs::Dir(_, mut entries) => {
            entries.pop().unwrap()
        },
        Fs::FileEntry(_, _) => panic!("broken"),
    }
}

fn fs_size(fs: &Fs) -> u64 {
    match fs {
        Fs::FileEntry(_, s) => *s,
        Fs::Dir(_, entries) => {
            entries.iter().map(|e| fs_size(e)).sum()
        }
    }
}

fn find_at_most(size: u64, fs: &Fs) -> Vec<&Fs> {
    let mut found : Vec<&Fs> = Vec::new();
    match fs {
        Fs::FileEntry(_, _) => (),
        Fs::Dir(_, entries) => {
            if fs_size(fs) <= size {
                found.push(fs);
            }
            for e in entries {
              for find in find_at_most(size, e) {
                  found.push(find)
              }
            }
        }
    }
    found
}

fn find_at_least(size: u64, fs: &Fs) -> Vec<&Fs> {
    let mut found : Vec<&Fs> = Vec::new();
    match fs {
        Fs::FileEntry(_, _) => (),
        Fs::Dir(_, entries) => {
            if fs_size(fs) >= size {
                found.push(fs);
            }
            for e in entries {
              for find in find_at_least(size, e) {
                  found.push(find)
              }
            }
        }
    }
    found
}

fn process(file: File) {
    let lines = BufReader::new(file).lines();
    let mut commands : Vec<Line> = Vec::new();
    for line in lines {
        if let Ok(content) = line {
            let cmd = parse_command(content);
            match cmd {
                None => (),
                Some(v) => commands.push(v),
            }
        }
    }
    let fs = fs_from_commands(commands);
    println!("Entries: {:?}", &fs);
    println!("Size of all: {}", fs_size(&fs));
    let at_most : u64 = 100000;
    let small_fs = find_at_most(at_most, &fs);
    let total_small : u64 = small_fs.iter().map(|e| fs_size(e)).sum();
    println!("Total size of dirs of at most {} size: {}", at_most, total_small);

    let total : u64 = 70000000;
    let free = total - fs_size(&fs);
    let free_required : u64 = 30000000;
    let need_to_free = free_required - free;
    println!("Need to free {}", need_to_free); 
    let mut larger_dirs = find_at_least(need_to_free, &fs);
    println!("Larger dirs {:?}", larger_dirs);
    larger_dirs.sort_by(|a, b| fs_size(b).cmp(&fs_size(a)));
    let smallest_fitting = larger_dirs.pop().unwrap();
    println!("Delete {:?}, size {}", smallest_fitting, fs_size(smallest_fitting));
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

