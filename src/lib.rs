#[cfg(feature = "proc_macro")]
extern crate proc_macro;

#[cfg(feature = "proc_macro")]
#[cfg_attr(docsrs, doc(cfg(feature = "proc_macro")))]
pub use proc_macro::comp;
