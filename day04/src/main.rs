use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

type Range = (u8, u8);

fn parse(input: &[String]) -> Vec<(Range, Range)> {
    input
        .iter()
        .map(|line| {
            let ranges: Vec<Range> = line
                .split(',')
                .map(|range| {
                    let delimiters: Vec<u8> = range.split('-').map(|c| c.parse().unwrap()).collect();

                    (delimiters[0], delimiters[1])
                })
                .collect();

            (ranges[0], ranges[1])
        })
        .collect()
}

fn part1(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(
            |&pair| matches!(pair, ((a, b), (x, y)) if (a <= x && b >= y) || (x <= a && y >= b)),
        )
        .count()
}

fn part2(input: &[(Range, Range)]) -> usize {
    input
        .iter()
        .filter(|&pair| {
            matches!(pair, ((a, b), (x, y)) if ((a <= x && x <= b) || (a <= y && y <= b))
                    || ((x <= a && a <= y) || (x <= b && b <= y)))
        })
        .count()
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));

    Ok(())
}
