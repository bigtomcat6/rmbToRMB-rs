#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod rmb_to_rmb;
pub use rmb_to_rmb::rmb_to_rmb;


// Code automatically generated when creating RS-NAPI
// You can delete it. 
#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
