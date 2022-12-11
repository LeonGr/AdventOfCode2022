fn read_input() -> String {
    let input = include_str!("../input");
    input.to_string()
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    new_monkey: Box<dyn Fn(u64) -> usize>,
    inspected: usize,
    divisible_by: u64,
}

fn parse_starting_items(input: &str) -> Vec<u64> {
    input
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn parse_operation(input: &str) -> Box<dyn Fn(u64) -> u64> {
    let second_number = input.split(' ').last().unwrap().parse::<u64>().ok();
    let multiply = input.contains('*');

    let operation = move |item| {
        let second = match second_number {
            Some(n) => n,
            None => item,
        };

        if multiply {
            item * second
        } else {
            item + second
        }
    };

    Box::new(operation)
}

fn parse_new_monkey_fn(
    test_str: &str,
    true_str: &str,
    false_str: &str,
) -> (Box<dyn Fn(u64) -> usize>, u64) {
    let divisible_by = test_str.split(' ').last().unwrap().parse().unwrap();

    let new_monkey_nums: Vec<usize> = vec![true_str, false_str]
        .iter()
        .map(|&result| result.trim().split(' ').last().unwrap().parse().unwrap())
        .collect();

    let new_monkey = move |item| new_monkey_nums[usize::from(item % divisible_by != 0)];

    (Box::new(new_monkey), divisible_by)
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("Monkey ")
        .skip(1)
        .map(|monkey_text| {
            let parts: Vec<&str> = monkey_text.split("\n  ").collect();

            let items = parse_starting_items(parts[1]);
            let operation = parse_operation(parts[2]);
            let (new_monkey, divisible_by) = parse_new_monkey_fn(parts[3], parts[4], parts[5]);

            Monkey {
                items,
                operation,
                new_monkey,
                inspected: 0,
                divisible_by,
            }
        })
        .collect()
}

fn monkey_throws(monkey: &mut Monkey, divide_by: u64, limit: u64) -> Vec<(usize, u64)> {
    let throws = monkey
        .items
        .iter()
        .map(|item| {
            let new_value = ((monkey.operation)(*item) / divide_by) % limit;
            let new_monkey = (monkey.new_monkey)(new_value);

            monkey.inspected += 1;
            (new_monkey, new_value)
        })
        .collect();

    monkey.items.clear();

    throws
}

fn monkeys_catch(monkeys: &mut [Monkey], throws: &[(usize, u64)]) {
    for (num_monkey, item) in throws.iter() {
        let monkey = &mut monkeys[*num_monkey];
        monkey.items.push(*item);
    }
}

fn top_inspected_product(monkeys: &mut Vec<Monkey>, rounds: usize, divide_by: u64) -> usize {
    let limit = monkeys.iter().map(|monkey| monkey.divisible_by).product();

    (0..rounds).for_each(|_| {
        (0..monkeys.len()).for_each(|num_monkey| {
            let monkey = &mut monkeys[num_monkey];

            let throws = monkey_throws(monkey, divide_by, limit);
            monkeys_catch(monkeys, &throws);
        });
    });

    let mut inspected: Vec<usize> = monkeys.iter().map(|monkey| monkey.inspected).collect();

    inspected.sort_unstable();
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
