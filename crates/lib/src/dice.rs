use crate::random_range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dice {
    max: usize,
    count: usize,
    modifier: isize,
}

pub enum DiceError {
    InvalidText,
    InvalidModifierText,
    InvalidCountText,
    InvalidSidesText,
}

impl Dice {
    pub fn parse_text(text: &str) -> Result<Self, DiceError> {
        let text = text.to_lowercase();

        let parts: Vec<&str> = text.split('m').collect();

        let modifier = match parts.len() {
            2 => parts[1]
                .parse()
                .map_err(|_| DiceError::InvalidModifierText)?,
            1 => 0,
            _ => return Err(DiceError::InvalidText),
        };

        let dice_parts: Vec<&str> = parts[0].split('d').collect();

        if dice_parts.len() != 2 {
            return Err(DiceError::InvalidText);
        }

        let count = if !dice_parts[0].is_empty() {
            dice_parts[0]
                .parse()
                .map_err(|_| DiceError::InvalidCountText)?
        } else {
            1
        };

        let max: usize = dice_parts[1]
            .parse()
            .map_err(|_| DiceError::InvalidSidesText)?;

        Ok(Self {
            max,
            count,
            modifier,
        })
    }

    pub fn new_once(max: usize) -> Self {
        Self::new(max, 1, 0)
    }

    pub fn new(max: usize, count: usize, modifier: isize) -> Self {
        Self {
            max,
            count,
            modifier,
        }
    }

    pub fn throw(&self) -> isize {
        let mut result: isize = self.modifier;

        for _ in 0..self.count {
            result += random_range(1..=self.max) as isize;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let dice = Dice::new(6, 2, 3);
        assert_eq!(dice.max, 6);
        assert_eq!(dice.count, 2);
        assert_eq!(dice.modifier, 3);
    }

    #[test]
    fn test_new_once() {
        let dice = Dice::new_once(6);
        assert_eq!(dice.max, 6);
        assert_eq!(dice.count, 1);
        assert_eq!(dice.modifier, 0);
    }
}
