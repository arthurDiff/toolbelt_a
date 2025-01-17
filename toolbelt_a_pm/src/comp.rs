// python list comprehension proc macro
// comp: mapping for_if_clause+
// mapping: expression
// for_if_clause:
//    | 'for' pattern 'in' sequence ('if' expression)*
// pattern: name (, name)*

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Expr, Pat, Token,
};

pub(super) struct Comprehension {
    mapping: Expr,
    for_if_clause: ForIfClause,
    additional_for_ifs: Vec<ForIfClause>,
}

impl Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
            additional_for_ifs: parse_zero_or_more(input),
        })
    }
}

impl ToTokens for Comprehension {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut inv_for_ifs = std::iter::once(&self.for_if_clause)
            .chain(&self.additional_for_ifs)
            .rev();

        let init_output = {
            let mapping = &self.mapping;

            let ForIfClause {
                pattern,
                sequence,
                conditions,
            } = inv_for_ifs.next().expect("Guaranteed One ForIfClause");

            quote! {
                core::iter::IntoIterator::into_iter(#sequence).flat_map(move|#pattern|{
                    (true #(&& (#conditions))*).then(|| #mapping)
                })
            }
        };

        tokens.extend(inv_for_ifs.fold(init_output, |output, fic| {
            let ForIfClause {
                pattern,
                sequence,
                conditions,
            } = fic;
            quote! {
                core::iter::IntoIterator::into_iter(#sequence).filter_map(move|#pattern|{
                    (true #(&& (#conditions))*).then(|| #output)
                }).flatten()
            }
        }))
    }
}

struct ForIfClause {
    pattern: Pat,
    sequence: Expr,
    conditions: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![for]>()?;
        let pattern = Pat::parse_single(input)?;
        _ = input.parse::<Token![in]>()?;
        let sequence = input.parse()?;
        let conditions = parse_zero_or_more(input);
        Ok(Self {
            pattern,
            sequence,
            conditions,
        })
    }
}

struct Condition(Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![if]>()?;
        input.parse().map(Self)
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(val) = input.parse() {
        result.push(val);
    }
    result
}
