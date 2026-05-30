#![deny(clippy::all)]

mod rmb_to_rmb;

#[cfg(feature = "napi")]
mod napi_bindings;

pub use rmb_to_rmb::{
  to_rmb_upper_from_cents, to_rmb_upper_from_cents_str, to_rmb_upper_from_f64,
  to_rmb_upper_from_str, RmbError, MAX_CENTS, MAX_INTEGER,
};
