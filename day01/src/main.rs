use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

fn parse(input: &[String]) -> Vec<Vec<u32>> {
    input
        .split(std::string::String::is_empty)
        .map(|elf| elf.iter().map(|calories| calories.parse().unwrap()).collect())
        .collect()
}

fn part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

fn part2(input: &[Vec<u32>]) -> u32 {
    let mut elves: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();

    elves.sort_unstable();
    elves.reverse();

    elves.iter().take(3).sum()
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));

    Ok(())
}
