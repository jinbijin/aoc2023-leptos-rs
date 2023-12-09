mod haunted_wasteland;

use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum ProblemPart {
    Part1,
    Part2
}

#[component(transparent)]
pub fn Routes() -> impl IntoView {
    view! {
        <Route path="problems" view=Problems>
            <Route path="haunted-wasteland" view=haunted_wasteland::Main />
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
