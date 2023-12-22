#[cfg(feature = "ssr")]
use std::fmt::Display;
#[cfg(feature = "ssr")]
use std::time::{Duration, Instant};
#[cfg(feature = "ssr")]
use super::ProblemPart;
#[cfg(feature = "ssr")]
use leptos::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimedSolutionResponse {
    pub solution: String,
    pub duration_description: String,
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
pub fn with_timing<T: Display>(inner: &dyn Fn(ProblemPart, String) -> T, part: ProblemPart, input: String) -> Result<TimedSolutionResponse, ServerFnError> {
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

#[macro_export]
macro_rules! as_server_fn_with_timing {
    ($solver:item) => {
        use leptos::*;
        use crate::app::problems::{ProblemPart, timing::TimedSolutionResponse};
        #[cfg(feature = "ssr")]
        use crate::app::problems::timing::with_timing;

        #[cfg(feature = "ssr")]
        $solver

        #[server(Solve)]
        pub async fn solve_with_timing(part: ProblemPart, input: String) -> Result<TimedSolutionResponse, ServerFnError> {
            with_timing(&solve, part, input)
        }
    }
}

