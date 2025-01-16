#[cfg(any(feature = "proc_macro", feature = "comp"))]
extern crate proc_macro_a;

#[cfg(any(feature = "proc_macro", feature = "comp"))]
#[cfg_attr(docsrs, doc(any(feature = "proc_macro", feature = "comp")))]
pub use proc_macro_a::comp;
