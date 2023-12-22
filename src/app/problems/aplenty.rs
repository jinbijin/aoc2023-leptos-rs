#[cfg(feature = "ssr")]
mod workflow;
#[cfg(feature = "ssr")]
mod influx;
#[cfg(feature = "ssr")]
mod influx_range;
#[cfg(feature = "ssr")]
mod processor;

#[cfg(feature = "ssr")]
use self::{influx::Influx, workflow::Workflow, processor::{Processor, ProcessorResult}};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let (workflow, influx) = input.split_once("\n\n").unwrap();
        let workflow = Workflow::from_str(workflow);
        let influx = Influx::from_str(influx);
        let processor = Processor::from(workflow);

        match part {
            ProblemPart::Part1 =>
                influx.iter()
                    .filter(move |influx_item| processor.process(influx_item, "in") == ProcessorResult::Accept)
                    .map(|influx_item| influx_item.value())
                    .sum(),
            ProblemPart::Part2 =>
                processor.process_range().into_iter().map(|r| r.size()).sum()
        }
    }
}
