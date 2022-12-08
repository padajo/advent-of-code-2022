use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct ElfFs {
    dirs: HashMap<String, Dir>,
}

impl ElfFs {
    fn new() -> ElfFs {
        ElfFs {
            dirs: HashMap::new(),
        }
    }

    fn get_dir_size(&self, path: String) -> u32 {
        let mut size: u32 = 0;
        if self.dirs.contains_key(&path) {
            let dir: &Dir = self.dirs.get(&path).unwrap();
            size += dir.size();
            for f in dir.files.iter() {
                if f.filetype.eq("dir") {
                    size += self.get_dir_size(format!("{}/{}", path, f.name));
                }
            }
        }
        size
    }
}

struct Dir {
    path: String,
    files: Vec<ElfFile>,
}

impl Dir {
    fn new(path: String) -> Dir {
        Dir {
            path: path,
            files: Vec::new(),
        }
    }

    fn size(&self) -> u32 {
        let mut size: u32 = 0;
        for f in self.files.iter() {
            size += f.size;
        }
        size
    }
}

#[derive(Debug)]
struct ElfFile {
    name: String,
    size: u32,        // is 0 for dir
    filetype: String, // file or dir
}

fn main() {
    println!("Advent of code 2022, Day 7 Part 1");

    let mut wd: Vec<String> = Vec::new();
    // let mut path: String = String::new();
    // let mut dirs: HashMap<String, Dir> = HashMap::new(); // ref by path (wd)
    let mut elffs: ElfFs = ElfFs::new();

    let root_path = "/".to_string();
    elffs
        .dirs
        .insert(root_path.clone(), Dir::new(root_path.clone()));
    let mut current_dir: &mut Dir = elffs.dirs.get_mut(&root_path).unwrap();
    let mut read_data: bool = false;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            let s: String = Ok::<String, String>(line.unwrap()).unwrap();
            let data: Vec<&str> = s.split(' ').collect::<Vec<&str>>();
            if data[0].eq("$") {
                read_data = false;
                // println!("Command: {}", s);
                if data[1].eq("cd") {
                    if data[2].eq("..") {
                        wd.pop();
                    } else if data[2].eq("/") {
                        wd.clear();
                    } else {
                        wd.push(data[2].to_string());
                    }
                    let path = format!("/{}", wd.join("/"));
                    // println!("New path: {}", path);
                    if !elffs.dirs.contains_key(&path) {
                        elffs.dirs.insert(path.clone(), Dir::new(path.clone()));
                    }
                    current_dir = elffs.dirs.get_mut(&path).unwrap();
                } else if data[1].eq("ls") {
                    read_data = true;
                } else {
                    println!("Unknown command");
                }
            } else {
                if read_data {
                    if data[0].eq("dir") {
                        // it's a directory
                        current_dir.files.push(ElfFile {
                            name: data[1].to_string(),
                            size: 0,
                            filetype: "dir".to_string(),
                        });
                    } else {
                        // it's a file
                        current_dir.files.push(ElfFile {
                            name: data[1].to_string(),
                            size: data[0].parse::<u32>().unwrap(),
                            filetype: "file".to_string(),
                        });
                    }
                } else {
                    println!("Unknown data");
                }
            }
        }
    }

    let mut all_dirs: Vec<&String> = elffs.dirs.keys().collect();
    all_dirs.sort();

    let mut dir_sum: u32 = 0;

    for d_path in all_dirs.iter() {
        let d: &Dir = elffs.dirs.get(d_path.clone()).unwrap();
        let b: i64 = d.files.iter().map(|f| f.size as i64).sum();
        if b < 100_000 {
            let rec_size = elffs.get_dir_size(d_path.to_string());
            if rec_size < 100_000 {
                println!("{} ({})", d_path, b);
                println!("-> {}", rec_size);
                dir_sum += rec_size;
            }
        }
    }

    println!("\nTotal size of all directories: {}", dir_sum);
}
