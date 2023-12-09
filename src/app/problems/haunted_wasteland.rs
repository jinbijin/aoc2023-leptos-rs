mod direction;
mod node;
mod eventually_periodic;
mod network_path;
mod crossroads;
mod network;

use leptos::*;
use super::{ProblemPart, ProblemForm};

use self::{direction::Direction, node::Node, network::Network};

fn solve_1(network: Network, directions: Vec<Direction>) -> Option<usize> {
    let node = Node::new("AAA");
    let path = network.get_path(node, &directions);
    let indices = path.get_indices_satisfying(|node| *node == Node::new("ZZZ"));

    indices.get_first_index()
}

fn solve_2(network: Network, directions: Vec<Direction>) -> Option<usize> {
    let nodes = network.get_starting_nodes();

    let mut cycles = nodes.into_iter().map(|node| network.get_path(node, &directions))
        .map(|path| path.get_indices_satisfying(|node| node.is_end()));

    let mut product = cycles.next().unwrap();
    while let Some(cycle) = cycles.next() {
        product = product.mul(cycle);
    }

    product.get_first_index()
}

#[server(HauntedWasteland)]
pub async fn solve(part: ProblemPart, input: String) -> Result<String, ServerFnError> {
    let (directions, network) = input.split_once("\n\n").unwrap();
    let directions = directions.chars().map(|x| -> Direction { x.into() }).collect::<Vec<Direction>>();

    let network: Network = network.into();

    match part {
        ProblemPart::Part1 => {
            if let Some(solution) = solve_1(network, directions) {
                Ok(solution.to_string())
            } else {
                Ok(format!("Never"))
            }
        },
        ProblemPart::Part2 => {
            if let Some(solution) = solve_2(network, directions) {
                Ok(solution.to_string())
            } else {
                Ok(format!("Never"))
            }
        }
    }
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<HauntedWasteland>();

    view! {
        <ProblemForm name="Day 8: Haunted Wasteland" action=action />
    }
}
