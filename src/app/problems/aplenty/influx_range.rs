// Inclusive range
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InfluxPropertyRange {
    pub from: usize,
    pub to: usize
}

impl InfluxPropertyRange {
    pub fn size(&self) -> usize {
        self.to + 1 - self.from
    }
}

impl Default for InfluxPropertyRange {
    fn default() -> Self {
        Self {
            from: 1,
            to: 4000
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct InfluxRange {
    pub xtreme: InfluxPropertyRange,
    pub musical: InfluxPropertyRange,
    pub aerodynamic: InfluxPropertyRange,
    pub shiny: InfluxPropertyRange,
}

impl InfluxRange {
    pub fn size(&self) -> usize {
        self.xtreme.size() * self.musical.size() * self.aerodynamic.size() * self.shiny.size()
    }

    pub fn with_xtreme_from(&self, from: usize) -> InfluxRange {
        let mut result = *self;
        result.xtreme.from = from;
        result
    }

    pub fn with_xtreme_to(&self, to: usize) -> InfluxRange {
        let mut result = *self;
        result.xtreme.to = to;
        result
    }

    pub fn with_musical_from(&self, from: usize) -> InfluxRange {
        let mut result = *self;
        result.musical.from = from;
        result
    }

    pub fn with_musical_to(&self, to: usize) -> InfluxRange {
        let mut result = *self;
        result.musical.to = to;
        result
    }

    pub fn with_aerodynamic_from(&self, from: usize) -> InfluxRange {
        let mut result = *self;
        result.aerodynamic.from = from;
        result
    }

    pub fn with_aerodynamic_to(&self, to: usize) -> InfluxRange {
        let mut result = *self;
        result.aerodynamic.to = to;
        result
    }

    pub fn with_shiny_from(&self, from: usize) -> InfluxRange {
        let mut result = *self;
        result.shiny.from = from;
        result
    }

    pub fn with_shiny_to(&self, to: usize) -> InfluxRange {
        let mut result = *self;
        result.shiny.to = to;
        result
    }
}
