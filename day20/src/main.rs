fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

fn parse(lines: &[String]) -> Vec<i64> {
    lines.iter().map(|line| line.parse().unwrap()).collect()
}

fn decrypt(input: &[i64], rotations: usize, decryption_key: i64) -> i64 {
    let input: Vec<i64> = input.iter().map(|x| *x * decryption_key).collect();

    let mut coords_with_original_position: Vec<(usize, i64)> =
        input.iter().enumerate().map(|(i, x)| (i, *x)).collect();

    let len = input.len();

    (0..rotations).for_each(|_| {
        (0..input.len()).for_each(|i| {
            let current = input[i];
            let position = coords_with_original_position
                .iter()
                .position(|(original_index, _)| *original_index == i)
                .unwrap();
            coords_with_original_position.remove(position);
            #[allow(clippy::cast_possible_truncation)]
            let new_position = ((position as i64) + current).rem_euclid(len as i64 - 1) as usize;

            coords_with_original_position.insert(new_position, (i, current));
        });
    });

    let final_coords: Vec<i64> = coords_with_original_position
        .iter()
        .map(|(_, x)| *x)
        .collect();

    let zero_positon = final_coords.iter().position(|x| *x == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|index| final_coords.get((zero_positon + index) % len).unwrap())
        .sum()
}

fn part1(input: &[i64]) -> i64 {
    decrypt(input, 1, 1)
}

fn part2(input: &[i64]) -> i64 {
    decrypt(input, 10, 811_589_153)
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
