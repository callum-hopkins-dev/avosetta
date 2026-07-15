use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    ast::{Group, Input},
    generate::{Generate, Stream},
};

mod ast;
mod completion;
mod generate;
mod parse;

#[proc_macro]
pub fn asx(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as Input);

    let mut tokens = TokenStream::new();

    match syn::parse2::<Group>(input.tokens) {
        Ok(group) => {
            tokens.extend(crate::completion::to_token_stream(&input.crate_ident));

            let mut stream = Stream::new(input.crate_ident);
            group.generate(&mut stream);
            tokens.extend(stream.into_token_stream());
        }

        Err(err) => {
            tokens.extend(crate::completion::to_token_stream(&input.crate_ident));
            tokens.extend(err.into_compile_error());
        }
    };

    quote! {{ #tokens }}.into()
}
