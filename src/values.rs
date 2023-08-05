#[derive(Debug, Clone)]
pub struct IntegralType {
    pub bitwidth: u32,
    pub signed: bool,
}

#[derive(Debug, Clone)]
pub struct Constant {
    /// Only the integral part, ignoing the minus (-) sign if present
    pub value: u64,
    pub ty: IntegralType,
}

pub fn make_constant(val: &str) -> Constant {
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

    let integral = u64::from_str_radix(&val, radix)
        .unwrap_or_else(|e| panic!("Failed to parse integer {}: {:?}", val, e));

    let bitwidth = match (signed, integral) {
        (false, x) if x <= (u8::MAX as u64) => 8,
        (false, x) if x <= (u32::MAX as u64) => 32,
        (false, x) if x <= (u64::MAX as u64) => 64,

        (true, x) if x <= (i8::MAX as u64) + 1 => 8,
        (true, x) if x <= (i32::MAX as u64) + 1 => 32,
        (true, x) if x <= (i64::MAX as u64) + 1 => 64,

        _ => panic!("Enum value too big (or small) to handle {}", integral),
    };

    Constant {
        value: integral,
        ty: IntegralType { bitwidth, signed },
    }
}
