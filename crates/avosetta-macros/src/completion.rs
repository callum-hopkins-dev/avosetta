use std::cell::RefCell;

use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};

use crate::ast::CrateIdent;

thread_local! {
    static COMPLETIONS: RefCell<Vec<Completion>> = const { RefCell::new(Vec::new()) };
}

#[inline]
pub fn push(completion: Completion) {
    COMPLETIONS.with_borrow_mut(|x| x.push(completion));
}

#[inline]
pub fn push_element(ident: Ident) {
    push(Completion::Element(ident))
}

#[inline]
pub fn push_attr(ident: Ident) {
    push(Completion::Attr(ident))
}

#[inline]
pub fn clear() {
    COMPLETIONS.with_borrow_mut(|x| x.clear());
}

pub fn to_token_stream(crate_ident: &CrateIdent) -> TokenStream {
    COMPLETIONS.with_borrow(|completions| {
        let mut tokens = TokenStream::new();

        for completion in completions {
            match completion {
                Completion::Element(ident) => quote! {
                    #[cfg(any())]
                    use #crate_ident::__completion::elements::#ident as _;
                },

                Completion::Attr(ident) => quote! {
                    #[cfg(any())]
                    use #crate_ident::__completion::attrs::#ident as _;
                },
            }
            .to_tokens(&mut tokens);
        }

        tokens
    })
}

#[derive(Debug, Clone)]
pub enum Completion {
    Element(Ident),
    Attr(Ident),
}
