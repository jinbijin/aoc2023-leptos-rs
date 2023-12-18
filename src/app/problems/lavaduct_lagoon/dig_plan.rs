use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DigDirection {
    Right,
    Down,
    Left,
    Up,
}

impl FromStr for DigDirection {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            _ => Err("Invalid direction")
        }
    }
}


/// """"Color""""
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    direction: DigDirection,
    length: isize,
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = "Invalid hex string";
        let s = s.strip_prefix("(#").ok_or_else(|| error)?;
        let s = s.strip_suffix(")").ok_or_else(|| error)?;

        if s.len() != 6 {
            return Err(error);
        }

        let bytes = s.bytes().map(|x| {
            if x >= b'0' && x <= b'9' {
                Ok((x - b'0') as isize)
            } else if x >= b'a' && x <= b'f' {
                Ok((x + 10 - b'a') as isize)
            } else {
                Err(error)
            }
        }).collect::<Result<Vec<isize>, Self::Err>>()?;

        let direction = match bytes[5] {
            0 => Ok(DigDirection::Right),
            1 => Ok(DigDirection::Down),
            2 => Ok(DigDirection::Left),
            3 => Ok(DigDirection::Up),
            _ => Err(error),
        }?;

        Ok(Self {
            direction,
            length: 16*16*16*16*bytes[0] + 16*16*16*bytes[1] + 16*16*bytes[2] + 16*bytes[3] + bytes[4],
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DigPlanStep {
    pub direction: DigDirection,
    pub length: isize,
    pub color: Color
}

impl FromStr for DigPlanStep {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let error = "Invalid dig plan step";

        let (direction, rest) = s.split_once(' ').ok_or_else(|| error)?;
        let (length, color) = rest.split_once(' ').ok_or_else(|| error)?;

        let direction = direction.parse::<DigDirection>()?;
        let length = length.parse::<isize>().map_err(|x| error)?;
        let color = color.parse::<Color>()?;

        Ok(Self {
            direction,
            length,
            color
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DigPlan(Vec<DigPlanStep>);

impl DigPlan {
    pub fn original_steps(&self) -> &Vec<DigPlanStep> {
        &self.0
    }

    pub fn new_steps(&self) -> Vec<DigPlanStep> {
        self.0.iter()
            .map(|x| DigPlanStep {
                direction: x.color.direction,
                length: x.color.length,
                color: x.color
            })
            .collect()
    }
}

impl Deref for DigPlan {
    type Target = Vec<DigPlanStep>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for DigPlan {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s.lines()
            .map(|line| line.parse::<DigPlanStep>())
            .collect::<Result<Vec<DigPlanStep>, Self::Err>>()?;

        Ok(Self(steps))
    }
}
