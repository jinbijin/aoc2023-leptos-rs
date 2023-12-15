use std::num::Wrapping;

pub trait LavaHash {
    fn lava_hash(&self, lava_hasher: &mut LavaHasher);

    fn get_lava_value(&self) -> usize {
        let mut lava_hasher = LavaHasher::new();
        self.lava_hash(&mut lava_hasher);
        lava_hasher.get_value()
    }
}

impl LavaHash for u8 {
    fn lava_hash(&self, lava_hasher: &mut LavaHasher) {
        lava_hasher.add(*self);
    }
}

impl LavaHash for [u8] {
    fn lava_hash(&self, lava_hasher: &mut LavaHasher) {
        lava_hasher.add_slice(self);
    }
}

pub struct LavaHasher {
    value: Wrapping<u8>,
}

impl LavaHasher {
    fn new() -> Self {
        Self {
            value: Wrapping(0)
        }
    }

    fn get_value(self) -> usize {
        self.value.0 as usize
    }

    pub fn add_slice(&mut self, values: &[u8]) {
        for value in values {
            self.add(*value);
        }
    }

    pub fn add(&mut self, value: u8) {
        let value = Wrapping(value);
        self.value += value;
        self.value *= 17;
    }
}
