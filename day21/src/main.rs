use std::collections::HashMap;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

type Monkey = String;

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, Debug)]
enum Job {
    Yell(i64),
    Math(Monkey, Monkey, Operation),
}

fn parse(lines: &[String]) -> HashMap<Monkey, Job> {
    lines
        .iter()
        .map(|line| {
            let (monkey, job) = line.split_once(": ").unwrap();

            let job = if let Some((monkey1, monkey2)) = job.split_once(" + ") {
                Job::Math(monkey1.to_string(), monkey2.to_string(), Operation::Add)
            } else if let Some((monkey1, monkey2)) = job.split_once(" - ") {
                Job::Math(monkey1.to_string(), monkey2.to_string(), Operation::Subtract)
            } else if let Some((monkey1, monkey2)) = job.split_once(" * ") {
                Job::Math(monkey1.to_string(), monkey2.to_string(), Operation::Multiply)
            } else if let Some((monkey1, monkey2)) = job.split_once(" / ") {
                Job::Math(monkey1.to_string(), monkey2.to_string(), Operation::Divide)
            } else {
                Job::Yell(job.parse().unwrap())
            };

            (monkey.to_string(), job)
        })
        .collect()
}

fn evaluate(monkey_business: &HashMap<Monkey, Job>, job: &Job) -> i64 {
    match job {
        Job::Yell(value) => *value,
        Job::Math(left_monkey, right_monkey, operation) => {
            let values = [left_monkey, right_monkey]
                .map(|monkey| evaluate(monkey_business, monkey_business.get(monkey).unwrap()));

            match operation {
                Operation::Add => values[0] + values[1],
                Operation::Subtract => values[0] - values[1],
                Operation::Multiply => values[0] * values[1],
                Operation::Divide => values[0] / values[1],
            }
        }
    }
}

fn part1(monkey_business: &HashMap<Monkey, Job>) -> i64 {
    let root = Monkey::from("root");

    evaluate(monkey_business, monkey_business.get(&root).unwrap())
}

fn try_evaluate(monkey_business: &HashMap<Monkey, Job>, job: &Job) -> Option<i64> {
    match job {
        Job::Yell(value) => Some(*value),
        Job::Math(left_monkey, right_monkey, operation) => {
            let values = [left_monkey, right_monkey].map(|monkey| {
                monkey_business
                    .get(monkey)
                    .and_then(|job| try_evaluate(monkey_business, job))
            });

            match (values[0], values[1]) {
                (Some(left_value), Some(right_value)) => match operation {
                    Operation::Add => Some(left_value + right_value),
                    Operation::Subtract => Some(left_value - right_value),
                    Operation::Multiply => Some(left_value * right_value),
                    Operation::Divide => Some(left_value / right_value),
                },
                _ => None,
            }
        }
    }
}

fn simplify(monkey_business: &mut HashMap<Monkey, Job>) {
    let copy = monkey_business.clone();

    for (monkey, job) in &copy {
        match job {
            Job::Yell(_) => continue,
            Job::Math(left_monkey, right_monkey, operation) => {
                if left_monkey == "humn" || right_monkey == "humn" {
                    continue;
                }

                let values = [left_monkey, right_monkey]
                    .map(|monkey| try_evaluate(&copy, monkey_business.get(monkey).unwrap()));

                if let (Some(left_value), Some(right_value)) = (values[0], values[1]) {
                    let folded_value = match operation {
                        Operation::Add => left_value + right_value,
                        Operation::Subtract => left_value - right_value,
                        Operation::Multiply => left_value * right_value,
                        Operation::Divide => left_value / right_value,
                    };

                    monkey_business.remove(left_monkey);
                    monkey_business.remove(right_monkey);
                    monkey_business.insert(monkey.to_string(), Job::Yell(folded_value));
                }
            }
        }
    }
}

fn monkey_job_to_string(monkey_business: &HashMap<Monkey, Job>, monkey: &Monkey) -> String {
    if monkey == "humn" {
        return String::from("x");
    }

    let job = monkey_business.get(monkey).unwrap();
    match job {
        Job::Yell(value) => value.to_string(),
        Job::Math(left_monkey, right_monkey, operation) => {
            let left_equation = monkey_job_to_string(monkey_business, left_monkey);
            let right_equation = monkey_job_to_string(monkey_business, right_monkey);

            let operator = match operation {
                Operation::Add => '+',
                Operation::Subtract => '-',
                Operation::Multiply => '*',
                Operation::Divide => '/',
            };

            format!("({left_equation} {operator} {right_equation})")
        }
    }
}

fn part2(monkey_business: &HashMap<Monkey, Job>) -> String {
    let mut monkey_business = monkey_business.clone();
    let root = Monkey::from("root");
    let me = Monkey::from("humn");

    monkey_business.remove(&me);

    simplify(&mut monkey_business);

    let root_job = monkey_business.get(&root).unwrap();
    if let Job::Math(left_monkey, right_monkey, Operation::Add) = root_job {
        format!(
            "plug this into some solver, good luck :) \n{} == {}",
            monkey_job_to_string(&monkey_business, left_monkey),
            monkey_job_to_string(&monkey_business, right_monkey)
        )
    } else {
        unreachable!()
    }
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
