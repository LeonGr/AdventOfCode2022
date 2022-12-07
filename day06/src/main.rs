fn read_input() -> String {
    let input = include_str!("../input");
    input.to_string()
}

fn find_start_of_message_marker(input: &str, distinct: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(distinct)
        .position(|window| std::collections::HashSet::<&char>::from_iter(window.iter()).len() == distinct)
        .unwrap()
        + distinct
}

fn part1(input: &str) -> usize {
    find_start_of_message_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_start_of_message_marker(input, 14)
}

fn main() {
    let input = read_input();

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
