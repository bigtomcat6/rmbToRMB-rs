use napi_derive::napi;

fn to_napi_error(error: crate::RmbError) -> napi::Error {
  napi::Error::from_reason(error.to_string())
}

/// Backward-compatible JavaScript API.
///
/// This adapter accepts a JavaScript number and rounds it to the nearest cent.
/// Prefer `rmbToRmbFromString` for deterministic financial formatting.
#[napi]
pub fn rmb_to_rmb(amount: f64) -> napi::Result<String> {
  crate::to_rmb_upper_from_f64(amount).map_err(to_napi_error)
}

/// Converts an amount represented in cents into uppercase RMB text.
///
/// This accepts a decimal string so callers do not need to rely on JavaScript
/// number precision for large cent values.
#[napi]
pub fn rmb_to_rmb_from_cents(cents: String) -> napi::Result<String> {
  crate::to_rmb_upper_from_cents_str(&cents).map_err(to_napi_error)
}

/// Parses a decimal amount string and converts it into uppercase RMB text.
#[napi]
pub fn rmb_to_rmb_from_string(amount: String) -> napi::Result<String> {
  crate::to_rmb_upper_from_str(&amount).map_err(to_napi_error)
}
