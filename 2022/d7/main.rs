use std::collections::HashMap;


fn main() {
    let input = include_str!("input.txt");
    let root = parse(input);
    let found = find_dirs(&root, 100000);
    
    let total_size: i32 = found
        .iter()
        .map(|dir| dir.size())
        .sum();
    println!("part1: {}", total_size);
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

    let mut dir_names: Vec<&str> = Vec::new();
    let mut files: Vec<Vec<(i32, &str)>> = Vec::new();
    let mut dirs: Vec<Vec<&str>> = Vec::new();

    let mut known_dirs: HashMap<&str, Dir> = HashMap::new();

    for part in parts {
        let command = part[0];
        if command.starts_with("cd") {
            let next_dir = part[0].split(" ").collect::<Vec<_>>()[1];
            if next_dir == ".." {
                let top_dir = dir_names.pop().unwrap();
                let top_files = files.pop().unwrap();
                let top_dirs = dirs.pop().unwrap();

                let resolved_dirs: Vec<_> = top_dirs
                    .iter()
                    .map(|dir| known_dirs.remove(dir)
                        .expect(&format!("expected resolved dir: \n\t{} at \n\t{} but not in \n\t{:?}", &dir, &top_dir, &known_dirs.keys())))
                    .collect();
                let resolved_files: Vec<_> = top_files
                    .iter()
                    .map(|f| File {
                        name: f.1.to_string(),
                        size: f.0
                    })
                    .collect();
                if let Some(existing ) = known_dirs.insert(top_dir, Dir { 
                    name: top_dir.to_string(), 
                    child_dirs: resolved_dirs, 
                    child_files: resolved_files 
                }) {
                    panic!("{:?} already present", existing);
                }

            } else {
                dir_names.push(next_dir);
            }
        }
        if command == "ls" {
            let mut cur_dirs: Vec<&str> = Vec::new();
            let mut cur_files: Vec<(i32, &str)> = Vec::new();
            for p in part[1..].iter() {
                if p.starts_with("dir") {
                    let dir = p.split(" ").collect::<Vec<_>>()[1];
                    cur_dirs.push(dir);
                } else {
                    let file_parts = p.split(" ").collect::<Vec<_>>();
                    cur_files.push((file_parts[0].parse().unwrap(), file_parts[1]));
                }
            }
            files.push(cur_files);
            dirs.push(cur_dirs);
        }
    }

    println!("done commands but remaining state");
    println!("dirs: \n\t{:?}\nfiles: \n\t{:?}\ndirs: \n\t{:?}\nknown_dirs: \n\t{:?}", 
        dir_names, files, dirs, known_dirs);

    loop {
        if let Some(top_dir) = dir_names.pop() {
            let top_files = files.pop().unwrap();
            let top_dirs = dirs.pop().unwrap();

            let resolved_dirs: Vec<_> = top_dirs
                .iter()
                .map(|dir| known_dirs.remove(dir).expect(&format!("missing: {:?}", dir)))
                .collect();
            let resolved_files: Vec<_> = top_files
                .iter()
                .map(|f| File {
                    name: f.1.to_string(),
                    size: f.0
                })
                .collect();
            if let Some(existing) = known_dirs.insert(top_dir, Dir { 
                name: top_dir.to_string(), 
                child_dirs: resolved_dirs, 
                child_files: resolved_files 
            }) {
                panic!("{:?} already exists", existing);
            }
        } else {
            break;
        }
    }
    let root = known_dirs.remove("/").expect("/");
    for (k, v) in known_dirs {
        println!("\t{:?}", v);
    }
    return root;
}

fn find_dirs(dir: &Dir, size: i32) -> Vec<&Dir> {
    let mut matching: Vec<&Dir> = Vec::new();

    for child in &dir.child_dirs {
        if child.size() <= size {
            matching.push(child);
            matching.extend(find_dirs(child, size));
        }
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
    child_dirs: Vec<Dir>,
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
            .map(|dir| dir as &dyn INode)
            .chain(self.child_files
                .iter()
                .map(|f| f as &dyn INode))
            .map(|inode| inode.size())
            .sum()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct DirState {
    dirs: Vec<String>,
    files: Vec<(i32, String)>
}

#[test]
fn test_size() {
    let root = Dir { 
        name: "/".to_string(),
        child_files: vec![
            File { name: ".tmp".to_string(), size: 1000 },
            File { name: ".tmp1".to_string(), size: 50 },
        ],
        child_dirs: vec![
            Dir {
                name: "a".to_string(),
                child_files: vec![
                    File { name: "a.tmp".to_string(), size: 10 }
                ],
                child_dirs: vec![]
            }
        ]
    };

    let total_size = root.size();
    assert_eq!(1060, total_size);
}

#[test]
fn test_part1_example() {
    let input = include_str!("input.example.txt");
    let root = parse(input);
    let found = find_dirs(&root, 100000);
    
    let total_size: i32 = found
        .iter()
        .map(|dir| dir.size())
        .sum();

    assert_eq!(95437, total_size);
}

#[test]
fn test_parse_repeats() {
    let input = include_str!("example.repeat.txt");
    let root = parse(input);
    println!("with repeats: {:?}", root);
}
