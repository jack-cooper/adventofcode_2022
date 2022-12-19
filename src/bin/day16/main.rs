use std::{
    collections::{HashMap, HashSet},
    fs,
};

use adventofcode_2022::{AnyResult, CustomError};
use once_cell::sync::Lazy;
use petgraph::{
    algo::floyd_warshall,
    prelude::{Graph, NodeIndex},
};
use regex::Regex;

type RoomGraph<'a> = Graph<Valve<'a>, u32, petgraph::Undirected>;

static ROOM_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("Valve (?P<id>[A-Z]{2}) has flow rate=(?P<flow_rate>\\d+); tunnels? leads? to valves? (?P<edges>(?:[A-Z]{2}(?:, )?)+)")
        .unwrap()
});

#[derive(Clone, Debug)]
struct Valve<'a> {
    flow_rate: i32,
    id: RoomId<'a>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct RoomId<'a>(&'a str);

impl<'a> TryFrom<&'a str> for RoomId<'a> {
    type Error = CustomError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if s.len() == 2 {
            Ok(Self(s))
        } else {
            Err(CustomError {
                msg: "A room ID should have a length of 2.".into(),
            })
        }
    }
}

fn create_graph(input: &str) -> Result<(RoomGraph, HashMap<RoomId, NodeIndex>), CustomError> {
    let mut graph: RoomGraph = Graph::new_undirected();

    let mut id_to_index: HashMap<RoomId, NodeIndex> = HashMap::new();

    let mut edges: HashMap<RoomId, Vec<RoomId>> = HashMap::new();

    for line in input.lines() {
        let captures = ROOM_REGEX.captures(line).ok_or(CustomError {
            msg: "Malformed room descriptor detected.".into(),
        })?;

        let id = captures
            .name("id")
            .ok_or(CustomError {
                msg: "No id detected.".into(),
            })?
            .as_str();

        let flow_rate: i32 = captures
            .name("flow_rate")
            .ok_or(CustomError {
                msg: "No flow rate detected.".into(),
            })?
            .as_str()
            .parse()
            .map_err(|_| CustomError {
                msg: "Non-numeric flow rate detected.".into(),
            })?;

        let valve = Valve {
            flow_rate,
            id: id.try_into()?,
        };

        let index = graph.add_node(valve);
        id_to_index.insert(id.try_into()?, index);

        let node_edges = captures
            .name("edges")
            .ok_or(CustomError {
                msg: "No edges detected.".into(),
            })?
            .as_str();

        edges.insert(
            id.try_into()?,
            node_edges.split(", ").flat_map(RoomId::try_from).collect(),
        );
    }

    for (id, adjacent_node_ids) in edges {
        let node_index = id_to_index[&id];

        for adjacent_node_id in adjacent_node_ids {
            let adjacent_node_index = id_to_index[&adjacent_node_id];

            graph.add_edge(node_index, adjacent_node_index, 1);
        }
    }

    Ok((graph, id_to_index))
}

fn get_route_lengths(graph: &RoomGraph) -> HashMap<(NodeIndex, NodeIndex), i32> {
    let mut route_lengths =
        floyd_warshall(&graph, |_| 1).expect("Constructed a graph with a negative cycle.");

    route_lengths.retain(|&(from, to), _| {
        (graph[from].flow_rate > 0 && graph[to].flow_rate > 0) || graph[from].id == RoomId("AA")
    });

    route_lengths
}

fn main() -> AnyResult {
    let input = fs::read_to_string("src/bin/day16/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> AnyResult {
    let (graph, id_to_index) = create_graph(input)?;

    let route_lengths = get_route_lengths(&graph);

    let mut valid_targets: HashSet<NodeIndex> = graph
        .node_indices()
        .filter(|&index| graph[index].flow_rate > 0)
        .collect();

    let mut minutes_remaining = 30;
    let mut current_index: NodeIndex = id_to_index[&RoomId("AA")];
    let mut maybe_target_index: Option<NodeIndex> = None;
    let mut steps_to_target = 0;
    let mut total_pressure_released = 0;

    while minutes_remaining > 0 {
        if maybe_target_index.is_none() {
            maybe_target_index =
                valid_targets
                    .iter()
                    .copied()
                    .max_by(|&target_index1, &target_index2| {
                        let route_length1 = route_lengths[&(current_index, target_index1)];
                        let route_length2 = route_lengths[&(current_index, target_index2)];

                        let valve1 = &graph[target_index1];
                        let valve2 = &graph[target_index2];

                        let exp = 2.0;

                        let (value1, value2) = if minutes_remaining > 10 {
                            let value1 = valve1.flow_rate as f32 / (route_length1 as f32).powf(exp);
                            let value2 = valve2.flow_rate as f32 / (route_length2 as f32).powf(exp);

                            (value1, value2)
                        } else {
                            let value1 =
                                (valve1.flow_rate * (minutes_remaining - 1 - route_length1)) as f32;
                            let value2 =
                                (valve2.flow_rate * (minutes_remaining - 1 - route_length2)) as f32;

                            (value1, value2)
                        };

                        value1.total_cmp(&value2)
                    });

            if let Some(target_index) = maybe_target_index {
                let route_length = route_lengths[&(current_index, target_index)];

                let valve = &graph[target_index];

                let time_active = (minutes_remaining - 1 - route_length).max(0);

                let pressure_released = valve.flow_rate * time_active;

                total_pressure_released += pressure_released;
                steps_to_target = route_length;

                println!(
                    "{:?} turned on with {} minutes remaining, for {} value ({} * {}).",
                    graph[target_index].id.0,
                    minutes_remaining - route_length,
                    pressure_released,
                    valve.flow_rate,
                    time_active
                );
            }
        }

        if let Some(target_index) = maybe_target_index {
            if current_index == target_index {
                maybe_target_index = None;
                valid_targets.remove(&target_index);
            } else {
                steps_to_target -= 1;

                if steps_to_target == 0 {
                    current_index = target_index;
                }
            }
        }

        minutes_remaining -= 1;
    }

    println!("Part 1 answer = {total_pressure_released}");
    Ok(())
}

fn part2(_input: &str) -> AnyResult {
    Ok(())
}
