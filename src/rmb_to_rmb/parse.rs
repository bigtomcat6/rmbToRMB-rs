use super::RmbError;

pub fn parse_cents(_input: &str) -> Result<i128, RmbError> {
  Err(RmbError::InvalidFormat)
}

pub fn parse_decimal_amount(_input: &str) -> Result<i128, RmbError> {
  Err(RmbError::InvalidFormat)
}
