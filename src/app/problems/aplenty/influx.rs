use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InfluxItem {
    pub xtreme: usize,
    pub musical: usize,
    pub aerodynamic: usize,
    pub shiny: usize
}

impl InfluxItem {
    fn from_str(s: &str) -> InfluxItem {
        let parts: Vec<_> = s.strip_prefix("{").unwrap()
            .strip_suffix("}").unwrap()
            .splitn(4, ",")
            .collect();

        let xtreme = parts[0].strip_prefix("x=").unwrap()
            .parse::<usize>().unwrap();
        let musical = parts[1].strip_prefix("m=").unwrap()
            .parse::<usize>().unwrap();
        let aerodynamic = parts[2].strip_prefix("a=").unwrap()
            .parse::<usize>().unwrap();
        let shiny = parts[3].strip_prefix("s=").unwrap()
            .parse::<usize>().unwrap();

        InfluxItem {
            xtreme,
            musical,
            aerodynamic,
            shiny
        }
    }

    pub fn value(&self) -> usize {
        self.xtreme + self.musical + self.aerodynamic + self.shiny
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Influx(Vec<InfluxItem>);

impl Influx {
    pub fn from_str(s: &str) -> Influx {
        Influx(s.lines().map(InfluxItem::from_str).collect())
    }
}

impl Deref for Influx {
    type Target = Vec<InfluxItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
