use std::collections::HashSet;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

type Coord = i32;
type Pos = (Coord, Coord);
type Beacon = Pos;
type Sensor = Pos;

fn parse(lines: &[String]) -> Vec<(Sensor, Beacon)> {
    lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(": closest beacon is at ").collect();
            let sensor_part = parts[0].split_once("Sensor at ").unwrap().1;

            let sensor: Vec<Coord> = sensor_part
                .split(", ")
                .map(|coord| coord.split_once('=').unwrap().1.parse().unwrap())
                .collect();
            let beacon: Vec<Coord> = parts[1]
                .split(", ")
                .map(|coord| coord.split_once('=').unwrap().1.parse().unwrap())
                .collect();

            ((sensor[0], sensor[1]), (beacon[0], beacon[1]))
        })
        .collect()
}

fn get_endpoints(input: &[(Sensor, Beacon)]) -> (Coord, Coord) {
    let ((min_x, max_x), max_d) = input.iter().fold(
        ((Coord::MAX, Coord::MIN), 0),
        |((min_x, max_x), max_d), (sensor, beacon)| {
            (
                (
                    sensor.0.min(sensor.1.min(min_x)),
                    sensor.0.max(sensor.1.max(max_x)),
                ),
                manhattan_distance(*sensor, *beacon).max(max_d),
            )
        },
    );

    (min_x - max_d, max_x + max_d)
}

fn manhattan_distance(first: Pos, second: Pos) -> i32 {
    (first.0 - second.0).abs() + (first.1 - second.1).abs()
}

fn is_in_range(sensor: Sensor, beacon: Beacon, pos: Pos) -> bool {
    manhattan_distance(sensor, beacon) >= manhattan_distance(sensor, pos)
}

fn part1(input: &[(Sensor, Beacon)]) -> usize {
    let (x_min, x_max) = get_endpoints(input);

    let y = 2_000_000;

    let horizontal_covered_positions = (x_min..=x_max)
        .filter(|x| {
            input
                .iter()
                .any(|(sensor, beacon)| is_in_range(*sensor, *beacon, (*x, y)))
        })
        .count();

    let beacons_at_y = input
        .iter()
        .filter_map(
            |(_, beacon)| {
                if beacon.1 == y {
                    Some(*beacon)
                } else {
                    None
                }
            },
        )
        .collect::<HashSet<Beacon>>()
        .len();

    horizontal_covered_positions - beacons_at_y
}

fn get_adjacent(sensor: Sensor, beacon: Beacon, min: i32, max: i32) -> Vec<Pos> {
    let distance = manhattan_distance(sensor, beacon) + 1;

    let mut points = vec![];

    for i in 0..=distance {
        let start = ((sensor.0 - distance), sensor.1);
        if start.0 + i >= min && start.0 + i <= max {
            if start.1 - i >= min && start.1 - i <= max {
                points.push((start.0 + i, start.1 - i));
            }
            if start.1 + i >= min && start.1 + i <= max {
                points.push((start.0 + i, start.1 + i));
            }
        }

        let start = ((sensor.0 + distance), sensor.1);
        if start.0 - i >= min && start.0 - i <= max {
            if start.1 - i >= min && start.1 - i <= max {
                points.push((start.0 - i, start.1 - i));
            }
            if start.1 + i >= min && start.1 + i <= max {
                points.push((start.0 - i, start.1 + i));
            }
        }
    }

    points
}

fn part2(input: &[(Sensor, Beacon)]) -> usize {
    let min = 0;
    let max = 4_000_000;

    let not_covered: Vec<Pos> = input
        .iter()
        .flat_map(|(sensor, beacon)| get_adjacent(*sensor, *beacon, min, max))
        .filter(|point| {
            let (x, y) = point;

            input
                .iter()
                .all(|(sensor, beacon)| !is_in_range(*sensor, *beacon, (*x, *y)))
        })
        .collect();

    let (x, y) = not_covered[0];

    (x as usize) * 4_000_000 + (y as usize)
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
