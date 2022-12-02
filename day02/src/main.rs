use std::io::BufRead;

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

type Match = (Shape, Shape);

fn parse_shape(letter: &str) -> Shape {
    match letter {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => unreachable!(),
    }
}

fn parse(input: &[String]) -> Vec<(Shape, String)> {
    input
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();

            (parse_shape(parts[0]), parts[1].to_owned())
        })
        .collect()
}

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

const LOST: u8 = 0;
const DRAW: u8 = 3;
const WON: u8 = 6;

fn score(game: &Match) -> u8 {
    match game {
        (Shape::Rock, me) => match me {
            Shape::Rock => DRAW + ROCK,
            Shape::Paper => WON + PAPER,
            Shape::Scissors => LOST + SCISSORS,
        },
        (Shape::Paper, me) => match me {
            Shape::Rock => LOST + ROCK,
            Shape::Paper => DRAW + PAPER,
            Shape::Scissors => WON + SCISSORS,
        },
        (Shape::Scissors, me) => match me {
            Shape::Rock => WON + ROCK,
            Shape::Paper => LOST + PAPER,
            Shape::Scissors => DRAW + SCISSORS,
        },
    }
}

fn part1(input: &[(Shape, String)]) -> u32 {
    input
        .iter()
        .map(|(opponent_shape, me)| score(&(*opponent_shape, parse_shape(me.as_str()))) as u32)
        .sum()
}

fn lose(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

fn win(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

fn draw(shape: &Shape) -> Shape {
    *shape
}

fn part2(input: &[(Shape, String)]) -> u32 {
    input
        .iter()
        .map(|(opponent_shape, me)| {
            let my_shape = match me.as_str() {
                "X" => lose(opponent_shape),
                "Y" => draw(opponent_shape),
                "Z" => win(opponent_shape),
                _ => unreachable!(),
            };

            score(&(*opponent_shape, my_shape)) as u32
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));

    Ok(())
}
