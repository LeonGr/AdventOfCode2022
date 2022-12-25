fn read_input() -> Vec<String> {
    include_str!("../input").lines().map(std::string::ToString::to_string).collect()
}

fn char_to_decimal(c: char) -> f64 {
    match c {
        '2' => 2.0,
        '1' => 1.0,
        '0' => 0.0,
        '-' => -1.0,
        '=' => -2.0,
        _ => unreachable!()
    }
}

fn decimal_to_char(n: f64) -> char {
    match n as i32 {
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

fn to_decimal(snafu: &String) -> f64 {
    snafu.chars().rev().enumerate().fold(0.0, |acc, (i, c)| {
        acc + (char_to_decimal(c) * pow5(i as f64))
    })
}

fn to_decimals(lines: &[String]) -> Vec<f64> {
    lines.iter().map(to_decimal).collect()
}

fn to_snafu(number: f64) -> String {
    let mut max_index = 0.0;
    loop {
        let pow = pow5(max_index);
        if pow >= number || 2.0 * pow >= number {
            break;
        }

        max_index += 1.0;
    }

    if (pow5(max_index - 1.0) - number).abs() < (pow5(max_index) - number).abs() {
        max_index -= 1.0;
    }

    let options = [2.0, 1.0, 0.0, -1.0, -2.0];

    let mut digits = vec![];
    let mut remaining = number;
    let mut i = max_index;
    loop {
        let pow = pow5(i);

        let all: Vec<_> = options.iter().map(|option| (option, remaining - (option * pow))).collect();

        let (digit, new_remaining) =
            all.iter().fold((f64::MAX, f64::MAX), |(best_option, best_remaining), &(option, new_remaining)| {
                if (remaining - (option * pow)).abs() < (remaining - (best_option * pow)).abs() {
                    (*option, new_remaining)
                } else {
                    (best_option, best_remaining)
                }
            });

        remaining = new_remaining;

        digits.push(digit);

        i -= 1.0;

        if i < 0.0 {
            break;
        }
    }

    digits.into_iter().map(decimal_to_char).collect()
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
