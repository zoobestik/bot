use bot_lib::dice::{Dice, DiceError};

#[test]
fn parse_valid_notations() {
    // basic notations without modifier
    assert!(matches!(Dice::parse_text("d6"), Ok(d) if d == Dice::new(6, 1, 0)));
    assert!(matches!(Dice::parse_text("2d10"), Ok(d) if d == Dice::new(10, 2, 0)));

    // with modifiers (positive, zero with plus, and negative)
    assert!(matches!(Dice::parse_text("2d4m3"), Ok(d) if d == Dice::new(4, 2, 3)));
    assert!(matches!(Dice::parse_text("1d6m+3"), Ok(d) if d == Dice::new(6, 1, 3)));
    assert!(matches!(
        Dice::parse_text("2d4m-3"),
        Ok(d) if d == Dice::new(4, 2, -3)
    ));

    // case-insensitive
    assert!(matches!(Dice::parse_text("D8"), Ok(d) if d == Dice::new(8, 1, 0)));

    // edge cases allowed by the current parser (count can be 0)
    assert!(matches!(Dice::parse_text("0d6"), Ok(d) if d == Dice::new(6, 0, 0)));
}

#[test]
fn parse_invalid_notations() {
    // empty and missing parts
    assert!(matches!(Dice::parse_text(""), Err(DiceError::InvalidText)));
    assert!(matches!(
        Dice::parse_text("d"),
        Err(DiceError::InvalidSidesText)
    ));
    assert!(matches!(
        Dice::parse_text("2d"),
        Err(DiceError::InvalidSidesText)
    ));

    // non-numeric count or sides
    assert!(matches!(
        Dice::parse_text("xd6"),
        Err(DiceError::InvalidCountText)
    ));
    assert!(matches!(
        Dice::parse_text("2dx"),
        Err(DiceError::InvalidSidesText)
    ));

    // duplicated separators or malformed structure
    assert!(matches!(
        Dice::parse_text("2dd10"),
        Err(DiceError::InvalidText)
    ));
    assert!(matches!(
        Dice::parse_text("1d6m3m1"),
        Err(DiceError::InvalidText)
    ));

    // misplaced modifier separator or invalid modifier text
    assert!(matches!(
        Dice::parse_text("2dm5"),
        Err(DiceError::InvalidSidesText)
    ));
    assert!(matches!(
        Dice::parse_text("1d6m"),
        Err(DiceError::InvalidModifierText)
    ));
    assert!(matches!(
        Dice::parse_text("1d6m-"),
        Err(DiceError::InvalidModifierText)
    ));
}

#[test]
fn parse_trimming_notations() {
    // current parser does not trim leading/trailing spaces
    assert!(matches!(
        Dice::parse_text(" 2d4m3"),
        Err(DiceError::InvalidCountText)
    ));

    assert!(matches!(
        Dice::parse_text("2d4m3 "),
        Err(DiceError::InvalidModifierText)
    ));
}
