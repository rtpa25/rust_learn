// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

pub struct Multiplier {
    current: u32,
    increase: u32,
}

impl Multiplier {
    pub fn new() -> Self {
        Multiplier {
            current: 1,
            increase: 1,
        }
    }

    pub fn powerup(&mut self, increase: u32) {
        self.increase = increase;
    }
}

impl Iterator for Multiplier {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current += self.increase;
        Some(current)
    }
}
