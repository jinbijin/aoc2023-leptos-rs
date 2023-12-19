use std::collections::HashMap;
use std::ops::Deref;
use super::{influx::InfluxItem, influx_range::InfluxRange};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SplitResult<T> {
    Then(T),
    Split { then: T, next: T },
    Next(T)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstructionProperty {
    Xtreme,
    Musical,
    Aerodynamic,
    Shiny
}

impl InstructionProperty {
    fn from_str(s: &str) -> InstructionProperty {
        match s {
            "x" => InstructionProperty::Xtreme,
            "m" => InstructionProperty::Musical,
            "a" => InstructionProperty::Aerodynamic,
            "s" => InstructionProperty::Shiny,
            _ => panic!("Invalid property {s}")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstructionOperator {
    Less,
    Greater
}

impl InstructionOperator {
    fn from_str(s: &str) -> InstructionOperator {
        match s {
            "<" => InstructionOperator::Less,
            ">" => InstructionOperator::Greater,
            _ => panic!("Invalid operator")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionCondition {
    pub property: InstructionProperty,
    pub operator: InstructionOperator,
    pub value: usize
}

impl InstructionCondition {
    fn from_str(s: &str) -> InstructionCondition {
        let (property, rest) = s.split_at(1);
        let (operator, value) = rest.split_at(1);

        let property = InstructionProperty::from_str(property);
        let operator = InstructionOperator::from_str(operator);
        let value = value.parse::<usize>().unwrap();

        InstructionCondition {
            property,
            operator,
            value
        }
    }

    pub fn is_satisfied_by(&self, influx_item: &InfluxItem) -> bool {
        match self.property {
            InstructionProperty::Xtreme =>
                match self.operator {
                    InstructionOperator::Less => influx_item.xtreme < self.value,
                    InstructionOperator::Greater => influx_item.xtreme > self.value,
                },
            InstructionProperty::Musical =>
                match self.operator {
                    InstructionOperator::Less => influx_item.musical < self.value,
                    InstructionOperator::Greater => influx_item.musical > self.value,
                },
            InstructionProperty::Aerodynamic =>
                match self.operator {
                    InstructionOperator::Less => influx_item.aerodynamic < self.value,
                    InstructionOperator::Greater => influx_item.aerodynamic > self.value,
                },
            InstructionProperty::Shiny =>
                match self.operator {
                    InstructionOperator::Less => influx_item.shiny < self.value,
                    InstructionOperator::Greater => influx_item.shiny > self.value,
                },
        }
    }

    pub fn split_range(&self, influx_range: &InfluxRange) -> SplitResult<InfluxRange> {
        match self.property {
            InstructionProperty::Xtreme =>
                match self.operator {
                    InstructionOperator::Less =>
                        if influx_range.xtreme.from < self.value && self.value <= influx_range.xtreme.to {
                            SplitResult::Split {
                                then: influx_range.with_xtreme_to(self.value - 1),
                                next: influx_range.with_xtreme_from(self.value),
                            }
                        } else if influx_range.xtreme.from >= self.value {
                            SplitResult::Next(*influx_range)
                        } else {
                            SplitResult::Then(*influx_range)
                        },
                    InstructionOperator::Greater =>
                        if influx_range.xtreme.from <= self.value && self.value < influx_range.xtreme.to {
                            SplitResult::Split {
                                then: influx_range.with_xtreme_from(self.value + 1),
                                next: influx_range.with_xtreme_to(self.value),
                            }
                        } else if influx_range.xtreme.from > self.value {
                            SplitResult::Then(*influx_range)
                        } else {
                            SplitResult::Next(*influx_range)
                        },
                },
            InstructionProperty::Musical =>
                match self.operator {
                    InstructionOperator::Less =>
                        if influx_range.musical.from < self.value && self.value <= influx_range.musical.to {
                            SplitResult::Split {
                                then: influx_range.with_musical_to(self.value - 1),
                                next: influx_range.with_musical_from(self.value),
                            }
                        } else if influx_range.musical.from >= self.value {
                            SplitResult::Next(*influx_range)
                        } else {
                            SplitResult::Then(*influx_range)
                        },
                    InstructionOperator::Greater =>
                        if influx_range.musical.from <= self.value && self.value < influx_range.musical.to {
                            SplitResult::Split {
                                then: influx_range.with_musical_from(self.value + 1),
                                next: influx_range.with_musical_to(self.value),
                            }
                        } else if influx_range.musical.from > self.value {
                            SplitResult::Then(*influx_range)
                        } else {
                            SplitResult::Next(*influx_range)
                        },
                },
            InstructionProperty::Aerodynamic =>
                match self.operator {
                    InstructionOperator::Less =>
                        if influx_range.aerodynamic.from < self.value && self.value <= influx_range.aerodynamic.to {
                            SplitResult::Split {
                                then: influx_range.with_aerodynamic_to(self.value - 1),
                                next: influx_range.with_aerodynamic_from(self.value),
                            }
                        } else if influx_range.aerodynamic.from >= self.value {
                            SplitResult::Next(*influx_range)
                        } else {
                            SplitResult::Then(*influx_range)
                        },
                    InstructionOperator::Greater =>
                        if influx_range.aerodynamic.from <= self.value && self.value < influx_range.aerodynamic.to {
                            SplitResult::Split {
                                then: influx_range.with_aerodynamic_from(self.value + 1),
                                next: influx_range.with_aerodynamic_to(self.value),
                            }
                        } else if influx_range.aerodynamic.from > self.value {
                            SplitResult::Then(*influx_range)
                        } else {
                            SplitResult::Next(*influx_range)
                        },
                },
            InstructionProperty::Shiny =>
                match self.operator {
                    InstructionOperator::Less =>
                        if influx_range.shiny.from < self.value && self.value <= influx_range.shiny.to {
                            SplitResult::Split {
                                then: influx_range.with_shiny_to(self.value - 1),
                                next: influx_range.with_shiny_from(self.value),
                            }
                        } else if influx_range.shiny.from >= self.value {
                            SplitResult::Next(*influx_range)
                        } else {
                            SplitResult::Then(*influx_range)
                        },
                    InstructionOperator::Greater =>
                        if influx_range.shiny.from <= self.value && self.value < influx_range.shiny.to {
                            SplitResult::Split {
                                then: influx_range.with_shiny_from(self.value + 1),
                                next: influx_range.with_shiny_to(self.value),
                            }
                        } else if influx_range.shiny.from > self.value {
                            SplitResult::Then(*influx_range)
                        } else {
                            SplitResult::Next(*influx_range)
                        },
                },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstructionResult<'a> {
    Accept,
    Reject,
    Goto(&'a str)
}

impl<'a> InstructionResult<'a> {
    fn from_str(s: &'a str) -> InstructionResult<'a> {
        match s {
            "A" => InstructionResult::Accept,
            "R" => InstructionResult::Reject,
            s => InstructionResult::Goto(s)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Instruction<'a> {
    pub condition: InstructionCondition,
    pub then: InstructionResult<'a>
}

impl<'a> Instruction<'a> {
    fn from_str(s: &'a str) -> Instruction<'a> {
        let (condition, then) = s.split_once(":").unwrap();
        let condition = InstructionCondition::from_str(condition);
        let then = InstructionResult::from_str(then);

        Instruction {
            condition,
            then
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WorkflowItem<'a> {
    pub instructions: Vec<Instruction<'a>>,
    pub final_instruction: InstructionResult<'a>,
}

impl<'a> WorkflowItem<'a> {
    fn from_str(s: &'a str) -> WorkflowItem<'a> {
        let mut items: Vec<_> = s.split(',').collect();
        let last = items.pop().unwrap();

        let final_instruction = InstructionResult::from_str(last);
        let instructions: Vec<_> = items.into_iter().map(Instruction::from_str)
            .collect();

        WorkflowItem {
            instructions,
            final_instruction
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow<'a>(HashMap<&'a str, WorkflowItem<'a>>);

impl<'a> Workflow<'a> {
    pub fn from_str(s: &'a str) -> Workflow<'a> {
        Workflow(s.lines().map(|line| {
            let (name, item) = line.strip_suffix("}").unwrap()
                .split_once("{").unwrap();
            let item = WorkflowItem::from_str(item);

            (name, item)
        }).collect())
    }
}

impl<'a> Deref for Workflow<'a> {
    type Target = HashMap<&'a str, WorkflowItem<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}