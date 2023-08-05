use std::num::ParseIntError;
use anyhow::{bail, Result};

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

#[derive(Debug, Clone)]
pub struct Constant {
    pub bitwidth: u32,
    /// Only the integral part, ignoing the minus (-) sign if present
    pub value: u64,
    pub signed: bool,
}

pub fn create_constant(val: &str) -> Result<Constant> {
    macro_rules! permutation_bits {
        ($bits:expr) => {
            u128::pow(2, $bits)
        };
    }

    let mut val = val.trim();

    let mut signed = false;
    if val.starts_with("-") {
        signed = true;
        val = (&val[1..]).trim_start();
    }

    let mut radix = 10;

    if !signed {
        if val.starts_with("0x") || val.starts_with("0X") {
            radix = 16;
            val = &val[2..];
        } else if val.starts_with("0b") || val.starts_with("0B") {
            radix = 2;
            val = &val[2..];
        } else if val.starts_with("0o") || val.starts_with("0O") {
            radix = 8;
            val = &val[2..];
        }
    }

    let val: String = val.chars().filter(|&c| c != '_').collect();

    let integral = u64::from_str_radix(&val, radix)?;
        // .unwrap_or_else(|e| panic!("Failed to parse integer {}: {:?}", val, e));

    let mut bitwidth = match (signed, integral) {
        (false, x) if x <= (u8::MAX as u64) => 8,
        (false, x) if x <= (u32::MAX as u64) => 32,
        (false, x) if x <= (u64::MAX as u64) => 64,

        (true, x) if x <= u64::pow(2, 8 - 1) => 8,
        (true, x) if x <= u64::pow(2, 32 - 1) => 32,
        (true, x) if x <= u64::pow(2, 64 - 1) => 64,

        _ => bail!("Enum value too big (or small) to handle {}", integral),
    };

    Ok(Constant {
        bitwidth,
        value: integral,
        signed,
    })
}
