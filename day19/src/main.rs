use std::collections::HashSet;

use regex::{Captures, Regex};

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

fn parse(lines: &[String]) -> Vec<BluePrint> {
    lines.iter()
        .map(|line| {
            let re =
                Regex::new(
                    r"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."
                    ).unwrap();

            let match_to_u8 = |captures: &Captures, index| {
                captures.get(index).unwrap().as_str().parse::<u8>().unwrap()
            };

            let captures = re.captures(line).unwrap();
            let ore_robot_cost = (match_to_u8(&captures, 1), 0, 0);
            let clay_robot_cost = (match_to_u8(&captures, 2), 0, 0);
            let obsidian_robot_cost = (match_to_u8(&captures, 3), match_to_u8(&captures, 4), 0);
            let geode_robot_cost = (match_to_u8(&captures, 5), 0, match_to_u8(&captures, 6));

            let max_costs = [ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost].iter()
                .fold((0, 0, 0), |(max_ore, max_clay, max_obsidian), (ore_cost, clay_cost, obsidian_cost)| {
                    (max_ore.max(*ore_cost), max_clay.max(*clay_cost), max_obsidian.max(*obsidian_cost))
                });

            BluePrint { ore_robot_cost, clay_robot_cost, obsidian_robot_cost, geode_robot_cost, max_costs }
        })
        .collect()
}

type MaterialCost = (u8, u8, u8);
type RobotCount = (u8, u8, u8, u8);
type MaterialCount = (u8, u8, u8, u8);


#[derive(Debug)]
struct BluePrint {
    ore_robot_cost: MaterialCost,
    clay_robot_cost: MaterialCost,
    obsidian_robot_cost: MaterialCost,
    geode_robot_cost: MaterialCost,
    max_costs: MaterialCost,
}

fn recurse(
    current_robots: RobotCount,
    current_material: MaterialCount,
    current_minute: u8,
    max_minute: u8,
    blueprint: &BluePrint,
    max_found: u8,
    seen: &mut HashSet<(u8, RobotCount, MaterialCount)>,
) -> u8 {
    if seen.contains(&(current_minute, current_robots, current_material)) {
        return 0;
    }

    seen.insert((current_minute, current_robots, current_material));

    let (ore, clay, obsidian, geode) = current_material;

    if current_minute == max_minute + 1 {
        return geode;
    }

    let (ore_robots, clay_robots, obsidian_robots, geode_robots) = current_robots;

    let mut maxumim_possible = geode;
    for i in 1..(max_minute + 1 - current_minute) {
        maxumim_possible += geode_robots + i;
    }
    if maxumim_possible < max_found {
        return max_found;
    }

    let (ore_cost, clay_cost, obsidian_cost) = blueprint.geode_robot_cost;

    if ore_cost <= ore && clay_cost <= clay && obsidian_cost <= obsidian {
        let new_robots = (ore_robots, clay_robots, obsidian_robots, geode_robots + 1);
        let new_ore = (
            ore - ore_cost + ore_robots,
            clay - clay_cost + clay_robots,
            obsidian - obsidian_cost + obsidian_robots,
            geode + geode_robots,
        );
        return recurse(new_robots, new_ore, current_minute + 1, max_minute, blueprint, max_found, seen);
    }

    let options = [
        blueprint.obsidian_robot_cost,
        blueprint.clay_robot_cost,
        blueprint.ore_robot_cost,
        (0, 0, 0),
    ];

    let (max_ore_cost, max_clay_cost, max_obsidian_cost) = blueprint.max_costs;

    let mut max_geodes = 0;
    for i in 0..options.len() {
        let (ore_cost, clay_cost, obsidian_cost) = options[i];

        let new_robots = if ore_cost <= ore && clay_cost <= clay && obsidian_cost <= obsidian {
            if i == 3 {
                if max_ore_cost <= ore && max_clay_cost <= clay && max_obsidian_cost <= obsidian {
                    continue;
                }

                current_robots
            } else if i == 2 {
                if ore_robots >= max_ore_cost {
                    continue;
                }

                (ore_robots + 1, clay_robots, obsidian_robots, geode_robots)
            } else if i == 1 {
                if clay_robots >= max_clay_cost {
                    continue;
                }

                (ore_robots, clay_robots + 1, obsidian_robots, geode_robots)
            } else if i == 0 {
                if obsidian_robots >= max_obsidian_cost {
                    continue;
                }

                (ore_robots, clay_robots, obsidian_robots + 1, geode_robots)
            } else {
                unreachable!()
            }
        } else {
            continue;
        };

        let new_ore = (
            ore - ore_cost + ore_robots,
            clay - clay_cost + clay_robots,
            obsidian - obsidian_cost + obsidian_robots,
            geode + geode_robots,
        );

        let result = recurse(new_robots, new_ore, current_minute + 1, max_minute, blueprint, max_geodes, seen);
        max_geodes = max_geodes.max(result);
    }

    max_geodes
}

impl BluePrint {
    fn get_max_geodes(&self, minutes: u8) -> usize {
        recurse((1, 0, 0, 0), (0, 0, 0, 0), 1, minutes, self, 0, &mut HashSet::new()) as usize
    }
}

fn part1(blueprints: &[BluePrint]) -> usize {
    (0..blueprints.len()).map(|i| {
            let blueprint = &blueprints[i];
            (i + 1) * blueprint.get_max_geodes(24)
        })
        .sum()
}

fn part2(blueprints: &[BluePrint]) -> usize {
    (0..3).map(|i| {
            let blueprint = &blueprints[i];
            blueprint.get_max_geodes(32)
        })
        .product()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
