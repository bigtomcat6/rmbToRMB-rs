use std::error::Error;
use std::fmt;

const UPPER_RMB: [&str; 10] = ["零", "壹", "贰", "叁", "肆", "伍", "陆", "柒", "捌", "玖"];
const GROUP_UNITS: [&str; 4] = ["", "万", "亿", "兆"];
const DIGIT_UNITS: [&str; 4] = ["仟", "佰", "拾", ""];

/// Maximum supported integer part: 9999兆9999亿9999万9999.
pub const MAX_INTEGER: i128 = 9_999_999_999_999_999;

/// Maximum supported amount in cents.
pub const MAX_CENTS: i128 = MAX_INTEGER * 100 + 99;

/// Errors returned by the RMB uppercase conversion APIs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RmbError {
  /// Negative values are not supported by this formatter.
  NegativeAmount,
  /// Floating-point adapters must receive finite numbers only.
  NonFiniteAmount,
  /// The supplied string is not a valid decimal amount.
  InvalidFormat,
  /// String amounts are not rounded and may contain at most two decimal places.
  TooManyDecimalPlaces,
  /// The amount exceeds the supported unit range.
  TooLarge,
}

impl fmt::Display for RmbError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::NegativeAmount => write!(f, "amount cannot be negative"),
      Self::NonFiniteAmount => write!(f, "amount must be a finite number"),
      Self::InvalidFormat => write!(f, "amount format is invalid"),
      Self::TooManyDecimalPlaces => write!(f, "amount cannot have more than two decimal places"),
      Self::TooLarge => write!(f, "amount is too large"),
    }
  }
}

impl Error for RmbError {}

/// Converts an amount represented in cents into uppercase RMB text.
///
/// This is the preferred API for financial code because it avoids floating-point
/// rounding issues.
///
/// # Examples
///
/// ```
/// use rmb_upper::to_rmb_upper_from_cents;
///
/// assert_eq!(to_rmb_upper_from_cents(12345).unwrap(), "壹佰贰拾叁元肆角伍分");
/// ```
pub fn to_rmb_upper_from_cents(cents: i128) -> Result<String, RmbError> {
  if cents < 0 {
    return Err(RmbError::NegativeAmount);
  }

  if cents > MAX_CENTS {
    return Err(RmbError::TooLarge);
  }

  let integer = cents / 100;
  let fraction = cents % 100;
  let jiao = fraction / 10;
  let fen = fraction % 10;

  let mut out = String::new();
  out.push_str(&format_integer(integer as u128));
  out.push('元');
  out.push_str(&format_fraction(jiao as usize, fen as usize, integer > 0));

  Ok(out)
}

/// Parses an integer cents string and converts it into uppercase RMB text.
///
/// This is useful for bindings and platforms where a JavaScript number may be
/// too imprecise for large financial integers.
///
/// # Examples
///
/// ```
/// use rmb_upper::to_rmb_upper_from_cents_str;
///
/// assert_eq!(to_rmb_upper_from_cents_str("12345").unwrap(), "壹佰贰拾叁元肆角伍分");
/// ```
pub fn to_rmb_upper_from_cents_str(cents: &str) -> Result<String, RmbError> {
  let cents = cents.trim();

  if cents.is_empty() {
    return Err(RmbError::InvalidFormat);
  }

  if cents.starts_with('-') {
    return Err(RmbError::NegativeAmount);
  }

  let cents = cents.strip_prefix('+').unwrap_or(cents);
  if cents.is_empty() || !cents.bytes().all(|byte| byte.is_ascii_digit()) {
    return Err(RmbError::InvalidFormat);
  }

  let mut value = 0_i128;
  for byte in cents.bytes() {
    value = value
      .checked_mul(10)
      .and_then(|current| current.checked_add((byte - b'0') as i128))
      .ok_or(RmbError::TooLarge)?;

    if value > MAX_CENTS {
      return Err(RmbError::TooLarge);
    }
  }

  to_rmb_upper_from_cents(value)
}

/// Parses a decimal amount string and converts it into uppercase RMB text.
///
/// This function does not round. Inputs with more than two decimal places are
/// rejected. Leading and trailing whitespace are ignored.
///
/// # Examples
///
/// ```
/// use rmb_upper::to_rmb_upper_from_str;
///
/// assert_eq!(to_rmb_upper_from_str("123.45").unwrap(), "壹佰贰拾叁元肆角伍分");
/// ```
pub fn to_rmb_upper_from_str(amount: &str) -> Result<String, RmbError> {
  let amount = amount.trim();

  if amount.is_empty() {
    return Err(RmbError::InvalidFormat);
  }

  if amount.starts_with('-') {
    return Err(RmbError::NegativeAmount);
  }

  if let Some(stripped) = amount.strip_prefix('+') {
    return parse_positive_decimal(stripped);
  }

  parse_positive_decimal(amount)
}

