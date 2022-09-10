use crate::core::entity::dice_set::{Dice, DiceSet};

use std::num::ParseIntError;
use std::string::String;

#[derive(Debug, PartialEq, Eq)]
pub enum DiceError {
    ParseDiceCountError(ParseIntError),
    ParseFaceError(ParseIntError),
    ParseModifierError(ParseIntError),
    InvalidDiceString(String),
}

pub trait DiceInterface {
    fn roll(&self, dice_sets: Vec<String>) -> Result<DiceSet, DiceError>;
}

pub struct DiceImplementation {}

impl DiceImplementation {
    pub fn new() -> Self {
        DiceImplementation {}
    }

    fn parse_dice_count_and_face(&self, dice_string: &str) -> Result<(u64, u64), DiceError> {
        match dice_string.to_lowercase().split('d').collect::<Vec<&str>>()[..] {
            [dice_count, face] => {
                let dice_count = match dice_count.parse::<u64>() {
                    Ok(dice_count) => dice_count,
                    Err(err) => return Err(DiceError::ParseDiceCountError(err)),
                };
                let face = match face.parse::<u64>() {
                    Ok(face) => face,
                    Err(err) => return Err(DiceError::ParseFaceError(err)),
                };
                Ok((dice_count, face))
            }
            [_] | [] | [_, _, _, ..] => Err(DiceError::InvalidDiceString(dice_string.to_string())),
        }
    }

    fn parse_dice_string(&self, dice_string: &str) -> Result<Dice, DiceError> {
        match dice_string.split('+').collect::<Vec<&str>>()[..] {
            [dice, modifier] => {
                let (dice_count, face) = self.parse_dice_count_and_face(dice).unwrap();
                let modifier = match modifier.parse::<u64>() {
                    Ok(modifier) => modifier,
                    Err(err) => return Err(DiceError::ParseModifierError(err)),
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
            [] | [_, _, _, ..] => Err(DiceError::InvalidDiceString(dice_string.to_string())),
        }
    }

    fn parse(&self, dice_set_string: Vec<String>) -> Result<DiceSet, DiceError> {
        // let mut dice_sets = Vec::new();
        let dice_vec = dice_set_string
            .into_iter()
            .map(|dice_string| self.parse_dice_string(&dice_string).unwrap())
            .collect::<Vec<Dice>>();
        Ok(DiceSet::new(&dice_vec))
    }
}

impl DiceInterface for DiceImplementation {
    fn roll(&self, dice_set_string: Vec<String>) -> Result<DiceSet, DiceError> {
        let mut dice_set = self.parse(dice_set_string).unwrap();
        dice_set.roll();
        Ok(dice_set)
    }
}

impl Default for DiceImplementation {
    fn default() -> Self {
        DiceImplementation::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dice_count_and_face() {
        let dice_roll = DiceImplementation::new();
        assert_eq!(dice_roll.parse_dice_count_and_face("1d6").unwrap(), (1, 6));
        assert_eq!(dice_roll.parse_dice_count_and_face("2D6").unwrap(), (2, 6));
    }

    #[test]
    fn test_parse_dice_count_and_face_fail() {
        let dice_roll = DiceImplementation::new();
        let empty_err = "".parse::<u64>().unwrap_err();
        let invalid_err = "invalid".parse::<u64>().unwrap_err();
        assert_eq!(
            dice_roll.parse_dice_count_and_face("1d"),
            Err(DiceError::ParseFaceError(empty_err.clone()))
        );
        assert_eq!(
            dice_roll.parse_dice_count_and_face("1dU"),
            Err(DiceError::ParseFaceError(invalid_err.clone()))
        );
        assert_eq!(
            dice_roll.parse_dice_count_and_face("d1"),
            Err(DiceError::ParseDiceCountError(empty_err))
        );
        assert_eq!(
            dice_roll.parse_dice_count_and_face("Ud1"),
            Err(DiceError::ParseDiceCountError(invalid_err))
        );
        assert_eq!(
            dice_roll.parse_dice_count_and_face("").unwrap_err(),
            DiceError::InvalidDiceString("".to_string())
        );
        assert_eq!(
            dice_roll.parse_dice_count_and_face("mememe").unwrap_err(),
            DiceError::InvalidDiceString("mememe".to_string())
        );
    }

    #[test]
    fn test_parse_dice_string() {
        let dice_roll = DiceImplementation::new();
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
        let dice_roll = DiceImplementation::new();
        let empty_err = "".parse::<u64>().unwrap_err();
        let invalid_err = "invalid".parse::<u64>().unwrap_err();
        assert_eq!(
            dice_roll.parse_dice_string("1d2+U").unwrap_err(),
            DiceError::ParseModifierError(invalid_err)
        );
        assert_eq!(
            dice_roll.parse_dice_string("1d2+").unwrap_err(),
            DiceError::ParseModifierError(empty_err)
        );
        assert_eq!(
            dice_roll.parse_dice_string("").unwrap_err(),
            DiceError::InvalidDiceString("".to_string())
        );
        assert_eq!(
            dice_roll.parse_dice_string("mememe").unwrap_err(),
            DiceError::InvalidDiceString("mememe".to_string())
        );
    }
}
