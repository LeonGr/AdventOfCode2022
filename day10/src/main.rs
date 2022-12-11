fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .to_string()
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

enum Operation {
    Noop,
    Add(i8),
}

fn parse(input: &[String]) -> Vec<Operation> {
    input
        .iter()
        .map(|line| match line.split_once(' ') {
            Some((_, add)) => Operation::Add(add.parse().unwrap()),
            None => Operation::Noop,
        })
        .collect()
}

fn get_values_x(operations: &[Operation]) -> Vec<i32> {
    operations
        .iter()
        .fold((1, vec![1]), |(mut x, mut values_x), op| {
            match op {
                Operation::Noop => values_x.push(x),
                Operation::Add(increment) => {
                    values_x.push(x);
                    values_x.push(x);
                    x += i32::from(*increment);
                }
            }

            (x, values_x)
        })
        .1
}

fn part1(values_x: &[i32]) -> i32 {
    (20..=220).step_by(40).fold(0, |acc, cycle| {
        acc + (cycle * values_x.get(cycle as usize).unwrap())
    })
}

fn part2(values_x: &[i32]) -> String {
    values_x
        .iter()
        .skip(1)
        .enumerate()
        .fold(String::new(), |mut output, (i, x)| {
            let index = (i % 40) as i32;
            output += if x - 1 <= index && index <= x + 1 {
                "#"
            } else {
                " "
            };

            if (i + 1) % 40 == 0 {
                output += "\n";
            }

            output
        })
}

fn main() {
    let lines = read_input();
    let operations = parse(&lines);
    let values_x = get_values_x(&operations);
    println!("part1: {}", part1(&values_x));
    println!("part2:\n{}", part2(&values_x));
}