/// Converts an `f64` amount to uppercase RMB text by rounding to the nearest cent.
///
/// Prefer [`to_rmb_upper_from_cents`] or [`to_rmb_upper_from_str`] for deterministic
/// financial code. This helper mainly exists for JavaScript/N-API compatibility.
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

fn parse_positive_decimal(amount: &str) -> Result<String, RmbError> {
  if amount.is_empty() {
    return Err(RmbError::InvalidFormat);
  }

  let mut parts = amount.split('.');
  let integer_part = parts.next().ok_or(RmbError::InvalidFormat)?;
  let fraction_part = parts.next();

  if parts.next().is_some() {
    return Err(RmbError::InvalidFormat);
  }

  if integer_part.is_empty() && fraction_part.unwrap_or_default().is_empty() {
    return Err(RmbError::InvalidFormat);
  }

  let integer = parse_integer_part(integer_part)?;
  let fraction = parse_fraction_part(fraction_part)?;
  let cents = integer
    .checked_mul(100)
    .and_then(|value| value.checked_add(fraction))
    .ok_or(RmbError::TooLarge)?;

  to_rmb_upper_from_cents(cents)
}

fn parse_integer_part(part: &str) -> Result<i128, RmbError> {
  if part.is_empty() {
    return Ok(0);
  }

  if !part.bytes().all(|byte| byte.is_ascii_digit()) {
    return Err(RmbError::InvalidFormat);
  }

  let mut value = 0_i128;
  for byte in part.bytes() {
    value = value
      .checked_mul(10)
      .and_then(|current| current.checked_add((byte - b'0') as i128))
      .ok_or(RmbError::TooLarge)?;

    if value > MAX_INTEGER {
      return Err(RmbError::TooLarge);
    }
  }

  Ok(value)
}

fn parse_fraction_part(part: Option<&str>) -> Result<i128, RmbError> {
  let Some(part) = part else {
    return Ok(0);
  };

  if part.len() > 2 {
    return Err(RmbError::TooManyDecimalPlaces);
  }

  if !part.bytes().all(|byte| byte.is_ascii_digit()) {
    return Err(RmbError::InvalidFormat);
  }

  match part.len() {
    0 => Ok(0),
    1 => Ok(((part.as_bytes()[0] - b'0') as i128) * 10),
    2 => Ok(((part.as_bytes()[0] - b'0') as i128) * 10 + (part.as_bytes()[1] - b'0') as i128),
    _ => unreachable!(),
  }
}

fn format_integer(integer: u128) -> String {
  if integer == 0 {
    return String::from("零");
  }

  let mut groups = Vec::new();
  let mut rest = integer;

  while rest > 0 {
    groups.push((rest % 10_000) as u16);
    rest /= 10_000;
  }

  debug_assert!(groups.len() <= GROUP_UNITS.len());

  let mut out = String::new();
  let mut zero_between_groups = false;

  for index in (0..groups.len()).rev() {
    let group = groups[index];

    if group == 0 {
      if !out.is_empty() {
        zero_between_groups = true;
      }
      continue;
    }

    if !out.is_empty() && (zero_between_groups || group < 1000) {
      out.push('零');
    }

    out.push_str(&format_group(group));
    out.push_str(GROUP_UNITS[index]);
    zero_between_groups = false;
  }

  out
}

fn format_group(group: u16) -> String {
  debug_assert!((1..10_000).contains(&group));

  let digits = [
    (group / 1000) % 10,
    (group / 100) % 10,
    (group / 10) % 10,
    group % 10,
  ];

  let mut out = String::new();
  let mut started = false;
  let mut zero_pending = false;

  for (index, digit) in digits.iter().enumerate() {
    if *digit == 0 {
      if started {
        zero_pending = true;
      }
      continue;
    }

    if zero_pending {
      out.push('零');
      zero_pending = false;
    }

    out.push_str(UPPER_RMB[*digit as usize]);
    out.push_str(DIGIT_UNITS[index]);
    started = true;
  }

  out
}

