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

        let line = &input[i];

        let parts: Vec<usize> = line
            .split(' ')
            .filter(|&word| word.chars().all(|c| c.is_numeric()))
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

fn get_top_crates(state: &State) -> String {
    let mut output = String::new();
    for stack in state {
        if !stack.is_empty() {
            output += &stack[0].to_string();
        }
    }

    output
}

fn part1(input: (State, &Vec<Move>)) -> String {
    let (mut state, moves) = input;

    for mv in moves {
        for _ in 0..mv.quantity {
            let c: Crate = state[mv.source].pop_front().unwrap();
            state[mv.target].push_front(c);
        }
    }

    get_top_crates(&state)
}

fn part2(input: (State, &Vec<Move>)) -> String {
    let (mut state, moves) = input;

    for mv in moves {
        let removed: Vec<char> = state[mv.source].drain(0..mv.quantity).collect();
        for c in removed.iter().rev() {
            state[mv.target].push_front(*c);
        }
    }

    get_top_crates(&state)
}

fn main() -> std::io::Result<()> {
    let lines = read_input_lines()?;
    let (state, moves) = parse(&lines);

    println!("part1: {}", part1((state.clone(), &moves)));
    println!("part2: {}", part2((state, &moves)));

    Ok(())
}
