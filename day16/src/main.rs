use std::collections::{HashMap, HashSet, VecDeque};

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

type Identifier = String;

#[derive(Debug)]
struct Valve {
    flow_rate: i32,
    tunnels: Vec<Identifier>,
}

fn parse(input: &[String]) -> HashMap<Identifier, Valve> {
    input.iter().fold(HashMap::new(), |mut acc, line| {
        let (valve, tunnels) =
            if let Some((valve, tunnels)) = line.split_once("; tunnels lead to valves ") {
                (valve, tunnels)
            } else if let Some((valve, tunnel)) = line.split_once("; tunnel leads to valve ") {
                (valve, tunnel)
            } else {
                unreachable!()
            };
        let (identifier, flow_rate) = valve.split_once(" has flow rate=").unwrap();

        let identifier = identifier.split_once("Valve ").unwrap().1.to_string();
        let flow_rate = flow_rate.parse().unwrap();

        let tunnels = tunnels
            .split(", ")
            .map(std::string::ToString::to_string)
            .collect();

        if acc.get_mut(&identifier).is_some() {
            unreachable!()
        } else {
            let valve = Valve { flow_rate, tunnels };
            acc.insert(identifier.clone(), valve);
        }

        acc
    })
}

fn get_path(
    valves: &HashMap<Identifier, Valve>,
    source: &Identifier,
    target: &Identifier,
) -> Vec<Identifier> {
    let mut seen: HashSet<Identifier> = HashSet::new();
    let mut queue = VecDeque::from([source]);
    let mut parents: HashMap<Identifier, Identifier> = HashMap::new();

    while !queue.is_empty() {
        let last = queue.pop_front().unwrap();
        if last == target {
            break;
        }

        let valve = valves.get(last).unwrap();
        for tunnel in &valve.tunnels {
            if seen.get(tunnel).is_none() {
                seen.insert(tunnel.to_string());
                parents.insert(tunnel.to_string(), last.to_string());
                queue.push_back(tunnel);
            }
        }
    }

    let mut path = vec![];
    let mut current = target;

    while current != source {
        path.push(current.to_string());
        let parent = parents.get(current).unwrap();
        current = parent;
    }

    path.reverse();

    path
}

fn get_all_paths_costs(
    valves: &HashMap<Identifier, Valve>,
) -> HashMap<Identifier, HashMap<Identifier, usize>> {
    let mut path_costs: HashMap<Identifier, HashMap<Identifier, usize>> = HashMap::new();

    for (id_s, valve_s) in valves {
        if valve_s.flow_rate == 0 && id_s != "AA" {
            continue;
        }

        let mut source_map = HashMap::new();

        for (id_t, valve_t) in valves {
            if id_t != "AA" && (id_s == id_t || valve_t.flow_rate == 0) {
                continue;
            }

            let path = get_path(valves, id_s, id_t);

            source_map.insert(id_t.to_string(), path.len());
        }

        path_costs.insert(id_s.to_string(), source_map);
    }

    path_costs
}

fn valid_permutations(
    costs: &HashMap<String, HashMap<String, usize>>,
    remaining: &Vec<String>,
    current: &mut Vec<Identifier>,
    current_cost: usize,
    max_cost: usize,
) -> Vec<Vec<Identifier>> {
    if remaining.is_empty() {
        return vec![current.clone()];
    }

    let mut new = vec![];

    let mut end = true;
    for r in remaining {
        let mut remaining = remaining.clone();
        let index = remaining.iter().position(|x| x == r).unwrap();
        remaining.remove(index);

        let last = current.last().unwrap();

        if r == last {
            continue;
        }

        let add_cost = costs.get(last).unwrap().get(r).unwrap() + 1;
        let cost = current_cost + add_cost;

        if cost > max_cost {
            continue;
        }

        end = false;

        current.push(r.clone());

        for p in valid_permutations(costs, &remaining, current, cost, max_cost) {
            new.push(p);
        }

        current.pop();
    }

    if end {
        new.push(current.clone());
    }

    new
}

fn get_score(
    costs: &HashMap<String, HashMap<String, usize>>,
    valves: &HashMap<Identifier, Valve>,
    path: &[Identifier],
    max_minute: i32,
) -> usize {
    let mut minute: i32 = 1;
    let mut releasing_pm: i32 = 0;
    let mut released: i32 = 0;

    let mut current = &path[0];
    for node in &path[1..] {
        // let path = get_path(valves, current, node);
        let path_costs = costs.get(current).unwrap().get(node).unwrap();

        let flow_rate = valves.get(node).unwrap().flow_rate;

        current = node;
        // let time_diff = (path.len() + 1) as i32;
        let time_diff = (path_costs + 1) as i32;
        minute += time_diff;
        released += releasing_pm * time_diff;
        releasing_pm += flow_rate;
    }

    while minute <= max_minute {
        minute += 1;
        released += releasing_pm;
    }

    released as usize
}

fn part1(valves: &HashMap<Identifier, Valve>) -> usize {
    let path_costs = get_all_paths_costs(valves);

    let valve_ids: Vec<Identifier> = path_costs.keys().cloned().collect();

    let aa = String::from("AA");
    let mut without_aa = valve_ids;
    let aa_index = without_aa.iter().position(|x| **x == aa).unwrap();
    without_aa.remove(aa_index);
    let mut start = vec![aa];
    let permutations = valid_permutations(&path_costs, &without_aa, &mut start, 0, 30);
    println!("perm: {}", permutations.len());

    permutations
        .iter()
        .map(|perm| get_score(&path_costs, valves, perm, 30))
        .max()
        .unwrap()
}

fn valid_permutations2(
    valves: &HashMap<Identifier, Valve>,
    costs: &HashMap<String, HashMap<String, usize>>,
    remaining: &Vec<String>,
) -> usize {
    let mut start = vec![String::from("AA")];
    let paths = valid_permutations(costs, remaining, &mut start, 0, 26);

    let mut paths_with_scores: Vec<(usize, Vec<Identifier>)> = paths
        .into_iter()
        .map(|path| {
            let score = get_score(costs, valves, &path, 26);
            (score, path)
        })
        .collect();

    paths_with_scores.sort_by(|path1, path2| path1.0.partial_cmp(&path2.0).unwrap());
    paths_with_scores.reverse();
    let max_score = paths_with_scores.first().unwrap().0;

    let aa = String::from("AA");
    let mut pairs = vec![];
    for (my_score, my_path) in &paths_with_scores {
        if my_score * 2 < max_score {
            break;
        }

        'x: for (elephant_score, elephant_path) in &paths_with_scores {
            if elephant_score * 2 < max_score {
                break;
            }

            for my_node in my_path {
                if *my_node == aa {
                    continue;
                }

                for elephant_node in elephant_path {
                    if *my_node == *elephant_node {
                        continue 'x;
                    }
                }
            }

            pairs.push((my_path, elephant_path));
        }
    }

    let mut max_score = 0;
    for (first, second) in pairs {
        let combined_score =
            get_score(costs, valves, first, 26) + get_score(costs, valves, second, 26);
        max_score = max_score.max(combined_score);
    }

    max_score
}

fn part2(valves: &HashMap<Identifier, Valve>) -> usize {
    let path_costs = get_all_paths_costs(valves);

    let valve_ids: Vec<Identifier> = path_costs.keys().cloned().collect();

    let aa = String::from("AA");
    let mut without_aa = valve_ids;
    let aa_index = without_aa.iter().position(|x| **x == aa).unwrap();
    without_aa.remove(aa_index);
    valid_permutations2(valves, &path_costs, &without_aa)
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