fn format_fraction(jiao: usize, fen: usize, integer_non_zero: bool) -> String {
  match (jiao, fen) {
    (0, 0) => String::from("整"),
    (0, fen) if integer_non_zero => format!("零{}分", UPPER_RMB[fen]),
    (0, fen) => format!("{}分", UPPER_RMB[fen]),
    (jiao, 0) => format!("{}角", UPPER_RMB[jiao]),
    (jiao, fen) => format!("{}角{}分", UPPER_RMB[jiao], UPPER_RMB[fen]),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn formats_zero_and_plain_integers() {
    assert_eq!(to_rmb_upper_from_cents(0).unwrap(), "零元整");
    assert_eq!(to_rmb_upper_from_cents(100).unwrap(), "壹元整");
    assert_eq!(to_rmb_upper_from_cents(1_000).unwrap(), "壹拾元整");
    assert_eq!(to_rmb_upper_from_cents(10_100).unwrap(), "壹佰零壹元整");
    assert_eq!(to_rmb_upper_from_cents(100_100).unwrap(), "壹仟零壹元整");
    assert_eq!(to_rmb_upper_from_cents(101_000).unwrap(), "壹仟零壹拾元整");
  }

  #[test]
  fn formats_decimal_parts() {
    assert_eq!(to_rmb_upper_from_cents(1).unwrap(), "零元壹分");
    assert_eq!(to_rmb_upper_from_cents(10).unwrap(), "零元壹角");
    assert_eq!(to_rmb_upper_from_cents(11).unwrap(), "零元壹角壹分");
    assert_eq!(to_rmb_upper_from_cents(101).unwrap(), "壹元零壹分");
    assert_eq!(to_rmb_upper_from_cents(110).unwrap(), "壹元壹角");
    assert_eq!(to_rmb_upper_from_cents(111).unwrap(), "壹元壹角壹分");
    assert_eq!(to_rmb_upper_from_cents(12_345).unwrap(), "壹佰贰拾叁元肆角伍分");
  }

  #[test]
  fn formats_group_boundaries_and_internal_zeroes() {
    assert_eq!(to_rmb_upper_from_cents(1_000_000).unwrap(), "壹万元整");
    assert_eq!(to_rmb_upper_from_cents(1_000_100).unwrap(), "壹万零壹元整");
    assert_eq!(to_rmb_upper_from_cents(1_001_000).unwrap(), "壹万零壹拾元整");
    assert_eq!(to_rmb_upper_from_cents(1_010_000).unwrap(), "壹万零壹佰元整");
    assert_eq!(to_rmb_upper_from_cents(1_100_000).unwrap(), "壹万壹仟元整");
    assert_eq!(to_rmb_upper_from_cents(10_000_000_100).unwrap(), "壹亿零壹元整");
    assert_eq!(to_rmb_upper_from_cents(10_001_000_100).unwrap(), "壹亿零壹万零壹元整");
  }

  #[test]
  fn supports_maximum_amount() {
    assert_eq!(
      to_rmb_upper_from_cents(MAX_CENTS).unwrap(),
      "玖仟玖佰玖拾玖兆玖仟玖佰玖拾玖亿玖仟玖佰玖拾玖万玖仟玖佰玖拾玖元玖角玖分"
    );
  }

  #[test]
  fn parses_decimal_strings_without_rounding() {
    assert_eq!(to_rmb_upper_from_str("001.20").unwrap(), "壹元贰角");
    assert_eq!(to_rmb_upper_from_str("1").unwrap(), "壹元整");
    assert_eq!(to_rmb_upper_from_str("1.").unwrap(), "壹元整");
    assert_eq!(to_rmb_upper_from_str(".05").unwrap(), "零元伍分");
    assert_eq!(to_rmb_upper_from_str(" +12.30 ").unwrap(), "壹拾贰元叁角");
  }

  #[test]
  fn parses_cents_strings() {
    assert_eq!(to_rmb_upper_from_cents_str("12345").unwrap(), "壹佰贰拾叁元肆角伍分");
    assert_eq!(to_rmb_upper_from_cents_str(" +001 ").unwrap(), "零元壹分");
    assert_eq!(to_rmb_upper_from_cents_str("0").unwrap(), "零元整");
  }

  #[test]
  fn rejects_invalid_amounts() {
    assert_eq!(to_rmb_upper_from_cents(-1), Err(RmbError::NegativeAmount));
    assert_eq!(to_rmb_upper_from_cents(MAX_CENTS + 1), Err(RmbError::TooLarge));
    assert_eq!(to_rmb_upper_from_cents_str("-1"), Err(RmbError::NegativeAmount));
    assert_eq!(to_rmb_upper_from_cents_str("1.00"), Err(RmbError::InvalidFormat));
    assert_eq!(to_rmb_upper_from_cents_str(""), Err(RmbError::InvalidFormat));
    assert_eq!(to_rmb_upper_from_str("-1.00"), Err(RmbError::NegativeAmount));
    assert_eq!(to_rmb_upper_from_str("1.234"), Err(RmbError::TooManyDecimalPlaces));
    assert_eq!(to_rmb_upper_from_str("1.2a"), Err(RmbError::InvalidFormat));
    assert_eq!(to_rmb_upper_from_str(""), Err(RmbError::InvalidFormat));
    assert_eq!(to_rmb_upper_from_str("."), Err(RmbError::InvalidFormat));
    assert_eq!(to_rmb_upper_from_f64(f64::NAN), Err(RmbError::NonFiniteAmount));
  }
}
