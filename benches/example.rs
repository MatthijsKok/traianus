use traianus::{parse_roman_numeral, NumeralError};

fn main() {
    divan::main();
}

#[divan::bench(args = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"])]
fn parse_roman_numeral_small(input: &str) -> Result<u16, NumeralError<'_>> {
    parse_roman_numeral(input)
}
