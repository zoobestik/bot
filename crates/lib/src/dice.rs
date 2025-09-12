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
        Self::new_simple(max, 1)
    }

    pub fn new_simple(max: usize, count: usize) -> Self {
        Self {
            max,
            count,
            modifier: 0,
        }
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
