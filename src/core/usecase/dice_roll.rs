use crate::core::entity::dice_set::{Dice, DiceSet};

use std::num::ParseIntError;
use std::string::String;

#[derive(Debug)]
pub enum DiceRollError {
    ParseDiceCountError(ParseIntError),
    ParseFaceError(ParseIntError),
    ParseModifierError(ParseIntError),
    InvalidDiceString(String),
}

pub trait DiceRollInterface {
    fn roll(&self, dice_sets: Vec<String>) -> Result<DiceSet, DiceRollError>;
}

pub struct DiceRoll {}

impl DiceRoll {
    pub fn new() -> Self {
        DiceRoll {}
    }

    fn parse_dice_count_and_face(&self, dice_string: &str) -> Result<(u64, u64), DiceRollError> {
        match dice_string.to_lowercase().split('d').collect::<Vec<&str>>()[..] {
            [dice_count, face] => {
                let dice_count = match dice_count.parse::<u64>() {
                    Ok(dice_count) => dice_count,
                    Err(err) => return Err(DiceRollError::ParseDiceCountError(err)),
                };
                let face = match face.parse::<u64>() {
                    Ok(face) => face,
                    Err(err) => return Err(DiceRollError::ParseFaceError(err)),
                };
                Ok((dice_count, face))
            }
            [_] | [] | [_, _, _, ..] => {
                Err(DiceRollError::InvalidDiceString(dice_string.to_string()))
            }
        }
    }

    fn parse_dice_string(&self, dice_string: &str) -> Result<Dice, DiceRollError> {
        match dice_string.split('+').collect::<Vec<&str>>()[..] {
            [dice, modifier] => {
                let (dice_count, face) = self.parse_dice_count_and_face(dice).unwrap();
                let modifier = match modifier.parse::<u64>() {
                    Ok(modifier) => modifier,
                    Err(err) => return Err(DiceRollError::ParseModifierError(err)),
                };
                Ok(Dice {
                    dice_count,
                    face,
                    modifier: Some(modifier),
                })
            }
            [dice] => {
                let (dice_count, face) = self.parse_dice_count_and_face(dice)?;
                Ok(Dice {
                    dice_count,
                    face,
                    modifier: None,
                })
            }
            [] | [_, _, _, ..] => Err(DiceRollError::InvalidDiceString(dice_string.to_string())),
        }
    }

    fn parse(&self, dice_set_string: Vec<String>) -> Result<DiceSet, DiceRollError> {
        // let mut dice_sets = Vec::new();
        let dice_vec = dice_set_string
            .into_iter()
            .map(|dice_string| self.parse_dice_string(&dice_string).unwrap())
            .collect::<Vec<Dice>>();
        Ok(DiceSet::new(&dice_vec))
    }
}

impl DiceRollInterface for DiceRoll {
    fn roll(&self, dice_set_string: Vec<String>) -> Result<DiceSet, DiceRollError> {
        let mut dice_set = self.parse(dice_set_string).unwrap();
        dice_set.roll();
        Ok(dice_set)
    }
}

impl Default for DiceRoll {
    fn default() -> Self {
        DiceRoll::new()
    }
}
