use super::{influx::InfluxItem, influx_range::InfluxRange, workflow::{InstructionResult, SplitResult, Workflow}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessorResult {
    Accept,
    Reject
}

#[derive(Debug)]
pub struct Processor<'a> {
    workflow: Workflow<'a>
}

impl<'a> Processor<'a> {
    pub fn from(workflow: Workflow<'a>) -> Processor<'a> {
        Processor {
            workflow
        }
    }

    pub fn process(&self, influx_item: &InfluxItem, name: &'a str) -> ProcessorResult {
        let workflow_item = self.workflow.get(name).unwrap();
        let result = workflow_item.instructions.iter()
            .find(|instruction| instruction.condition.is_satisfied_by(influx_item))
            .map_or(workflow_item.final_instruction, |instruction| instruction.then);

        match result {
            InstructionResult::Accept => ProcessorResult::Accept,
            InstructionResult::Reject => ProcessorResult::Reject,
            InstructionResult::Goto(name) => self.process(influx_item, name),
        }
    }

    pub fn process_range(&self) -> Vec<InfluxRange> {
        let initial_name: &'a str = "in";

        let mut item_stack = vec![(initial_name, InfluxRange::default())];
        let mut result: Vec<InfluxRange> = Vec::new();

        while let Some((name, influx_range)) = item_stack.pop() {
            let mut influx_range = Some(influx_range);

            let workflow_item = self.workflow.get(name).unwrap();

            for instruction in workflow_item.instructions.iter() {
                if let Some(range) = influx_range {
                    match instruction.condition.split_range(&range) {
                        SplitResult::Then(then) => {
                            influx_range = None;
                            match instruction.then {
                                InstructionResult::Accept => result.push(then),
                                InstructionResult::Reject => (),
                                InstructionResult::Goto(name) => item_stack.push((name, then))
                            };
                        },
                        SplitResult::Next(next) => {
                            influx_range = Some(next);
                        },
                        SplitResult::Split { then, next } => {
                            influx_range = Some(next);
                            match instruction.then {
                                InstructionResult::Accept => result.push(then),
                                InstructionResult::Reject => (),
                                InstructionResult::Goto(name) => item_stack.push((name, then))
                            }
                        }
                    }
                }
            }

            if let Some(range) = influx_range {
                match workflow_item.final_instruction {
                    InstructionResult::Accept => result.push(range),
                    InstructionResult::Reject => (),
                    InstructionResult::Goto(name) => item_stack.push((name, range))
                };
            }
        }

        result
    }
}