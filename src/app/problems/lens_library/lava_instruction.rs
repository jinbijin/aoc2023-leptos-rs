use super::lava_hash::{LavaHash, LavaHasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LavaAction {
    Set(u8),
    Decrement
}

impl LavaHash for LavaAction {
    fn lava_hash(&self, lava_hasher: &mut LavaHasher) {
        match self {
            LavaAction::Set(value) => {
                lava_hasher.add(61);
                lava_hasher.add(*value + 48);
            },
            LavaAction::Decrement => {
                lava_hasher.add(45);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LavaInstruction<'a> {
    pub label: &'a[u8],
    pub action: LavaAction
}

impl<'a> From<&'a str> for LavaInstruction<'a> {
    fn from(value: &'a str) -> Self {
        if value.ends_with('-') {
            let label = value.strip_suffix("-").unwrap().as_bytes();
            Self {
                label,
                action: LavaAction::Decrement
            }
        } else {
            let (label, value) = value.split_once('=').unwrap();
            let label = label.as_bytes();
            let value = value.parse::<u8>().unwrap();

            Self {
                label,
                action: LavaAction::Set(value)
            }
        }
    }
}

impl<'a> LavaHash for LavaInstruction<'a> {
    fn lava_hash(&self, lava_hasher: &mut LavaHasher) {
        lava_hasher.add_slice(self.label);
        self.action.lava_hash(lava_hasher);
    }
}
