use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NumeralError {
    #[error("Not implemented")]
    Unimplemented,
    // #[error("Invalid character: {0}")]
    // InvalidCharacter(char),
    // #[error("Invalid numeral: {0}")]
    // InvalidNumeral(String),
    // #[error("Invalid order: {0}")]
    // InvalidOrder(String),
}

pub fn parse_roman_numeral(input: &str) -> Result<u64, NumeralError> {
    if input.is_empty() {
        return Ok(0);
    }
    Err(NumeralError::Unimplemented)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_roman_numeral_empty() {
        let result = parse_roman_numeral("");
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn parse_roman_numeral_unimplemented() {
        let result = parse_roman_numeral("I");
        assert_eq!(result, Err(NumeralError::Unimplemented));
    }
}
