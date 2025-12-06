use std::{fs, path::PathBuf};

struct Entry {
    name: String,
    size: i64,
    entry_type: EntryType,
}

enum EntryType {
    File,
    Dir,
}

fn main() {
    let input = fs::read_to_string("../input/22/day7.txt").unwrap();

    let mut entries: Vec<Entry> = Vec::new();

    let mut current_path = "/".to_string();
    entries.push(Entry {
        name: current_path.to_string(),
        size: 0,
        entry_type: EntryType::Dir,
    });

    // parse input
    for line in input.lines() {
        // command
        if line.starts_with("$ ") {
            match &line[2..4] {
                "cd" => {
                    // construct new current_path String
                    let path = &line[5..];
                    match path {
                        "/" => current_path = "/".to_string(),
                        ".." => {
                            current_path = PathBuf::from(&current_path)
                                .parent()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string()
                        }
                        _ => {
                            current_path = create_full_path(&current_path, path);
                        }
                    }
                }
                "ls" => {
                    // ignore :)
                }
                _ => panic!("unknown command: {}", line),
            }
            continue;
        }
        // listing of a directory
        if line.starts_with("dir ") {
            let name = &line[4..];
            let full_path = create_full_path(&current_path, name);
            entries.push(Entry {
                name: full_path,
                size: 0,
                entry_type: EntryType::Dir,
            });
            continue;
        }

        // listing of a file
        let mut split = line.split_whitespace();
        let size = split.next().unwrap().parse().unwrap();
        let name = split.next().unwrap().to_string();

        let full_path = create_full_path(&current_path, &name);
        entries.push(Entry {
            name: full_path,
            size,
            entry_type: EntryType::File,
        });
    }

    // for entry in &entries {
    //     print_entry(entry);
    // }

    // calculate directory sizes
    let mut directories = Vec::new();
    for entry in &entries {
        match entry.entry_type {
            EntryType::File => (),
            EntryType::Dir => {
                let size = get_dir_size(&entry, &entries);
                directories.push(Entry {
                    name: entry.name.to_string(),
                    size,
                    entry_type: EntryType::Dir,
                });
            }
        }
    }

    // println!();
    // for dir in &directories {
    //     print_entry(&dir);
    // }

    // show only the ones with size less than 100000
    println!();
    let result: i64 = directories
        .iter()
        .filter(|d| d.size <= 100000)
        .map(|d| {
            print_entry(d);
            d.size
        })
        .sum();
    println!(" => {}", result);
}

fn print_entry(entry: &Entry) {
    let t = match entry.entry_type {
        EntryType::File => "File",
        EntryType::Dir => "Dir",
    };
    println!("{} -> {} -> {}", entry.name, t, entry.size);
}

fn get_dir_size(entry: &Entry, entries: &Vec<Entry>) -> i64 {
    match entry.entry_type {
        EntryType::File => 0,
        EntryType::Dir => entries
            .iter()
            .filter(|e| match e.entry_type {
                EntryType::File => true,
                EntryType::Dir => false,
            })
            .filter(|e| e.name.starts_with(&entry.name) && e.name.len() > entry.name.len() + 1)
            .map(|e| match e.entry_type {
                EntryType::File => e.size,
                EntryType::Dir => get_dir_size(e, entries),
            })
            .sum(),
    }
}

fn create_full_path(current_path: &String, name: &str) -> String {
    let full_path = if "/".eq(current_path) {
        format!("/{}", &name)
    } else {
        format!("{}/{}", &current_path, &name)
    };
    full_path
}
