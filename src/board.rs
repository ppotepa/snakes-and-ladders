use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Board {
    size: u32,
    transitions: HashMap<u32, u32>,
}

impl Board {
    pub fn standard() -> Self {
        let transitions = HashMap::from([
            (3, 22),
            (5, 14),
            (9, 31),
            (20, 38),
            (28, 84),
            (47, 26),
            (51, 67),
            (62, 18),
            (72, 91),
            (87, 24),
            (99, 80),
        ]);

        Self {
            size: 100,
            transitions,
        }
    }

    pub fn with_transitions(size: u32, transitions: &[(u32, u32)]) -> Self {
        let transitions = transitions.iter().copied().collect();
        Self { size, transitions }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn transition_at(&self, position: u32) -> Option<u32> {
        self.transitions.get(&position).copied()
    }
}
