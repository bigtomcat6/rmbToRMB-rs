use super::{RmbError, MAX_CENTS, MAX_INTEGER};

pub fn parse_cents(input: &str) -> Result<i128, RmbError> {
  parse_unsigned(clean_amount(input)?, MAX_CENTS)
}

pub fn parse_decimal_amount(input: &str) -> Result<i128, RmbError> {
  let input = clean_amount(input)?;
  let (integer, fraction) = split_decimal(input)?;
  let integer = if integer.is_empty() {
    0
  } else {
    parse_unsigned(integer, MAX_INTEGER)?
  };
  let fraction = parse_fraction(fraction)?;
  integer
    .checked_mul(100)
    .and_then(|value| value.checked_add(fraction))
    .ok_or(RmbError::TooLarge)
}

fn clean_amount(input: &str) -> Result<&str, RmbError> {
  let input = input.trim();
  if input.is_empty() {
    return Err(RmbError::InvalidFormat);
  }
  if input.starts_with('-') {
    return Err(RmbError::NegativeAmount);
  }
  let input = input.strip_prefix('+').unwrap_or(input);
  if input.is_empty() {
    return Err(RmbError::InvalidFormat);
  }
  Ok(input)
}

fn split_decimal(input: &str) -> Result<(&str, Option<&str>), RmbError> {
  let mut parts = input.split('.');
  let integer = parts.next().ok_or(RmbError::InvalidFormat)?;
  let fraction = parts.next();
  let has_more_parts = parts.next().is_some();
  let is_empty_decimal = integer.is_empty() && fraction.unwrap_or("").is_empty();

  if has_more_parts || is_empty_decimal {
    return Err(RmbError::InvalidFormat);
  }
  Ok((integer, fraction))
}

fn parse_unsigned(input: &str, max: i128) -> Result<i128, RmbError> {
  if input.is_empty() || !input.chars().all(|ch| ch.is_ascii_digit()) {
    return Err(RmbError::InvalidFormat);
  }
  let mut value = 0_i128;
  for ch in input.chars() {
    let digit = ch.to_digit(10).ok_or(RmbError::InvalidFormat)? as i128;
    value = value
      .checked_mul(10)
      .and_then(|current| current.checked_add(digit))
      .ok_or(RmbError::TooLarge)?;
    if value > max {
      return Err(RmbError::TooLarge);
    }
  }
  Ok(value)
}

fn parse_fraction(input: Option<&str>) -> Result<i128, RmbError> {
  let Some(input) = input else {
    return Ok(0);
  };
  if input.len() > 2 {
    return Err(RmbError::TooManyDecimalPlaces);
  }
  if !input.chars().all(|ch| ch.is_ascii_digit()) {
    return Err(RmbError::InvalidFormat);
  }
  let value = match input.len() {
    0 => 0,
    1 => {
      let digit = input.chars().next().unwrap().to_digit(10).unwrap();
      digit as i128 * 10
    }
    2 => input.parse::<i128>().map_err(|_| RmbError::InvalidFormat)?,
    _ => unreachable!(),
  };
  Ok(value)
}
