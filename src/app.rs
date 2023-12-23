mod common;
#[cfg(feature = "ssr")]
mod math;
mod problems;

use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/aoc2023.css"/>
        <Title text="Advent of Code 2023"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
        }>
            <main>
                <Routes>
                    <Route path="" view=PageLayout>
                        <problems::Routes />
                        <Route path="" view=HomePage />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn PageLayout() -> impl IntoView {
    view! {
        <h1>"Advent of Code 2023"</h1>
        <nav>
            <ul>
                <li><A href="problems">Problems</A></li>
            </ul>
        </nav>
        <Outlet />
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <p>"Welcome to Advent of Code 2023!"</p>
    }
}
