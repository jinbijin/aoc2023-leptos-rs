#[cfg(feature = "ssr")]
mod mirror;
#[cfg(feature = "ssr")]
mod volcanic_land;
#[cfg(feature = "ssr")]
mod volcanic_patch;

use leptos::*;
use super::{ProblemForm, ProblemPart};
#[cfg(feature = "ssr")]
use self::volcanic_land::VolcanicLand;

#[server(PointOfIncidence)]
pub async fn solve(part: ProblemPart, input: String) -> Result<String, ServerFnError> {
    let lands = input.split("\n\n")
        .map(|land_str| -> VolcanicLand { land_str.into() });

    let result: usize = match part {
        ProblemPart::Part1 =>
            lands.map(|land| land.find_mirror())
                .map(|mirror| mirror.weight())
                .sum(),
        ProblemPart::Part2 =>
            lands.map(|land| land.find_smudged_mirror())
                .map(|mirror| mirror.weight())
                .sum(),
    };

    Ok(format!("{}", result))
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<PointOfIncidence>();

    view! {
        <ProblemForm name="Day 13: Point of Incidence" action=action />
    }
}
