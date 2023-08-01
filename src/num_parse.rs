use std::num::ParseIntError;

#[inline]
pub fn parse_int(input: &str) -> Result<i128, ParseIntError> {
    let input = input.trim();

    // hex
    if input.starts_with("0x") {
        return parse_int_with_base(&input[2..], 16);
    }

    // binary
    if input.starts_with("0b") {
        return parse_int_with_base(&input[2..], 2);
    }

    // octal
    if input.starts_with("0o") {
        return parse_int_with_base(&input[2..], 8);
    }
    
    // decimal
    parse_int_with_base(&input, 10)
}

#[inline]
fn parse_int_with_base(input: &str, base: u32) -> Result<i128, ParseIntError> {
    let input = input.chars().filter(|&c| c != '_').collect::<String>();
    i128::from_str_radix(&input, base)
}
