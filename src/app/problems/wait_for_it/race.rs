use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct Race {
    time: usize,
    distance: usize
}

impl Race {
    pub fn get_leniency(&self) -> usize {
        if 4 * self.distance >= self.time * self.time {
            0
        } else {
            let is_odd = self.time % 2;
            let discriminant: f64 = 0.25 * (self.time as f64) * (self.time as f64) - (self.distance as f64);
            let root = discriminant.sqrt() - 1.0 + 0.5 * (is_odd as f64);
            2 * (root.ceil() as usize) + 1 - is_odd
        }
    }
}

#[derive(Debug, Clone)]
pub struct RaceProgram(Vec<Race>);

impl Deref for RaceProgram {
    type Target = Vec<Race>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct RaceIterator<T: Iterator<Item = char>, U: Iterator<Item = char>> {
    time_chars: T,
    distance_chars: U,
}

trait AsRaceIterator<T: Iterator<Item = char>, U: Iterator<Item = char>> {
    fn into_race_iter(self) -> RaceIterator<T, U>;
}

impl<T: Iterator<Item = char>, U: Iterator<Item = char>> Iterator for RaceIterator<T, U> {
    type Item = Race;

    fn next(&mut self) -> Option<Self::Item> {
        let mut time_string = String::new();
        let mut distance_string = String::new();
        while let (Some(time_char), Some(distance_char)) = (self.time_chars.next(), self.distance_chars.next()) {
            if time_char == ' ' && distance_char == ' ' && !time_string.is_empty() && !distance_string.is_empty() {
                let time = time_string.parse::<usize>().unwrap();
                let distance = distance_string.parse::<usize>().unwrap();

                return Some(Race { time, distance });
            }

            if time_char != ' ' {
                time_string.push(time_char);
            }
            if distance_char != ' ' {
                distance_string.push(distance_char);
            }
        }

        if !time_string.is_empty() && !distance_string.is_empty() {
            let time = time_string.parse::<usize>().unwrap();
            let distance = distance_string.parse::<usize>().unwrap();

            Some(Race { time, distance })
        } else {
            None
        }
    }
}

impl<T: Iterator<Item = char>, U: Iterator<Item = char>> AsRaceIterator<T, U> for (T, U) {
    fn into_race_iter(self) -> RaceIterator<T, U> {
        let (time_chars, distance_chars) = self;
        RaceIterator {
            time_chars,
            distance_chars
        }
    }
}

impl From<&str> for RaceProgram {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect::<Vec<&str>>();
        let time_line = lines[0].strip_prefix("Time:     ").unwrap();
        let distance_line = lines[1].strip_prefix("Distance: ").unwrap();
        RaceProgram((time_line.chars(), distance_line.chars()).into_race_iter().collect())
    }
}

impl From<&str> for Race {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect::<Vec<&str>>();
        let time_line = lines[0].strip_prefix("Time:     ").unwrap();
        let distance_line = lines[1].strip_prefix("Distance: ").unwrap();
        let time = time_line.chars().filter(|x| *x != ' ').collect::<String>().parse::<usize>().unwrap();
        let distance = distance_line.chars().filter(|x| *x != ' ').collect::<String>().parse::<usize>().unwrap();

        Race {
            time,
            distance
        }
    }
}
