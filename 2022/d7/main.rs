use std::{collections::HashMap, slice::Iter};

fn main() {
    let input = include_str!("input.txt");
    let root = parse(input);
    let found = find_dirs_smaller_than(&root, 100000);

    println!("root: {:?}    ", root);
    println!("found: {}", found.len());

    let total_size: i32 = found.iter().map(|dir| dir.size()).sum();
    println!("part1: {}", total_size);

    let total_disk_space = 70000000;
    let required_space = 30000000;
    let used_space = root.size();
    let free_space = total_disk_space - used_space;
    let required_to_free = required_space - free_space;
    let mut dirs_that_could_make_space = find_dirs_larger_than(&root, required_to_free);
    dirs_that_could_make_space.sort_by(|a, b| a.size().partial_cmp(&b.size()).unwrap());
    println!("part2");
    println!("used: \t\t\t\t{}", used_space);
    println!("free: \t\t\t\t{}", free_space);
    println!("required to free: \t\t{}", required_to_free);
    println!(
        "smallest satisfying: \t\t{}",
        dirs_that_could_make_space[0].size()
    );
}

fn to_command_chunks(input: &str) -> Vec<Vec<&str>> {
    input
        .split("$ ")
        .map(|part| part.lines().collect::<Vec<_>>())
        .skip(1)
        .collect()
}

fn parse(input: &str) -> Dir {
    let parts: Vec<_> = to_command_chunks(input);
    let mut commands = parts.iter();
    let root = commands.next().expect("root");
    if root[0] != "cd /" {
        panic!("unexpected start: {:?}", root);
    }
    process_commands(commands, Dir::new("/"), vec![])
}

fn process_commands(mut commands: Iter<Vec<&str>>, mut cwd: Dir, mut parents: Vec<Dir>) -> Dir {
    if let Some(command) = commands.next() {
        // println!("processing: {:?}", command);
        // println!("with: \n\t{:?}\n\t{:?}", cwd, parents);
        let mut command_itr = command.iter();
        let command = command_itr.next().unwrap();
        if command.starts_with("cd") {
            let next_dir = command.split(" ").collect::<Vec<_>>()[1];
            if next_dir == ".." {
                let mut parent = parents.pop().unwrap();
                parent.add_dir(cwd);
                return process_commands(commands, parent, parents);
            } else {
                let next_cwd = Dir::new(next_dir);
                parents.push(cwd);
                return process_commands(commands, next_cwd, parents);
            }
        } else {
            // ls
            for item in command_itr {
                let file_parts: Vec<_> = item.split(" ").collect();
                if file_parts[0] == "dir" {
                    cwd.add_dir(Dir::new(file_parts[1]));
                } else {
                    cwd.add_file(File {
                        name: file_parts[1].to_string(),
                        size: file_parts[0].parse().expect("file size"),
                    });
                }
            }
            return process_commands(commands, cwd, parents);
        }
    }
    // println!("going back up stack");
    // println!("cwd:\n{:?}\n", cwd.name);

    if let Some(mut parent) = parents.pop() {
        parent.add_dir(cwd);
        process_commands(commands, parent, parents)
    } else {
        cwd
    }
}

fn find_dirs_smaller_than(dir: &Dir, size: i32) -> Vec<&Dir> {
    let mut matching: Vec<&Dir> = Vec::new();
    // println!("scanning children of {:?}: {:?}",
    //     dir.name,
    //     dir.child_dirs.values().map(|c|&c.name).collect::<Vec<_>>());

    for child in dir.child_dirs.values() {
        if child.size() <= size {
            matching.push(child);
        }
        matching.extend(find_dirs_smaller_than(child, size));
    }

    matching
}

fn find_dirs_larger_than(dir: &Dir, size: i32) -> Vec<&Dir> {
    let mut matching: Vec<&Dir> = Vec::new();
    // println!("scanning children of {:?}: {:?}",
    //     dir.name,
    //     dir.child_dirs.values().map(|c|&c.name).collect::<Vec<_>>());

    for child in dir.child_dirs.values() {
        if child.size() >= size {
            matching.push(child);
        }
        matching.extend(find_dirs_larger_than(child, size));
    }

    matching
}

trait INode {
    fn size(&self) -> i32;
}

#[derive(Debug, PartialEq, Clone)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Dir {
    name: String,
    child_dirs: HashMap<String, Dir>,
    child_files: Vec<File>,
}

impl INode for File {
    fn size(&self) -> i32 {
        self.size
    }
}

impl INode for Dir {
    fn size(&self) -> i32 {
        self.child_dirs
            .iter()
            .map(|dir| dir.1 as &dyn INode)
            .chain(self.child_files.iter().map(|f| f as &dyn INode))
            .map(|inode| inode.size())
            .sum()
    }
}

impl Dir {
    fn new(name: &str) -> Dir {
        Dir {
            name: name.to_string(),
            child_dirs: HashMap::new(),
            child_files: vec![],
        }
    }

    fn add_dir(&mut self, child: Dir) {
        self.child_dirs.insert(child.name.clone(), child);
    }

    fn add_file(&mut self, child: File) {
        self.child_files.push(child);
    }
}

#[derive(Debug, PartialEq, Clone)]
struct DirState {
    dirs: Vec<String>,
    files: Vec<(i32, String)>,
}

#[test]
fn test_size() {
    let mut root = Dir::new("/");
    root.add_file(File {
        name: ".tmp".to_string(),
        size: 1000,
    });
    root.add_file(File {
        name: ".tmp1".to_string(),
        size: 50,
    });
    root.add_dir(Dir {
        name: "a".to_string(),
        child_dirs: HashMap::new(),
        child_files: vec![File {
            name: "a.tmp".to_string(),
            size: 10,
        }],
    });

    let total_size = root.size();
    assert_eq!(1060, total_size);
}

#[test]
fn test_part1_example() {
    let input = include_str!("input.example.txt");
    let root = parse(input);
    let found = find_dirs_smaller_than(&root, 100000);

    let total_size: i32 = found.iter().map(|dir| dir.size()).sum();

    assert_eq!(95437, total_size);
}

#[test]
fn test_parse_repeats() {
    let input = include_str!("example.repeat.txt");
    let root = parse(input);
    println!("\n\nwith repeats:\n{:?}", root);
}
