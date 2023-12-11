#[cfg(feature = "ssr")]
mod cosmic_coords;
#[cfg(feature = "ssr")]
mod cosmic_grid;
#[cfg(feature = "ssr")]
mod cosmic_interval;

use leptos::*;
use super::{ProblemPart, ProblemForm};

#[cfg(feature = "ssr")]
use self::cosmic_grid::CosmicGrid;

#[cfg(feature = "ssr")]
fn solve_grid(cosmic_grid: &CosmicGrid, expansion_factor: usize) -> usize {
    cosmic_grid.weight(expansion_factor)
}

#[server(CosmicExpansion)]
pub async fn solve(part: ProblemPart, input: String) -> Result<String, ServerFnError> {
    let cosmic_grid: CosmicGrid = input.as_str().into();
    let expansion_factor = match part {
        ProblemPart::Part1 => 2usize,
        ProblemPart::Part2 => 1000000usize
    };

    Ok(format!("{}", solve_grid(&cosmic_grid, expansion_factor)))
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<CosmicExpansion>();

    view! {
        <ProblemForm name="Day 11: Cosmic Expansion" action=action />
    }
}
