use traianus::parse_roman_numeral;

fn main() {
    divan::main();
}

#[divan::bench(args = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X"])]
fn parse_roman_numeral_small(input: &str) -> Result<u64, traianus::NumeralError<'_>> {
    parse_roman_numeral(input)
}
