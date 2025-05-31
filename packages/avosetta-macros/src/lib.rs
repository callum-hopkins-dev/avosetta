use crate::{ast::Group, generate::Generate};

mod ast;
mod generate;
mod parse;

#[proc_macro]
#[inline]
pub fn html(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(item as Group)
        .to_token_stream()
        .into()
}
