use proc_macro2::Span;
use syn::{
    Expr, Ident, Item, LitStr, Pat, Stmt, Token, braced, bracketed,
    parse::Parse,
    punctuated::Punctuated,
    token::{Brace, Bracket},
};

use crate::ast::*;

impl Parse for Input {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            crate_ident: input.parse()?,
            _comma: input.parse()?,
            tokens: input.parse()?,
        })
    }
}

impl Parse for CrateIdent {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![crate]) {
            Ok(Self::Crate(input.parse()?))
        } else {
            Ok(Self::Ident(input.parse()?))
        }
    }
}

impl Parse for Group {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse()?);
        }

        Ok(Self(nodes.into_boxed_slice()))
    }
}

impl Parse for Node {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) || lookahead.peek(Ident) {
            if let Ok(ident) = input.fork().parse() {
                crate::completion::push_element(ident);
            }

            if input.peek2(Brace)
                || (input.peek2(Bracket) && input.peek3(Brace))
                || input.peek2(Token![;])
                || (input.peek2(Bracket) && input.peek3(Token![;]))
            {
                Ok(Self::Element(input.parse()?))
            } else {
                Ok(Self::Literal(input.parse()?))
            }
        } else if lookahead.peek(Token![@]) {
            Ok(Self::Interp(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for Interp {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _at_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl Parse for InterpValue {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![if]) {
            Ok(Self::If(input.parse()?))
        } else if input.peek(Token![match]) {
            Ok(Self::Match(input.parse()?))
        } else if input.peek(Token![for]) {
            Ok(Self::For(input.parse()?))
        } else if input.peek(Token![let]) {
            Ok(Self::Stmt(input.parse()?))
        } else {
            if input.fork().parse::<Item>().is_ok() {
                Ok(Self::Stmt(Stmt::Item(input.parse()?)))
            } else {
                Ok(Self::Expr(input.parse()?))
            }
        }
    }
}

impl Parse for InterpIf {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let then_branch;

        Ok(Self {
            _if_token: input.parse()?,
            cond: input.call(Expr::parse_without_eager_brace)?,
            _brace: braced!(then_branch in input),
            then_branch: then_branch.parse()?,
            else_if: {
                let mut v = Vec::new();

                while input.peek(Token![else]) && input.peek2(Token![if]) {
                    v.push(input.parse()?);
                }

                v.into_boxed_slice()
            },
            else_branch: if input.peek(Token![else]) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl Parse for InterpElseIf {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let group;

        Ok(Self {
            _else_token: input.parse()?,
            _if_token: input.parse()?,
            cond: input.call(Expr::parse_without_eager_brace)?,
            _brace: braced!(group in input),
            group: group.parse()?,
        })
    }
}

impl Parse for InterpElse {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let group;

        Ok(Self {
            _else_token: input.parse()?,
            _brace: braced!(group in input),
            group: group.parse()?,
        })
    }
}

impl Parse for InterpFor {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let body;

        Ok(Self {
            _for_token: input.parse()?,
            pat: input.call(Pat::parse_multi_with_leading_vert)?,
            _in_token: input.parse()?,
            expr: input.call(Expr::parse_without_eager_brace)?,
            _brace: braced!(body in input),
            body: body.parse()?,
        })
    }
}

impl Parse for InterpMatch {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let arms;

        Ok(Self {
            _match_token: input.parse()?,
            expr: input.call(Expr::parse_without_eager_brace)?,
            _brace_token: braced!(arms in input),
            arms: {
                let mut v = Vec::new();

                while !arms.is_empty() {
                    v.push(arms.parse()?);
                }

                v.into_boxed_slice()
            },
        })
    }
}

impl Parse for InterpArm {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: input.call(Pat::parse_multi_with_leading_vert)?,
            _fat_arrow_token: input.parse()?,
            expr: input.parse()?,
            _comma_token: input.parse()?,
        })
    }
}

impl Parse for InterpArmExpr {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Brace) {
            Ok(Self::Group(input.parse()?))
        } else if lookahead.peek(LitStr) {
            Ok(Self::Literal(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for InterpArmGroup {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let group;

        Ok(Self {
            _brace: braced!(group in input),
            group: group.parse()?,
        })
    }
}

impl Parse for Element {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(Token![;]) || (input.peek2(Bracket) && input.peek3(Token![;])) {
            Ok(Self::Void(input.parse()?))
        } else {
            Ok(Self::Normal(input.parse()?))
        }
    }
}

impl Parse for Normal {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inner;

        Ok(Self {
            name: input.parse()?,

            attrs: input.parse()?,

            _brace: braced!(inner in input),

            inner: inner.parse::<Group>()?,
        })
    }
}

impl Parse for Void {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            attrs: input.parse()?,
            _semi_token: input.parse()?,
        })
    }
}

impl Parse for Attrs {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Bracket) {
            let inner;

            Ok(Self {
                _bracket: bracketed!(inner in input),
                inner: inner.parse_terminated(Attr::parse, Token![,])?,
            })
        } else {
            Ok(Self {
                _bracket: Bracket(Span::call_site()),
                inner: Punctuated::new(),
            })
        }
    }
}

impl Parse for Attr {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(ident) = input.fork().parse() {
            crate::completion::push_attr(ident);
        }

        Ok(Self {
            name: input.parse()?,
            value: if input.peek(Token![=]) {
                Some(input.parse()?)
            } else {
                None
            },
        })
    }
}

impl Parse for Name {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            Ok(Self::Lit(input.parse()?))
        } else if lookahead.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else if lookahead.peek(Token![as]) {
            let _as: Token![as] = input.parse()?;
            Ok(Self::Lit(LitStr::new("as", _as.span)))
        } else if lookahead.peek(Token![type]) {
            let _type: Token![type] = input.parse()?;
            Ok(Self::Lit(LitStr::new("type", _type.span)))
        } else if lookahead.peek(Token![for]) {
            let _for: Token![for] = input.parse()?;
            Ok(Self::Lit(LitStr::new("for", _for.span)))
        } else if lookahead.peek(Token![loop]) {
            let _loop: Token![loop] = input.parse()?;
            Ok(Self::Lit(LitStr::new("loop", _loop.span)))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for AttrValue {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _eq_token: input.parse()?,
            expr: input.parse()?,
        })
    }
}
