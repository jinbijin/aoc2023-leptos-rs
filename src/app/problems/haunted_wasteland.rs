#[cfg(feature="ssr")]
mod direction;
#[cfg(feature="ssr")]
mod node;
#[cfg(feature="ssr")]
mod eventually_periodic;
#[cfg(feature="ssr")]
mod network_path;
#[cfg(feature="ssr")]
mod crossroads;
#[cfg(feature="ssr")]
mod network;

#[cfg(feature="ssr")]
use self::{direction::Direction, node::Node, network::Network};

use crate::as_server_fn_with_timing;

#[cfg(feature="ssr")]
fn solve_1(network: Network, directions: Vec<Direction>) -> Option<usize> {
    let node = Node::new("AAA");
    let path = network.get_path(node, &directions);
    let indices = path.get_indices_satisfying(|node| *node == Node::new("ZZZ"));

    indices.get_first_index()
}

#[cfg(feature="ssr")]
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

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> String {
        let (directions, network) = input.split_once("\n\n").unwrap();
        let directions = directions.chars().map(|x| -> Direction { x.into() }).collect::<Vec<Direction>>();

        let network: Network = network.into();

        match part {
            ProblemPart::Part1 => {
                if let Some(solution) = solve_1(network, directions) {
                    solution.to_string()
                } else {
                    "Never".to_string()
                }
            },
            ProblemPart::Part2 => {
                if let Some(solution) = solve_2(network, directions) {
                    solution.to_string()
                } else {
                    "Never".to_string()
                }
            }
        }
    }
}
