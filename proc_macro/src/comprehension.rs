// python list comprehension proc macro
// comp: mapping for_if_clause+
// mapping: expression
// for_if_clause:
//    | 'for' pattern 'in' sequence ('if' expression)*
// pattern: name (, name)*

use syn::{
    parse::{Parse, ParseStream},
    Expr, Pat, Token,
};

struct Comprehension {
    mapping: Expr,
    for_if_clause: ForIfClause,
}

impl Parse for Comprehension {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
        })
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

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(val) = input.parse() {
        result.push(val);
    }
    result
}
