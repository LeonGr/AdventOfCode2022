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
        a if a <= 90 => a - 64 + 26,
        a if a >= 97 => a - 96,
        _ => unreachable!(),
    }
}

fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|line| {
            let half = line.len() / 2;
            let chars = line.chars();

            let first_half: String = chars.clone().take(half).collect();
            let second_half: String = chars.skip(half).take(half).collect();

            (first_half, second_half)
        })
        .map(|(left, right)| {
            let left_chars: Vec<char> = left.chars().collect();
            let right_chars: Vec<char> = right.chars().collect();

            let mut priority = 0;
            for i in 0..left.len() {
                let l = left_chars.get(i).unwrap();

                if let Some(p) = right_chars.iter().position(|&c| c == *l) {
                    priority = get_priority(right_chars.get(p).unwrap())
                };
            }

            priority
        })
        .sum()
}

fn part2(input: &[String]) -> u32 {
    let mut i = 0;
    let mut total = 0;

    loop {
        if i >= input.len() {
            break;
        }

        let first: Vec<char> = input.get(i).unwrap().chars().collect();
        let second: Vec<char> = input.get(i + 1).unwrap().chars().collect();
        let third: Vec<char> = input.get(i + 2).unwrap().chars().collect();

        for i in 0..first.len() {
            let l = first.get(i).unwrap();

            if let (Some(p), Some(_)) = (
                second.iter().position(|&c| c == *l),
                third.iter().position(|&c| c == *l),
            ) {
                total += get_priority(second.get(p).unwrap());
                break;
            };
        }

        i += 3;
    }

    total
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));

    Ok(())
}
