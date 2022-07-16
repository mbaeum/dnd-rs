use crate::core::entity::dice_set::{Dice, DiceSet};

use std::num::ParseIntError;
use std::string::String;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dice_count_and_face() {
        let dice_roll = DiceRoll::new();
        assert_eq!(dice_roll.parse_dice_count_and_face("1d6").unwrap(), (1, 6));
        assert_eq!(dice_roll.parse_dice_count_and_face("2D6").unwrap(), (2, 6));
    }

    #[test]
    fn test_parse_dice_count_and_face_fail() {
        let dice_roll = DiceRoll::new();
        let empty_err = "".parse::<u64>().unwrap_err();
        let invalid_err = "invalid".parse::<u64>().unwrap_err();
        assert_eq!(
            dice_roll.parse_dice_count_and_face("1d"),
            Err(DiceRollError::ParseFaceError(empty_err.clone()))
        );
        assert_eq!(
            dice_roll.parse_dice_count_and_face("1dU"),
            Err(DiceRollError::ParseFaceError(invalid_err.clone()))
        );
        assert_eq!(
            dice_roll.parse_dice_count_and_face("d1"),
            Err(DiceRollError::ParseDiceCountError(empty_err.clone()))
        );
        assert_eq!(
            dice_roll.parse_dice_count_and_face("Ud1"),
            Err(DiceRollError::ParseDiceCountError(invalid_err.clone()))
        );
        assert_eq!(
            dice_roll.parse_dice_count_and_face("").unwrap_err(),
            DiceRollError::InvalidDiceString("".to_string())
        );
        assert_eq!(
            dice_roll.parse_dice_count_and_face("mememe").unwrap_err(),
            DiceRollError::InvalidDiceString("mememe".to_string())
        );
    }

    #[test]
    fn test_parse_dice_string() {
        let dice_roll = DiceRoll::new();
        assert_eq!(
            dice_roll.parse_dice_string("1d6+1").unwrap(),
            Dice {
                dice_count: 1,
                face: 6,
                modifier: Some(1),
            }
        );
        assert_eq!(
            dice_roll.parse_dice_string("1d6").unwrap(),
            Dice {
                dice_count: 1,
                face: 6,
                modifier: None,
            }
        );
    }

    #[test]
    fn test_parse_dice_string_fail() {
        let dice_roll = DiceRoll::new();
        let empty_err = "".parse::<u64>().unwrap_err();
        let invalid_err = "invalid".parse::<u64>().unwrap_err();
        assert_eq!(
            dice_roll.parse_dice_string("1d2+U").unwrap_err(),
            DiceRollError::ParseModifierError(invalid_err.clone())
        );
        assert_eq!(
            dice_roll.parse_dice_string("1d2+").unwrap_err(),
            DiceRollError::ParseModifierError(empty_err.clone())
        );
        assert_eq!(
            dice_roll.parse_dice_string("").unwrap_err(),
            DiceRollError::InvalidDiceString("".to_string())
        );
        assert_eq!(
            dice_roll.parse_dice_string("mememe").unwrap_err(),
            DiceRollError::InvalidDiceString("mememe".to_string())
        );
    }
}
