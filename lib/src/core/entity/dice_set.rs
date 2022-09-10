use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Dice {
    pub dice_count: u64,
    pub face: u64,
    pub modifier: Option<u64>,
}

impl Dice {
    pub fn new(dice_count: u64, face: u64, modifier: Option<u64>) -> Self {
        Dice {
            dice_count,
            face,
            modifier,
        }
    }

    pub fn roll(&self) -> u64 {
        let mut result = 0;
        for _ in 0..self.dice_count {
            result += rand::random::<u64>() % self.face + 1;
        }
        result += self.modifier.unwrap_or(0);
        result
    }
}

#[derive(Clone, Debug)]
pub struct DiceSet {
    pub dice_map: HashMap<Dice, u64>,
    pub result: u64,
}

impl DiceSet {
    pub fn new(dice: &[Dice]) -> Self {
        let dice_map = dice
            .iter()
            .map(|&d| (d, 0_u64))
            .collect::<HashMap<Dice, u64>>();
        DiceSet {
            dice_map,
            result: 0,
        }
    }

    pub fn roll(&mut self) {
        let mut result_map = HashMap::<Dice, u64>::new();
        for dice in self.dice_map.keys() {
            let roll = dice.roll();
            result_map.insert(*dice, roll);
        }
        self.dice_map = result_map;
        self.result = self.dice_map.values().sum();
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}d{}", self.dice_count, self.face)?;
        if self.modifier.is_some() {
            write!(f, "+{}", self.modifier.unwrap())?;
        }
        Ok(())
    }
}

impl Display for DiceSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Total: \t\t{}", self.result)?;
        for (dice, result) in &self.dice_map {
            writeln!(f, "|---{}: \t{}", dice, result)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dice_roll() {
        let dice = Dice::new(1, 20, Some(2));
        let result = dice.roll();
        assert!(result > 2 && result <= 22);
    }
    #[test]
    fn test_dice_set_roll() {
        let dice = Dice::new(1, 20, Some(2));
        let mut dice_set = DiceSet::new(&[dice]);
        dice_set.roll();
        assert!(dice_set.result > 2 && dice_set.result <= 22);
    }
}
