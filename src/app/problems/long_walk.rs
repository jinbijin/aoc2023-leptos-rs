#[cfg(feature = "ssr")]
mod forest_trails;

#[cfg(feature = "ssr")]
use self::forest_trails::ForestTrails;

use crate::as_server_fn_with_timing;

// Format notes:
// * Vertices are the squares surrounded by slopes, and equivalently the "branch points" in the path
//   (including start and end points)
// * There is a unique path between each pair of directly connected vertices, if any

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let trails = input.parse::<ForestTrails>().unwrap();
        let graph = trails.as_graph(part == ProblemPart::Part2);

        graph.longest_distance_between(trails.starting_vertex(), trails.ending_vertex()).unwrap()
    }
}