mod haunted_wasteland;
mod pipe_maze;
mod cosmic_expansion;
mod hot_springs;
mod point_of_incidence;
mod parabolic_reflector_dish;
mod lens_library;
mod the_floor_will_be_lava;
mod clumsy_crucible;
mod lavaduct_lagoon;

#[cfg(feature = "ssr")]
use std::fmt::Display;
#[cfg(feature = "ssr")]
use std::time::{Duration, Instant};
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ProblemPart {
    Part1,
    Part2
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimedSolutionResponse {
    solution: String,
    duration_description: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TimedSolution<T: Display> {
    solution: T,
    duration: Duration,
}

#[cfg(feature = "ssr")]
impl<T: Display> From<TimedSolution<T>> for TimedSolutionResponse {
    fn from(value: TimedSolution<T>) -> Self {
        Self {
            solution: value.solution.to_string(),
            duration_description: get_description(value.duration),
        }
    }
}

#[cfg(feature = "ssr")]
fn with_timing<T: Display>(inner: &dyn Fn(ProblemPart, String) -> T, part: ProblemPart, input: String) -> Result<TimedSolutionResponse, ServerFnError> {
    let instant = Instant::now();
    let solution = inner(part, input);
    let duration = instant.elapsed();
    let timed_solution = TimedSolution {
        solution,
        duration
    };

    Ok(timed_solution.into())
}

#[cfg(feature = "ssr")]
fn get_description(duration: Duration) -> String {
    let nanos = duration.as_nanos();
    if nanos < 10_000 { // 10 us
        format!("{}.000 ns", nanos)
    } else if nanos < 10_000_000 { // 10 ms
        let fraction = nanos % 1000;
        let amount = nanos / 1000;
        format!("{}.{:0>3} \u{03BC}s", amount, fraction)
    } else if nanos < 10_000_000_000 { // 10 s
        let micros = nanos / 1000;
        let fraction = micros % 1000;
        let amount = micros / 1000;
        format!("{}.{:0>3} ms", amount, fraction)
    } else if nanos < 600_000_000_000 { // 10 min
        let millis = nanos / 1_000_000;
        let fraction = millis % 1000;
        let amount = millis / 1000;
        format!("{}.{:0>3} s", amount, fraction)
    } else {
        "Too long, please optimize!".to_string()
    }
}

#[component(transparent)]
pub fn Routes() -> impl IntoView {
    view! {
        <Route path="problems" view=Problems>
            <Route path="haunted-wasteland" view=haunted_wasteland::Main />
            <Route path="pipe-maze" view=pipe_maze::Main />
            <Route path="cosmic-expansion" view=cosmic_expansion::Main />
            <Route path="hot-springs" view=hot_springs::Main />
            <Route path="point-of-incidence" view=point_of_incidence::Main />
            <Route path="parabolic-reflector-dish" view=parabolic_reflector_dish::Main />
            <Route path="lens-library" view=lens_library::Main />
            <Route path="the-floor-will-be-lava" view=the_floor_will_be_lava::Main />
            <Route path="clumsy-crucible" view=clumsy_crucible::Main />
            <Route path="lavaduct-lagoon" view=lavaduct_lagoon::Main />
            <Route path="" view=NoProblem />
        </Route>
    }
}

#[component]
pub fn Problems() -> impl IntoView {
    view! {
        <nav>
            <ul>
                <li><A href="haunted-wasteland">"Day 8: Haunted Wasteland"</A></li>
                <li><A href="pipe-maze">"Day 10: Pipe Maze"</A></li>
                <li><A href="cosmic-expansion">"Day 11: Cosmic Expansion"</A></li>
                <li><A href="hot-springs">"Day 12: Hot Springs"</A></li>
                <li><A href="point-of-incidence">"Day 13: Point of Incidence"</A></li>
                <li><A href="parabolic-reflector-dish">"Day 14: Parabolic Reflector Dish"</A></li>
                <li><A href="lens-library">"Day 15: Lens Library"</A></li>
                <li><A href="the-floor-will-be-lava">"Day 16: The Floor Will Be Lava"</A></li>
                <li><A href="clumsy-crucible">"Day 17: Clumsy Crucible"</A></li>
                <li><A href="lavaduct-lagoon">"Day 18: Lavaduct Lagoon"</A></li>
            </ul>
        </nav>
        <Outlet />
    }
}

#[component]
fn NoProblem() -> impl IntoView {
    view! {
        <p>Please select a problem</p>
    }
}

#[component]
fn ProblemForm<T>(name: &'static str, action: Action<T, Result<String, ServerFnError>>) -> impl IntoView
where
    T: Clone + ServerFn
{
    let value = action.value();

    view! {
        <h2>{ name }</h2>
        <ActionForm action=action>
            <div>
                <label for="problem-part">Problem part</label>
                <select name="part" id="problem-part">
                    <option value="Part1">Part 1</option>
                    <option value="Part2">Part 2</option>
                </select>
            </div>
            <div>
                <label for="problem-input">Problem input</label>
                <textarea name="input" id="problem-input" />
            </div>
            <input type="submit" value="Solve!"/>
        </ActionForm>
        <Show when=move || if let Some(Ok(_)) = value() { true } else { false }>
            <p>Result: <input readonly class="numeric" prop:value=move || if let Some(Ok(value)) = value() { value } else { "".to_string() }/></p>
        </Show>
    }
}

#[component]
fn TimedProblemForm<T>(name: &'static str, action: Action<T, Result<TimedSolutionResponse, ServerFnError>>) -> impl IntoView
where
    T: Clone + ServerFn
{
    let value = action.value();

    view! {
        <h2>{ name }</h2>
        <ActionForm action=action>
            <div>
                <label for="problem-part">Problem part</label>
                <select name="part" id="problem-part">
                    <option value="Part1">Part 1</option>
                    <option value="Part2">Part 2</option>
                </select>
            </div>
            <div>
                <label for="problem-input">Problem input</label>
                <textarea name="input" id="problem-input" />
            </div>
            <input type="submit" value="Solve!"/>
        </ActionForm>
        <Show when=move || if let Some(Ok(_)) = value() { true } else { false }>
            <p>Solution: <input readonly class="numeric" prop:value=move || if let Some(Ok(value)) = value() { value.solution } else { "".to_string() }/></p>
            <p>"Computed in " { value().map(|value| value.map(|value| value.duration_description)) }</p>
        </Show>
    }
}
