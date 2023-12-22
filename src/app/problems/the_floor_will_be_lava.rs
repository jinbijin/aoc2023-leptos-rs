#[cfg(feature = "ssr")]
mod beams;
#[cfg(feature = "ssr")]
mod beam_generator;
#[cfg(feature = "ssr")]
mod contraption;

#[cfg(feature = "ssr")]
use self::{beam_generator::{BeamGenerator, BeamGeneratorState}, contraption::Contraption};

use crate::as_server_fn_with_timing;

as_server_fn_with_timing! {
    fn solve(part: ProblemPart, input: String) -> usize {
        let contraption: Contraption = input.as_str().into();
        match part {
            ProblemPart::Part1 => {
                let generator = BeamGenerator::from(&contraption, BeamGeneratorState::HorizontalForward { y: 0, x_from: None });
                let collection = generator.process();

                collection.energized_tile_count()
            },
            ProblemPart::Part2 => {
                (0..contraption.width).map(|x| BeamGeneratorState::VerticalForward { x, y_from: None })
                    .chain((0..contraption.width).map(|x| BeamGeneratorState::VerticalBackward { x, y_to: None }))
                    .chain((0..contraption.height).map(|y| BeamGeneratorState::HorizontalForward { y, x_from: None }))
                    .chain((0..contraption.height).map(|y| BeamGeneratorState::HorizontalBackward { y, x_to: None }))
                    .map(|state| BeamGenerator::from(&contraption, state))
                    .map(|generator| generator.process())
                    .map(|collection| collection.energized_tile_count())
                    .max().unwrap()
            }
        }
    }
}
