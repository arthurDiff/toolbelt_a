#[cfg(feature = "prc_mcr")]
extern crate proc_macro;

#[cfg(feature = "prc_mcr")]
#[cfg_attr(docsrs, doc(cfg(feature = "prc_mcr")))]
pub use proc_macro::comp;
