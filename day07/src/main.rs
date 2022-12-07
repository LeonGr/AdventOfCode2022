fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input.to_string().lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, Clone)]
struct Directory {
    directories: Vec<Directory>,
    files_size: usize,
}

impl Directory {
    fn size(&self) -> usize {
        self.files_size + self.directories.iter().map(|dir| dir.size()).sum::<usize>()
    }
}

fn get_dirs(input: &[String]) -> Vec<Directory> {
    let mut stack: Vec<Directory> = vec![];
    let mut directories = vec![];

    let mut i = 0;
    while i < input.len() {
        let line = &input[i];

        if line.starts_with('$') {
            if line.contains("cd") {
                if line.contains("..") {
                    let last = stack.pop().unwrap();
                    let parent: &mut Directory = stack.last_mut().unwrap();
                    parent.directories.push(last.clone());
                    directories.push(last);
                } else {
                    let dir = Directory {
                        directories: vec![],
                        files_size: 0,
                    };

                    stack.push(dir);
                }
            } else {
                let mut dir_file_size = 0;

                let mut j = i + 1;
                while j < input.len() {
                    let ls_line = &input[j];

                    if ls_line.starts_with('$') {
                        break;
                    }

                    if !ls_line.starts_with("dir") {
                        dir_file_size += ls_line.split_once(' ').unwrap().0.parse::<usize>().unwrap();
                    }

                    j += 1;
                }

                stack.last_mut().unwrap().files_size += dir_file_size;

                i = j - 1;
            }
        }

        i += 1;
    }

    while stack.len() > 1 {
        let last = stack.pop().unwrap();
        stack.last_mut().unwrap().directories.push(last.clone());
        directories.push(last);
    }

    directories.append(&mut stack);

    directories
}

fn part1(dirs: &[Directory]) -> usize {
    dirs.iter()
        .filter_map(|dir| match dir.size() {
            size if size <= 100_000 => Some(size),
            _ => None,
        })
        .sum()
}

fn part2(dirs: &[Directory]) -> usize {
    let to_delete = 30_000_000 - (70_000_000 - dirs.last().unwrap().size());
    dirs.iter().fold(usize::MAX, |acc, dir| match dir.size() {
        size if size >= to_delete => usize::min(acc, size),
        _ => acc,
    })
}

fn main() {
    let lines = read_input();
    let dirs = get_dirs(&lines);

    println!("part1: {}", part1(&dirs));
    println!("part2: {}", part2(&dirs));
}
