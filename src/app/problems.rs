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
mod aplenty;
mod pulse_propagation;
mod step_counter;
mod sand_slabs;
mod timing;

use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use self::timing::TimedSolutionResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ProblemPart {
    Part1,
    Part2
}

macro_rules! problem_routes_def {
    ($( $title:literal at $module:ident ),*) => {
        #[component(transparent)]
        pub fn Routes() -> impl IntoView {
            view! {
                <Route path="problems" view=Problems>
                    $(
                        <Route path=stringify!($module)
                            view=|| view! {
                                <TimedProblemForm action={ create_server_action::<$module::Solve>() }>$title</TimedProblemForm>
                            }
                        />
                    )*
                    <Route path="" view=NoProblem />
                </Route>
            }
        }

        #[component]
        pub fn Problems() -> impl IntoView {
            view! {
                <nav>
                    <ul>
                        $(
                            <li><A href=stringify!($module)>$title</A></li>
                        )*
                    </ul>
                </nav>
                <Outlet />
            }
        }
    };
}

problem_routes_def! {
    "Day 8: Haunted Wasteland" at haunted_wasteland,
    "Day 10: Pipe Maze" at pipe_maze,
    "Day 11: Cosmic Expansion" at cosmic_expansion,
    "Day 12: Hot Springs" at hot_springs,
    "Day 13: Point of Incidence" at point_of_incidence,
    "Day 14: Parabolic Reflector Dish" at parabolic_reflector_dish,
    "Day 15: Lens Library" at lens_library,
    "Day 16: The Floor Will Be Lava" at the_floor_will_be_lava,
    "Day 17: Clumsy Crucible" at clumsy_crucible,
    "Day 18: Lavaduct Lagoon" at lavaduct_lagoon,
    "Day 19: Aplenty" at aplenty,
    "Day 20: Pulse Propagation" at pulse_propagation,
    "Day 21: Step Counter" at step_counter,
    "Day 22: Sand Slabs" at sand_slabs
}

#[component]
fn NoProblem() -> impl IntoView {
    view! {
        <p>Please select a problem</p>
    }
}

#[component]
fn TimedProblemForm<T>(action: Action<T, Result<TimedSolutionResponse, ServerFnError>>, children: Children) -> impl IntoView
where
    T: Clone + ServerFn
{
    let value = action.value();

    view! {
        <h2>{ children() }</h2>
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
