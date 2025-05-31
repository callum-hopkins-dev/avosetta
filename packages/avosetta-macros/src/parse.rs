use syn::{
    Expr, Ident, LitStr, Pat, Token, braced, bracketed,
    parse::Parse,
    token::{Brace, Bracket},
};

use crate::ast::{
    Attr, AttrValue, Attrs, Element, Group, Interp, InterpArm, InterpArmExpr, InterpArmGroup,
    InterpElse, InterpElseIf, InterpFor, InterpIf, InterpMatch, InterpValue, Name, Node, Normal,
    Void,
};

impl Parse for Group {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut v = Vec::new();

        while !input.is_empty() {
            v.push(input.parse()?);
        }

        Ok(Self(v.into_boxed_slice()))
    }
}

impl Parse for Node {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) || lookahead.peek(Ident) {
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
        } else {
            Ok(Self::Expr(input.parse()?))
        }
    }
}

impl Parse for InterpIf {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let then_branch;

        Ok(Self {
            _if_token: input.parse()?,
            cond: input.parse()?,
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
            cond: input.parse()?,
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
            expr: input.parse()?,
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
            attrs: if input.peek(Bracket) {
                Some(input.parse()?)
            } else {
                None
            },
            _brace: braced!(inner in input),
            inner: if inner.is_empty() {
                None
            } else {
                Some(inner.parse()?)
            },
        })
    }
}

impl Parse for Void {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            attrs: if input.peek(Bracket) {
                Some(input.parse()?)
            } else {
                None
            },
            _semi_token: input.parse()?,
        })
    }
}

impl Parse for Attrs {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inner;

        Ok(Self {
            _bracket: bracketed!(inner in input),
            inner: inner.parse_terminated(Attr::parse, Token![,])?,
        })
    }
}

impl Parse for Attr {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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
