use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn get_priority(a: &char) -> u32 {
    match *a as u32 {
        a if a <= ('Z' as u32) => a - ('A' as u32) + 27,
        a if a >= ('a' as u32) => a - ('a' as u32) + 1,
        _ => unreachable!(),
    }
}

fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            left.chars().fold(0, |acc, l| {
                if right.chars().any(|c| c == l) {
                    get_priority(&l)
                } else {
                    acc
                }
            })
        })
        .sum()
}

fn part2(input: &[String]) -> u32 {
    input.chunks(3).fold(0, |acc, strings| {
        acc + strings.get(0).unwrap().chars().fold(0, |acc, l| {
            if let (Some(_), Some(_)) = (
                strings.get(1).unwrap().chars().position(|c| c == l),
                strings.get(2).unwrap().chars().position(|c| c == l),
            ) {
                get_priority(&l)
            } else {
                acc
            }
        })
    })
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));

    Ok(())
}
