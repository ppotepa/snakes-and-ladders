use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Board {
    size: u32,
    transitions: HashMap<u32, u32>,
}

impl Board {
    pub fn standard() -> Self {
        // Classic-ish snakes & ladders layout.
        Self::with_transitions(
            100,
            &[
                (4, 14),
                (9, 31),
                (20, 38),
                (28, 84),
                (40, 59),
                (51, 67),
                (63, 81),
                (17, 7),
                (54, 34),
                (62, 19),
                (64, 60),
                (87, 24),
                (93, 73),
                (95, 75),
                (99, 78),
            ],
        )
    }

    pub fn with_transitions(size: u32, transitions: &[(u32, u32)]) -> Self {
        assert!(size >= 2, "board size must be at least 2");

        let mut mapped = HashMap::with_capacity(transitions.len());
        for &(from, to) in transitions {
            assert!(
                (1..=size).contains(&from) && (1..=size).contains(&to),
                "transition out of board bounds: {from}->{to} (size={size})"
            );
            assert!(
                from != to,
                "transition cannot point to the same tile: {from}"
            );
            mapped.insert(from, to);
        }

        Self {
            size,
            transitions: mapped,
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn transition_at(&self, position: u32) -> Option<u32> {
        self.transitions.get(&position).copied()
    }
}
