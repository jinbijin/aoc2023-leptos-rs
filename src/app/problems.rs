mod haunted_wasteland;
mod pipe_maze;
mod cosmic_expansion;
mod hot_springs;
mod point_of_incidence;
mod parabolic_reflector_dish;

use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ProblemPart {
    Part1,
    Part2
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
