fn read_input() -> Vec<String> {
    include_str!("../input").lines().map(std::string::ToString::to_string).collect()
}

type Snafu = char;

fn snafu_to_decimal(c: Snafu) -> f64 {
    match c {
        '2' => 2.0,
        '1' => 1.0,
        '0' => 0.0,
        '-' => -1.0,
        '=' => -2.0,
        _ => unreachable!()
    }
}

fn decimal_to_snafu(n: i64) -> Snafu {
    match n {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => unreachable!()
    }
}

fn pow5(x: f64) -> f64 {
    5f64.powf(x)
}

fn to_decimal(snafu: &String) -> i64 {
    snafu.chars().rev().enumerate().fold(0, |acc, (i, c)| {
        acc + (snafu_to_decimal(c) * pow5(i as f64)) as i64
    })
}

fn to_decimals(lines: &[String]) -> Vec<i64> {
    lines.iter().map(to_decimal).collect()
}

fn to_snafu(mut number: i64) -> String {
    let mut digits = vec![];
    loop {
        digits.push(decimal_to_snafu(((number % 5) + 2) % 5 - 2));
        number += 2;
        number /= 5;

        if number == 0 {
            break;
        }
   }

    digits.into_iter().rev().collect()
}

fn part1(lines: &[String]) -> String {
    let sum = to_decimals(lines).iter().sum();
    let snafu = to_snafu(sum);

    assert_eq!(to_decimal(&snafu), sum);

    snafu
}

fn main() {
    let lines = read_input();
    println!("part1: {}", part1(&lines));
}
