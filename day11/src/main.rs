use std::collections::HashMap;

fn read_input() -> String {
    let input = include_str!("../input");
    input.to_string()
}

type Item = u64;

struct Monkey {
    items: Vec<Item>,
    operation: Box<dyn Fn(Item) -> Item>,
    test: Box<dyn Fn(Item) -> bool>,
    result: HashMap<bool, usize>,
    inspected: usize,
    divisible_by: Item,
}

fn parse_starting_items(input: &str) -> Vec<Item> {
    input
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn parse_operation(input: &str) -> Box<dyn Fn(Item) -> Item> {
    let rhs = input.split_once("Operation: new = ").unwrap().1;

    if let Some((_, r)) = rhs.split_once(" * ") {
        if let Ok(n) = r.parse::<Item>() {
            let operation = move |item: Item| item * n;

            Box::new(operation)
        } else {
            let operation = |item: Item| item * item;

            Box::new(operation)
        }
    } else if let Some((_, r)) = rhs.split_once(" + ") {
        if let Ok(n) = r.parse::<Item>() {
            let operation = move |item: Item| item + n;

            Box::new(operation)
        } else {
            let operation = |item: Item| item + item;

            Box::new(operation)
        }
    } else {
        unreachable!();
    }
}

fn parse_test(input: &str) -> (Box<dyn Fn(Item) -> bool>, Item) {
    let divisible_by: Item = input
        .split_once("Test: divisible by ")
        .unwrap()
        .1
        .parse()
        .unwrap();

    let test = move |item: Item| item % divisible_by == 0;

    (Box::new(test), divisible_by)
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("Monkey ")
        .skip(1)
        .map(|monkey_text| {
            let parts: Vec<&str> = monkey_text.split("\n  ").collect();

            let items = parse_starting_items(parts[1]);
            let operation: Box<dyn Fn(Item) -> Item> = parse_operation(parts[2]);
            let (test, divisible_by): (Box<dyn Fn(Item) -> bool>, Item) = parse_test(parts[3]);
            let result: HashMap<bool, usize> = vec![parts[4], parts[5]]
                .iter()
                .map(|&result| {
                    if let Some((_, monkey)) = result.split_once("  If true: throw to monkey ") {
                        (true, monkey.trim().parse().unwrap())
                    } else if let Some((_, monkey)) =
                        result.split_once("  If false: throw to monkey ")
                    {
                        (false, monkey.trim().parse().unwrap())
                    } else {
                        unreachable!()
                    }
                })
                .collect();

            Monkey {
                items,
                operation,
                test,
                result,
                inspected: 0,
                divisible_by,
            }
        })
        .collect()
}

fn monkey_throws(monkey: &mut Monkey, divide_by: Item, limit: Item) -> Vec<(usize, Item)> {
    let throws = monkey
        .items
        .iter()
        .map(|item| {
            let new_value = ((monkey.operation)(*item) / divide_by) % limit;
            let test_result = (monkey.test)(new_value);
            let new_monkey = monkey.result.get(&test_result).unwrap();

            monkey.inspected += 1;
            (*new_monkey, new_value)
        })
        .collect();

    monkey.items.clear();

    throws
}

fn monkeys_catch(monkeys: &mut [Monkey], throws: &[(usize, Item)]) {
    throws.iter().for_each(|(num_monkey, item)| {
        let monkey = &mut monkeys[*num_monkey];
        monkey.items.push(*item);
    });
}

fn top_inspected_product(monkeys: &mut Vec<Monkey>, rounds: usize, divide_by: Item) -> usize {
    let limit = monkeys.iter().map(|monkey| monkey.divisible_by).product();

    (0..rounds).for_each(|_| {
        (0..monkeys.len()).for_each(|num_monkey| {
            let monkey = &mut monkeys[num_monkey];

            let throws = monkey_throws(monkey, divide_by, limit);
            monkeys_catch(monkeys, &throws);
        });
    });

    let mut inspected: Vec<usize> = monkeys.iter().map(|monkey| monkey.inspected).collect();

    inspected.sort();
    inspected.reverse();
    inspected.iter().take(2).product()
}

fn part1(monkeys: &mut Vec<Monkey>) -> usize {
    top_inspected_product(monkeys, 20, 3)
}

fn part2(monkeys: &mut Vec<Monkey>) -> usize {
    top_inspected_product(monkeys, 10_000, 1)
}

fn main() {
    let input = read_input();
    let mut parsed = parse(&input);
    println!("part1: {}", part1(&mut parsed));

    let input = read_input();
    let mut parsed = parse(&input);
    println!("part2: {}", part2(&mut parsed));
}
