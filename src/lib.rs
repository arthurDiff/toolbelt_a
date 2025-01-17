#[cfg(feature = "comp")]
extern crate toolbelt_a_pm;
#[cfg(any(feature = "proc_macro", feature = "comp"))]
pub mod proc_macro;

#[cfg(any(
    feature = "sync",
    feature = "worker",
    feature = "result_worker",
    feature = "thread_pool"
))]
pub mod sync;

mod result_error;
pub use crate::result_error::*;
