use std::error::Error;
use std::fmt;

mod format;
mod parse;

#[cfg(test)]
mod tests;

pub const MAX_INTEGER: i128 = 9_999_999_999_999_999;
pub const MAX_CENTS: i128 = MAX_INTEGER * 100 + 99;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RmbError {
    NegativeAmount,
    NonFiniteAmount,
    InvalidFormat,
    TooManyDecimalPlaces,
    TooLarge,
}

impl fmt::Display for RmbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeAmount => write!(f, "amount cannot be negative"),
            Self::NonFiniteAmount => write!(f, "amount must be a finite number"),
            Self::InvalidFormat => write!(f, "amount format is invalid"),
            Self::TooManyDecimalPlaces => {
                write!(f, "amount cannot have more than two decimal places")
            }
            Self::TooLarge => write!(f, "amount is too large"),
        }
    }
}

impl Error for RmbError {}

pub fn to_rmb_upper_from_cents(cents: i128) -> Result<String, RmbError> {
    if cents < 0 {
        return Err(RmbError::NegativeAmount);
    }

    if cents > MAX_CENTS {
        return Err(RmbError::TooLarge);
    }

    let integer = (cents / 100) as u128;
    let fraction = (cents % 100) as u8;
    Ok(format::format_amount(integer, fraction))
}

pub fn to_rmb_upper_from_cents_str(cents: &str) -> Result<String, RmbError> {
    parse::parse_cents(cents).and_then(to_rmb_upper_from_cents)
}

pub fn to_rmb_upper_from_str(amount: &str) -> Result<String, RmbError> {
    parse::parse_decimal_amount(amount).and_then(to_rmb_upper_from_cents)
}

pub fn to_rmb_upper_from_f64(amount: f64) -> Result<String, RmbError> {
    if !amount.is_finite() {
        return Err(RmbError::NonFiniteAmount);
    }

    if amount < 0.0 {
        return Err(RmbError::NegativeAmount);
    }

    let cents = (amount * 100.0).round();
    if cents > MAX_CENTS as f64 {
        return Err(RmbError::TooLarge);
    }

    to_rmb_upper_from_cents(cents as i128)
}
