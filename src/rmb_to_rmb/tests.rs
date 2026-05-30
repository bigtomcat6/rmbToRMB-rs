use super::*;

fn s(codes: &[u32]) -> String {
  codes
    .iter()
    .map(|code| char::from_u32(*code).expect("valid CJK codepoint"))
    .collect()
}

#[test]
fn formats_zero_and_plain_integers() {
  assert_eq!(
    to_rmb_upper_from_cents(0).unwrap(),
    s(&[0x96f6, 0x5143, 0x6574]),
  );
  assert_eq!(
    to_rmb_upper_from_cents(100).unwrap(),
    s(&[0x58f9, 0x5143, 0x6574]),
  );
  assert_eq!(
    to_rmb_upper_from_cents(1_000).unwrap(),
    s(&[0x58f9, 0x62fe, 0x5143, 0x6574]),
  );
}

#[test]
fn formats_decimal_parts() {
  assert_eq!(
    to_rmb_upper_from_cents(1).unwrap(),
    s(&[0x96f6, 0x5143, 0x58f9, 0x5206]),
  );
  assert_eq!(
    to_rmb_upper_from_cents(10).unwrap(),
    s(&[0x96f6, 0x5143, 0x58f9, 0x89d2]),
  );
  assert_eq!(
    to_rmb_upper_from_cents(101).unwrap(),
    s(&[0x58f9, 0x5143, 0x96f6, 0x58f9, 0x5206]),
  );
}

#[test]
fn formats_group_boundaries_and_internal_zeroes() {
  assert_eq!(
    to_rmb_upper_from_cents(1_000_000).unwrap(),
    s(&[0x58f9, 0x4e07, 0x5143, 0x6574]),
  );
  assert_eq!(
    to_rmb_upper_from_cents(1_000_100).unwrap(),
    s(&[0x58f9, 0x4e07, 0x96f6, 0x58f9, 0x5143, 0x6574]),
  );
}

#[test]
fn parses_strings() {
  assert_eq!(
    to_rmb_upper_from_str("001.20").unwrap(),
    s(&[0x58f9, 0x5143, 0x8d30, 0x89d2]),
  );
  assert_eq!(
    to_rmb_upper_from_cents_str("12345").unwrap(),
    to_rmb_upper_from_str("123.45").unwrap(),
  );
}

#[test]
fn rejects_invalid_amounts() {
  assert_eq!(to_rmb_upper_from_cents(-1), Err(RmbError::NegativeAmount));
  assert_eq!(to_rmb_upper_from_cents(MAX_CENTS + 1), Err(RmbError::TooLarge));
  assert_eq!(to_rmb_upper_from_cents_str("-1"), Err(RmbError::NegativeAmount));
  assert_eq!(to_rmb_upper_from_cents_str("1.00"), Err(RmbError::InvalidFormat));
  assert_eq!(to_rmb_upper_from_str("1.234"), Err(RmbError::TooManyDecimalPlaces));
  assert_eq!(to_rmb_upper_from_f64(f64::NAN), Err(RmbError::NonFiniteAmount));
}
