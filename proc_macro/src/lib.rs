mod comp;

/// Python list comprehension implementation
///
/// | 'mapping' for 'expr' in 'sequence' (if 'condition')* |
///
/// ```
/// let new_list = comp![x + 1 for x in [1,2,3,4]];
/// assert_eq!(new_list, [2,3,4,5]);
/// ```
#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // parse input
    let c = syn::parse_macro_input!(input as comp::Comprehension);
    // redner output
    quote::quote! { #c }.into()
}
