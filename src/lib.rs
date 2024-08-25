use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NumeralError<'a> {
    #[error("Not implemented")]
    Unimplemented,
    #[error("Invalid character: {0}")]
    InvalidCharacter(char),
    #[error("Invalid numeral: {0}")]
    InvalidNumeral(&'a str),
    #[error("Invalid order: {0}")]
    InvalidOrder(&'a str),
}

/// Parse a Roman numeral string into a u64.
///
/// See the Wikipedia page on [Roman numerals](https://en.wikipedia.org/wiki/Roman_numerals) for more information.
///
/// # Examples
///
/// ```rust
/// use traianus::parse_roman_numeral;
///
/// assert_eq!(parse_roman_numeral("MMXXIV"), Ok(2024));
/// ```
pub fn parse_roman_numeral(input: &str) -> Result<u64, NumeralError> {
    let mut result: u64 = 0;
    // let mut result: i64 = 0; // Result is i64 to allow for subtraction at the beginning of the numeral.

    let mut chars = input.chars().peekable();

    while let Some(char) = chars.next() {
        match char {
            'I' => {
                if let Some(next_char) = chars.peek() {
                    // FIXME - This will break if we substract at the beginning of the numeral.
                    // match next_char {
                    //     'V' | 'X' => {
                    //         result -= 1;
                    //     }
                    //     _ => {
                    //         result += 1;
                    //     }
                    // }
                    match next_char {
                        'V' => {
                            result += 4;
                            chars.next();
                        }
                        'X' => {
                            result += 9;
                            chars.next();
                        }
                        _ => {
                            result += 1;
                        }
                    }
                } else {
                    result += 1;
                }
            }
            'V' => {
                result += 5;
            }
            'X' => {
                if let Some(next_char) = chars.peek() {
                    // FIXME - This will break if we substract at the beginning of the numeral.
                    // match next_char {
                    //     'L' | 'C' => {
                    //         result -= 10;
                    //     }
                    //     _ => {
                    //         result += 10;
                    //     }
                    // }

                    match next_char {
                        'L' => {
                            result += 40;
                            chars.next();
                        }
                        'C' => {
                            result += 90;
                            chars.next();
                        }
                        _ => {
                            result += 10;
                        }
                    }
                } else {
                    result += 10;
                }
            }
            'L' => {
                result += 50;
            }
            'C' => {
                // result += 100;
                if let Some(next_char) = chars.peek() {
                    match next_char {
                        'D' => {
                            result += 400;
                            chars.next();
                        }
                        'M' => {
                            result += 900;
                            chars.next();
                        }
                        _ => {
                            result += 100;
                        }
                    }
                } else {
                    result += 100;
                }
            }
            'D' => {
                result += 500;
            }
            'M' => {
                result += 1000;
            }
            _ => {
                return Err(NumeralError::InvalidCharacter(char));
            }
        }
    }

    Ok(result)
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
    fn parse_roman_numeral_one() {
        assert_eq!(parse_roman_numeral("I"), Ok(1));
    }

    #[test]
    fn parse_roman_numeral_small() {
        assert_eq!(parse_roman_numeral("I"), Ok(1));
        assert_eq!(parse_roman_numeral("II"), Ok(2));
        assert_eq!(parse_roman_numeral("III"), Ok(3));
        assert_eq!(parse_roman_numeral("IV"), Ok(4));
        assert_eq!(parse_roman_numeral("V"), Ok(5));
        assert_eq!(parse_roman_numeral("VI"), Ok(6));
        assert_eq!(parse_roman_numeral("VII"), Ok(7));
        assert_eq!(parse_roman_numeral("VIII"), Ok(8));
        assert_eq!(parse_roman_numeral("IX"), Ok(9));
    }

    #[test]
    fn parse_roman_numeral_invalid_character() {
        assert_eq!(
            parse_roman_numeral("XIZI"),
            Err(NumeralError::InvalidCharacter('Z'))
        );
    }

    // #[test]
    // fn parse_roman_numeral_incorrect_repetition() {
    //     assert_eq!(
    //         parse_roman_numeral("IIII"),
    //         Err(NumeralError::InvalidNumeral("IIII".into()))
    //     );
    // }

    // #[test]
    // fn parse_roman_numeral_unimplemented() {
    //     let result = parse_roman_numeral("MMXXIV");
    //     assert_eq!(result, Err(NumeralError::Unimplemented));
    // }
}
