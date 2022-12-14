use std::{collections::VecDeque, io::BufRead};

fn read_input_lines() -> std::io::Result<Vec<String>> {
    let input_file = std::fs::File::open("input")?;
    let file_reader = std::io::BufReader::new(input_file);

    Ok(file_reader
        .lines()
        .filter_map(std::io::Result::ok)
        .collect())
}

struct Move {
    quantity: usize,
    source: usize,
    target: usize,
}

type Crate = char;

type State = Vec<VecDeque<Crate>>;

fn parse(input: &[String]) -> (State, Vec<Move>) {
    let mut state = vec![VecDeque::new(); 9];

    let mut i = 0;
    loop {
        let line = &input[i];

        let chars: Vec<char> = line.chars().collect();

        for j in 0..line.len() {
            let c = chars.get(j).unwrap();

            if c.is_alphabetic() {
                let index = j / 4;

                state[index].push_back(*c);
            }
        }

        i += 1;

        if line.is_empty() {
            break;
        }
    }

    let mut moves = vec![];

    loop {
        if i >= input.len() {
            break;
        }

        let parts: Vec<usize> = input[i]
            .split(' ')
            .filter(|&word| word.chars().all(char::is_numeric))
            .map(|x| x.to_string().parse().unwrap())
            .collect();

        moves.push(Move {
            quantity: parts[0],
            source: parts[1] - 1,
            target: parts[2] - 1,
        });

        i += 1;
    }

    (state, moves)
}

fn get_top_crates(state: &mut State) -> String {
    state
        .iter_mut()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.pop_front().unwrap())
        .collect()
}

fn part1(input: (State, &Vec<Move>)) -> String {
    let (mut state, moves) = input;

    for mv in moves.iter() {
        (0..mv.quantity).for_each(|_| {
            let c: Crate = state[mv.source].pop_front().unwrap();
            state[mv.target].push_front(c);
        });
    }

    get_top_crates(&mut state)
}

fn part2(input: (State, &Vec<Move>)) -> String {
    let (mut state, moves) = input;

    for mv in moves.iter() {
        let removed: Vec<char> = state[mv.source].drain(0..mv.quantity).collect();
        removed.iter().rev().for_each(|c| {
            state[mv.target].push_front(*c);
        });
    }

    get_top_crates(&mut state)
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let (state, moves) = parse(&lines);

    println!("part1: {}", part1((state.clone(), &moves)));
    println!("part2: {}", part2((state, &moves)));

    Ok(())
}
