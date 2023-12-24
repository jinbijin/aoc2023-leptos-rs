use std::convert::Infallible;
use std::str::FromStr;
use crate::app::math::Gcd;
use super::test_area::FlatArea;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl FromStr for Position {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.splitn(3, ", ").map(|x| x.parse::<i128>().unwrap()).collect();
        Ok(Self {
            x: parts[0],
            y: parts[1],
            z: parts[2]
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Velocity {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl FromStr for Velocity {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.splitn(3, ", ").map(|x| x.parse::<i128>().unwrap()).collect();
        Ok(Self {
            x: parts[0],
            y: parts[1],
            z: parts[2]
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hailstone {
    pub position: Position,
    pub velocity: Velocity
}

impl Hailstone {
    fn projection_intersects_in(&self, rhs: &Hailstone, area: &FlatArea) -> bool {
        let self_path = self.as_projected_path();
        let rhs_path = rhs.as_projected_path();
        match self_path.intersection(&rhs_path) {
            IntersectionResult::None => false,
            IntersectionResult::Intersected(intersection) => {
                intersection.is_in_area(area) && self.is_in_future(intersection) && rhs.is_in_future(intersection)
            },
            IntersectionResult::Coincided(_) => todo!("Not expecting this outcome!")
        }
    }

    fn as_projected_path(&self) -> ProjectedPath {
        let gcd = self.velocity.x.gcd(self.velocity.y).gcd;
        ProjectedPath {
            a: self.velocity.y / gcd,
            b: -self.velocity.x / gcd,
            offset: (self.velocity.y * self.position.x - self.velocity.x * self.position.y) / gcd
        }
    }

    fn is_in_future(&self, intersection: ProjectedIntersection) -> bool {
        if self.velocity.x != 0 {
            let diff = intersection.x - (self.position.x * intersection.denominator);
            diff == 0 || ((self.velocity.x > 0) == (diff > 0))
        } else {
            let diff = intersection.y - (self.position.y * intersection.denominator);
            diff == 0 || ((self.velocity.y > 0) == (diff > 0))
        }
    }
}

impl FromStr for Hailstone {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (position, velocity) = s.split_once(" @ ").unwrap();
        let position = position.parse::<Position>()?;
        let velocity = velocity.parse::<Velocity>()?;

        Ok(Self {
            position,
            velocity
        })
    }
}

#[derive(Debug, Clone)]
pub struct HailstoneCollection(Vec<Hailstone>);

impl HailstoneCollection {
    pub fn intersection_count_inside(&self, area: &FlatArea) -> usize {
        let mut count = 0usize;
        for i in 0..self.0.len() {
            for j in 0..i {
                if self.0[i].projection_intersects_in(&self.0[j], &area) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn print_equations(&self) {
        let vars = vec!['t', 'u', 'v'];
        for i in 0..3 {
            let stone = self.0[i];
            leptos::logging::log!(
                "a * {} + ({}) * {} + x + ({}), ",
                vars[i],
                -stone.velocity.x,
                vars[i],
                -stone.position.x,
            );
            leptos::logging::log!(
                "b * {} + ({}) * {} + y + ({}), ",
                vars[i],
                -stone.velocity.y,
                vars[i],
                -stone.position.y,
            );
            leptos::logging::log!(
                "c * {} + ({}) * {} + z + ({}), ",
                vars[i],
                -stone.velocity.z,
                vars[i],
                -stone.position.z,
            );
        }
    }
}

impl FromStr for HailstoneCollection {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HailstoneCollection(s.lines().map(|line| line.parse::<Hailstone>().unwrap()).collect()))
    }
}

/// Represents the point `(x / denominator, y / denominator)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ProjectedIntersection {
    x: i128,
    y: i128,
    denominator: i128,
}

impl ProjectedIntersection {
    fn is_in_area(&self, area: &FlatArea) -> bool {
        area.x.start * self.denominator <= self.x && self.x <= area.x.end * self.denominator &&
            area.y.start * self.denominator <= self.y && self.y <= area.y.end * self.denominator
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum IntersectionResult<Intersection, Line> {
    None,
    Intersected(Intersection),
    Coincided(Line),
}

/// Represents the line `a * x + b * y = offset`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectedPath {
    pub a: i128,
    pub b: i128,
    pub offset: i128,
}

impl ProjectedPath {
    fn intersection(&self, rhs: &ProjectedPath) -> IntersectionResult<ProjectedIntersection, ProjectedPath> {
        let denominator = self.a * rhs.b - self.b * rhs.a;
        if denominator == 0 {
            if (self.a != 0 && self.a * rhs.offset == self.offset * rhs.a) || (self.b != 0 && self.b * rhs.offset == self.offset * rhs.b) {
                IntersectionResult::Coincided(*self)
            } else {
                leptos::logging::log!("Parallel: {:?} and {:?}", self, rhs);
                IntersectionResult::None
            }
        } else {
            let x = rhs.b * self.offset - self.b * rhs.offset;
            let y = self.a * rhs.offset - rhs.a * self.offset;
            let sign = if denominator > 0 { 1i128 } else { -1i128 };

            // force denominator to be positive
            IntersectionResult::Intersected(ProjectedIntersection {
                x: sign * x,
                y: sign * y,
                denominator: sign * denominator
            })
        }
    }
}
