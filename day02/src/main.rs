use std::{io::BufRead, str::FromStr};

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

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(String::from("Unexpected shape letter")),
        }
    }
}

type Match = (Shape, Shape);

fn parse(input: &[String]) -> Vec<(Shape, String)> {
    input
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();

            (parts[0].parse().unwrap(), parts[1].to_owned())
        })
        .collect()
}

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

const LOST: u8 = 0;
const DRAW: u8 = 3;
const WON: u8 = 6;

fn score(game: Match) -> u8 {
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
        .map(|(opponent_shape, me)| u32::from(score((*opponent_shape, me.parse().unwrap()))))
        .sum()
}

fn lose(shape: Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

fn win(shape: Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

fn draw(shape: Shape) -> Shape {
    shape
}

fn part2(input: &[(Shape, String)]) -> u32 {
    input
        .iter()
        .map(|(opponent_shape, me)| {
            let my_shape = match me.as_str() {
                "X" => lose(*opponent_shape),
                "Y" => draw(*opponent_shape),
                "Z" => win(*opponent_shape),
                _ => unreachable!(),
            };

            u32::from(score((*opponent_shape, my_shape)))
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
