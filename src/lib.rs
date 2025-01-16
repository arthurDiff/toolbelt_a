pub mod proc_macro;
mod result_error;
pub mod sync;

pub use crate::result_error::*;

#[cfg(feature = "comp")]
extern crate proc_macro_a;
