use std::collections::HashSet;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input.to_string().lines().map(std::string::ToString::to_string).collect()
}

type Coord = i32;
type Pos = (Coord, Coord);
type Beacon = Pos;
type Sensor = Pos;

fn parse(lines: &[String]) -> Vec<(Sensor, Beacon)> {
    lines.iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(": closest beacon is at ").collect();
            let sensor_part = parts[0].split_once("Sensor at ").unwrap().1;

            let sensor: Vec<Coord> = sensor_part.split(", ").map(|coord| coord.split_once("=").unwrap().1.parse().unwrap()).collect();
            let beacon: Vec<Coord> = parts[1].split(", ").map(|coord| coord.split_once("=").unwrap().1.parse().unwrap()).collect();

            ((sensor[0], sensor[1]), (beacon[0], beacon[1]))
        })
    .collect()
}

fn get_endpoints(input: &[(Sensor, Beacon)]) -> (Coord, Coord, Coord, Coord) {
    let end_points =
        input.iter()
            .fold((Coord::MAX, Coord::MIN, Coord::MAX, Coord::MIN), |acc, segments| {
                let (mut x_min, mut x_max, mut y_min, mut y_max) = acc;
                for segment in [segments.0, segments.1] {
                        let (x, y) = segment;

                        x_min = Coord::min(x, x_min);
                        x_max = Coord::max(x, x_max);
                        y_min = Coord::min(y, y_min);
                        y_max = Coord::max(y, y_max);
                    }

                (x_min, x_max, y_min, y_max)
            });

    end_points
}

fn manhattan_distance(first: Pos, second: Pos) -> i32 {
    i32::abs(first.0 - second.0) + i32::abs(first.1 - second.1)
}

fn is_in_range(sensor: Sensor, beacon: Beacon, pos: Pos) -> bool {
    manhattan_distance(sensor, beacon) >= manhattan_distance(sensor, pos)
}

fn part1(input: &[(Sensor, Beacon)]) -> usize {
    let (x_min, x_max, y_min, y_max) = get_endpoints(input);
    println!("{:?}", (x_min, x_max, y_min, y_max));


    let y = 2_000_000;
    // let y = 10;

    let horizontal_covered_positions =
        (x_min-2_000_000..=x_max+2_000_000).filter(|x| {
            input.iter().any(|(sensor, beacon)| {
                is_in_range(*sensor, *beacon, (*x, y))
            })
        })
        .count();

    let beacons_at_y =
        input.iter().filter_map(|(_, beacon)| {
            if beacon.1 == y {
                Some(*beacon)
            } else {
                None
            }
        })
        .collect::<HashSet<Beacon>>()
        .len();

    horizontal_covered_positions - beacons_at_y
}

fn get_adjacent(sensor: Sensor, beacon: Beacon) -> Vec<Pos> {
    let distance = manhattan_distance(sensor, beacon) + 1;

    let mut points = vec![];

    for i in 0..=distance {
        let start = ((sensor.0 - distance), sensor.1);
        points.push((start.0 + i, start.1 - i));
        points.push((start.0 + i, start.1 + i));

        let start = ((sensor.0 + distance), sensor.1);
        points.push((start.0 - i, start.1 - i));
        points.push((start.0 - i, start.1 + i));
    }

    points
}

fn part2(input: &[(Sensor, Beacon)]) -> usize {
    let min = 0;
    // let max = 20;
    let max = 4_000_000;

    let points: HashSet<Pos> = input.iter()
        .flat_map(|(sensor, beacon)| {
            get_adjacent(*sensor, *beacon).into_iter().filter(|(x, y)| *x >= 0 && *y >= 0 && *x <= max && *y <= max).collect::<Vec<_>>()
        })
        .collect();

    println!("{}", points.len());

    let test: Vec<Pos> =
        points.into_iter().filter(|point| {
            let (x, y) = point;

            input.iter().all(|(sensor, beacon)| {
                !is_in_range(*sensor, *beacon, (*x, *y))
            })
        })
        .collect();

    let (x, y) = test[0];

    (x as usize) * 4_000_000 + (y as usize)
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
