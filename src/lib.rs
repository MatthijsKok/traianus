use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NumeralError<'a> {
    #[error("Invalid character: {0}")]
    InvalidCharacter(char),
    #[error("Invalid numeral: {0}")]
    InvalidNumeral(&'a str),
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
pub fn parse_roman_numeral(input: &str) -> Result<u64, NumeralError<'_>> {
    let valid_chars = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
    for c in input.chars() {
        if !valid_chars.contains(&c) {
            return Err(NumeralError::InvalidCharacter(c));
        }
    }

    let mut result: u64 = 0;
    // let mut result: i64 = 0; // Result is i64 to allow for subtraction at the beginning of the numeral.

    let mut chars = input.chars().peekable();

    while let Some(char) = chars.next() {
        match char {
            'I' => {
                if result % 5 >= 3 {
                    return Err(NumeralError::InvalidNumeral(input));
                }
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
                            if result % 5 > 0 {
                                return Err(NumeralError::InvalidNumeral(input));
                            }
                            result += 4;
                            chars.next();
                        }
                        'X' => {
                            if result % 10 > 0 {
                                return Err(NumeralError::InvalidNumeral(input));
                            }
                            result += 9;
                            chars.next();
                        }
                        'L' | 'C' | 'D' | 'M' => {
                            return Err(NumeralError::InvalidNumeral(input));
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
                // TODO did this if block get _completely_ obsoleted by the 'V' in the match block down?
                // if result % 10 >= 5 {
                //     return Err(NumeralError::InvalidNumeral(input));
                // }
                if let Some('V' | 'X' | 'L' | 'C' | 'D' | 'M') = chars.peek() {
                    return Err(NumeralError::InvalidNumeral(input));
                }
                result += 5;
            }
            'X' => {
                if result % 50 >= 30 {
                    return Err(NumeralError::InvalidNumeral(input));
                }
                if let Some(next_char) = chars.peek() {
                    match next_char {
                        'L' => {
                            if result % 50 > 0 {
                                return Err(NumeralError::InvalidNumeral(input));
                            }
                            result += 40;
                            chars.next();
                        }
                        'C' => {
                            if result % 100 > 0 {
                                return Err(NumeralError::InvalidNumeral(input));
                            }
                            result += 90;
                            chars.next();
                        }
                        'D' | 'M' => {
                            return Err(NumeralError::InvalidNumeral(input));
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
                // TODO did this if block get _completely_ obsoleted by the 'L' in the match block down?
                // if result % 100 >= 50 {
                //     return Err(NumeralError::InvalidNumeral(input));
                // }
                if let Some('L' | 'C' | 'D' | 'M') = chars.peek() {
                    return Err(NumeralError::InvalidNumeral(input));
                }
                result += 50;
            }
            'C' => {
                if result % 500 >= 300 {
                    return Err(NumeralError::InvalidNumeral(input));
                }
                if let Some(next_char) = chars.peek() {
                    match next_char {
                        'D' => {
                            if result % 500 > 0 {
                                return Err(NumeralError::InvalidNumeral(input));
                            }
                            result += 400;
                            chars.next();
                        }
                        'M' => {
                            if result % 1000 > 0 {
                                return Err(NumeralError::InvalidNumeral(input));
                            }
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
                // TODO did this if block get _completely_ obsoleted by the 'D' in the match block down?
                // if result % 1000 >= 500 {
                //     return Err(NumeralError::InvalidNumeral(input));
                // }
                if let Some('D' | 'M') = chars.peek() {
                    return Err(NumeralError::InvalidNumeral(input));
                }
                result += 500;
            }
            'M' => {
                if result % 5000 >= 3000 {
                    return Err(NumeralError::InvalidNumeral(input));
                }
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
    fn parse_roman_numeral_small() {
        assert_eq!(parse_roman_numeral(""), Ok(0));
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
    fn parse_roman_numeral_large() {
        assert_eq!(parse_roman_numeral("LXIX"), Ok(69));
        assert_eq!(parse_roman_numeral("MCMLXIX"), Ok(1969));
        assert_eq!(parse_roman_numeral("CMXCIX"), Ok(999));
        assert_eq!(parse_roman_numeral("XXXIX"), Ok(39));
        assert_eq!(parse_roman_numeral("MMMCMXXXIX"), Ok(3939));
        assert_eq!(parse_roman_numeral("MMMCMXCIX"), Ok(3999));
        assert_eq!(parse_roman_numeral("ML"), Ok(1050));
        assert_eq!(parse_roman_numeral("DI"), Ok(501));
        assert_eq!(parse_roman_numeral("CIII"), Ok(103));
        assert_eq!(parse_roman_numeral("LIV"), Ok(54));
    }

    #[test]
    fn parse_roman_numeral_invalid_numeral() {
        let invalid_numerals = vec![
            "IIII", "VV", "XXXX", "LL", "CCCC", "DD", "MMMM", "IC", "IL", "VX", "LC", "DM", "IIX",
            "VVX", "XM", "IIIIX", "IM", "IIV", "VX", "XXC",
        ];
        for numeral in invalid_numerals {
            assert_eq!(
                parse_roman_numeral(numeral),
                Err(NumeralError::InvalidNumeral(numeral))
            );
        }
    }

    #[test]
    fn parse_roman_numeral_invalid_character() {
        assert_eq!(
            parse_roman_numeral("XIZI"),
            Err(NumeralError::InvalidCharacter('Z'))
        );
    }
}
