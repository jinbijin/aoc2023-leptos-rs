#[cfg(feature = "ssr")]
mod arrangement_counter;
#[cfg(feature = "ssr")]
mod spring_condition;

use leptos::*;
use super::{ProblemForm, ProblemPart};
#[cfg(feature = "ssr")]
use self::{arrangement_counter::ArrangementCounter, spring_condition::SpringCondition};

#[server(HotSprings)]
pub async fn solve(part: ProblemPart, input: String) -> Result<String, ServerFnError> {
    let arrangement_counter = ArrangementCounter::new();
    let result: usize = input.lines()
        .map(|line| {
            let (spring_conditions, damaged_group_sizes) = line.split_once(' ').unwrap();
            let mut spring_conditions: Vec<_> = spring_conditions.chars()
                .map(SpringCondition::read)
                .collect();
            let mut damaged_group_sizes: Vec<_> = damaged_group_sizes.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            if part == ProblemPart::Part2 {
                spring_conditions = vec![
                    spring_conditions.clone(),
                    vec![None],
                    spring_conditions.clone(),
                    vec![None],
                    spring_conditions.clone(),
                    vec![None],
                    spring_conditions.clone(),
                    vec![None],
                    spring_conditions.clone(),
                ]
                    .into_iter()
                    .flatten()
                    .collect();
                damaged_group_sizes = vec![
                    damaged_group_sizes.clone(),
                    damaged_group_sizes.clone(),
                    damaged_group_sizes.clone(),
                    damaged_group_sizes.clone(),
                    damaged_group_sizes.clone(),
                ]
                    .into_iter()
                    .flatten()
                    .collect();
            }

            arrangement_counter.count_arrangements(&spring_conditions, &damaged_group_sizes)
        })
        .sum();

    Ok(format!("{}", result))
}

#[component]
pub fn Main() -> impl IntoView {
    let action = create_server_action::<HotSprings>();

    view! {
        <ProblemForm name="Day 12: Hot Springs" action=action />
    }
}
