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
            let half = line.len() / 2;
            let chars = line.chars();

            let first_half: String = chars.clone().take(half).collect();
            let second_half: String = chars.skip(half).take(half).collect();

            (first_half, second_half)
        })
        .map(|(left, right)| {
            let left_chars: Vec<char> = left.chars().collect();

            let mut priority = 0;
            for i in 0..left.len() {
                let l = left_chars.get(i).unwrap();

                if let Some(_) = right.chars().position(|c| c == *l) {
                    priority = get_priority(l)
                };
            }

            priority
        })
        .sum()
}

fn part2(input: &[String]) -> u32 {
    let mut total = 0;

    for i in (0..input.len()).step_by(3) {
        let first: Vec<char> = input.get(i).unwrap().chars().collect();

        for j in 0..first.len() {
            let l = first.get(j).unwrap();

            if let (Some(_), Some(_)) = (
                input.get(i + 1).unwrap().chars().position(|c| c == *l),
                input.get(i + 2).unwrap().chars().position(|c| c == *l),
            ) {
                total += get_priority(l);
                break;
            };
        }
    }

    total
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;

    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));

    Ok(())
}
