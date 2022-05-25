use bracket_random::prelude::{RandomNumberGenerator, DiceType};


pub struct DiceRng {
    rng: RandomNumberGenerator,
}

impl Default for DiceRng {
    fn default() -> Self {
        Self { 
            rng: RandomNumberGenerator::new(),
        }
    }
}

impl DiceRng {
    pub fn roll(&mut self, dice: DiceType) -> i32 {
        self.rng.roll(dice) 
    }

    // pub fn roll_dice(&mut self, count: i32, faces: i32) -> i32 {
    //     self.rng.roll_dice(count, faces)
    // }
}