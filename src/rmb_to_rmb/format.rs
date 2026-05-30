pub fn format_amount(integer: u128, fraction: u8) -> String {
  let mut out = format_integer(integer);
  push_code(&mut out, 0x5143);
  out.push_str(&format_fraction(fraction, integer > 0));
  out
}

fn format_integer(integer: u128) -> String {
  if integer == 0 {
    return code_string(&[0x96f6]);
  }

  let mut groups = Vec::new();
  let mut rest = integer;
  while rest > 0 {
    groups.push((rest % 10_000) as u16);
    rest /= 10_000;
  }

  let mut out = String::new();
  let mut zero_gap = false;

  for index in (0..groups.len()).rev() {
    let group = groups[index];
    if group == 0 {
      zero_gap = !out.is_empty();
      continue;
    }

    if !out.is_empty() && (zero_gap || group < 1000) {
      push_digit(&mut out, 0);
    }

    out.push_str(&format_group(group));
    push_group_unit(&mut out, index);
    zero_gap = false;
  }

  out
}

fn format_group(group: u16) -> String {
  let digits = [group / 1000 % 10, group / 100 % 10, group / 10 % 10, group % 10];
  let mut out = String::new();
  let mut started = false;
  let mut zero_pending = false;

  for (index, digit) in digits.iter().enumerate() {
    if *digit == 0 {
      zero_pending |= started;
      continue;
    }

    if zero_pending {
      push_digit(&mut out, 0);
      zero_pending = false;
    }

    push_digit(&mut out, *digit as usize);
    push_digit_unit(&mut out, index);
    started = true;
  }

  out
}

fn format_fraction(fraction: u8, integer_non_zero: bool) -> String {
  let jiao = (fraction / 10) as usize;
  let fen = (fraction % 10) as usize;
  let mut out = String::new();

  match (jiao, fen) {
    (0, 0) => push_code(&mut out, 0x6574),
    (0, fen) if integer_non_zero => {
      push_digit(&mut out, 0);
      push_digit(&mut out, fen);
      push_code(&mut out, 0x5206);
    }
    (0, fen) => {
      push_digit(&mut out, fen);
      push_code(&mut out, 0x5206);
    }
    (jiao, 0) => {
      push_digit(&mut out, jiao);
      push_code(&mut out, 0x89d2);
    }
    (jiao, fen) => {
      push_digit(&mut out, jiao);
      push_code(&mut out, 0x89d2);
      push_digit(&mut out, fen);
      push_code(&mut out, 0x5206);
    }
  }

  out
}

fn push_digit(out: &mut String, digit: usize) {
  let code = match digit {
    0 => 0x96f6,
    1 => 0x58f9,
    2 => 0x8d30,
    3 => 0x53c1,
    4 => 0x8086,
    5 => 0x4f0d,
    6 => 0x9646,
    7 => 0x67d2,
    8 => 0x634c,
    9 => 0x7396,
    _ => unreachable!(),
  };
  push_code(out, code);
}

fn push_digit_unit(out: &mut String, index: usize) {
  let code = match index {
    0 => Some(0x4edf),
    1 => Some(0x4f70),
    2 => Some(0x62fe),
    3 => None,
    _ => unreachable!(),
  };
  if let Some(code) = code {
    push_code(out, code);
  }
}

fn push_group_unit(out: &mut String, index: usize) {
  let code = match index {
    0 => None,
    1 => Some(0x4e07),
    2 => Some(0x4ebf),
    3 => Some(0x5146),
    _ => unreachable!(),
  };
  if let Some(code) = code {
    push_code(out, code);
  }
}

fn code_string(codes: &[u32]) -> String {
  let mut out = String::new();
  for code in codes {
    push_code(&mut out, *code);
  }
  out
}

fn push_code(out: &mut String, code: u32) {
  out.push(char::from_u32(code).expect("valid CJK codepoint"));
}
