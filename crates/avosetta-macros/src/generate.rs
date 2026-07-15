use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Expr, ExprLit, Lit, LitStr};

use crate::ast::*;

pub struct Stream {
    estimated_len: usize,
    buf: String,
    tokens: TokenStream,

    crate_ident: CrateIdent,
    write_ident: Ident,
}

impl Stream {
    #[inline]
    pub fn new(crate_ident: CrateIdent) -> Self {
        Self {
            estimated_len: 0,
            buf: String::new(),
            tokens: TokenStream::new(),

            crate_ident,
            write_ident: Ident::new("__s", Span::mixed_site()),
        }
    }

    #[inline]
    pub const fn idents(&self) -> (&CrateIdent, &Ident) {
        (&self.crate_ident, &self.write_ident)
    }

    #[inline]
    pub fn push_raw(&mut self, s: &str) {
        self.buf.push_str(s);
        self.estimated_len += s.len();
    }

    pub fn push_escaped(&mut self, s: &str) {
        for ch in s.chars() {
            match ch {
                '&' => self.push_raw("&amp;"),

                '<' => self.push_raw("&lt;"),
                '>' => self.push_raw("&gt;"),

                '\'' => self.push_raw("&apos;"),
                '\"' => self.push_raw("&quot;"),

                ch => {
                    self.buf.push(ch);
                    self.estimated_len += ch.len_utf8();
                }
            }
        }
    }

    #[inline]
    pub fn push_tokens<T>(&mut self, tokens: T)
    where
        T: ToTokens,
    {
        self.flush();
        tokens.to_tokens(&mut self.tokens);
    }

    #[inline]
    pub fn push_write<T>(&mut self, tokens: T)
    where
        T: ToTokens,
    {
        let (crate_ident, write_ident) = self.idents();
        self.push_tokens(quote! { #crate_ident::Html::write(#tokens, #write_ident); });
    }

    pub fn push_scope<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut stream = Self {
            estimated_len: 0,
            buf: String::new(),
            tokens: TokenStream::new(),
            crate_ident: self.crate_ident.clone(),
            write_ident: self.write_ident.clone(),
        };

        (f)(&mut stream);
        stream.flush();

        self.estimated_len += stream.estimated_len;
        let tokens = stream.tokens;
        self.push_tokens(quote! { { #tokens } });
    }

    pub fn flush(&mut self) {
        let (crate_ident, write_ident) = self.idents();

        if !self.buf.is_empty() {
            let literal = proc_macro2::Literal::string(&self.buf);

            quote! { #crate_ident::Html::write(#crate_ident::Raw(#literal), #write_ident); }
                .to_tokens(&mut self.tokens);

            self.buf.clear();
        }
    }

    pub fn into_token_stream(mut self) -> TokenStream {
        self.flush();

        let Self {
            estimated_len,
            tokens,
            crate_ident,
            write_ident,
            ..
        } = self;

        quote! {{
            #[inline]
            const fn coerce<T: #crate_ident::Html>(x: T) -> impl #crate_ident::Html { x }

            struct Html<F>(F);

            impl<F> #crate_ident::Html for Html<F>
            where
                F: ::core::ops::FnOnce(&mut ::std::string::String)
            {
                #[inline]
                fn write(self, s: &mut ::std::string::String) { (self.0)(s) }
            }

            coerce(Html(move |#write_ident: &mut ::std::string::String| {
                ::std::string::String::reserve(#write_ident, #estimated_len);

                #tokens
            }))
        }}
    }
}

pub trait Generate {
    fn generate(&self, stream: &mut Stream);
}

impl Generate for Group {
    fn generate(&self, stream: &mut Stream) {
        let has_stmts = self.0.iter().any(|x| {
            matches!(
                x,
                Node::Interp(Interp {
                    value: InterpValue::Stmt(_),
                    ..
                })
            )
        });

        if has_stmts {
            stream.push_scope(|stream| {
                for node in &self.0 {
                    node.generate(stream);
                }
            });
        } else {
            for node in &self.0 {
                node.generate(stream);
            }
        }
    }
}

impl Generate for Node {
    fn generate(&self, stream: &mut Stream) {
        match self {
            Node::Element(x) => x.generate(stream),
            Node::Interp(x) => x.generate(stream),
            Node::Literal(x) => x.generate(stream),
        }
    }
}

impl Generate for Element {
    fn generate(&self, stream: &mut Stream) {
        match self {
            Element::Normal(x) => x.generate(stream),
            Element::Void(x) => x.generate(stream),
        }
    }
}

impl Generate for Normal {
    fn generate(&self, stream: &mut Stream) {
        stream.push_raw("<");
        self.name.generate(stream);
        self.attrs.generate(stream);
        stream.push_raw(">");

        self.inner.generate(stream);

        stream.push_raw("</");
        self.name.generate(stream);
        stream.push_raw(">");
    }
}

impl Generate for Void {
    fn generate(&self, stream: &mut Stream) {
        stream.push_raw("<");
        self.name.generate(stream);
        self.attrs.generate(stream);
        stream.push_raw(">");
    }
}

impl Generate for Interp {
    fn generate(&self, stream: &mut Stream) {
        match &self.value {
            InterpValue::Match(x) => x.generate(stream),
            InterpValue::If(x) => x.generate(stream),
            InterpValue::For(x) => x.generate(stream),

            InterpValue::Expr(Expr::Lit(ExprLit { lit, .. })) => lit.generate(stream),

            InterpValue::Expr(expr) => {
                stream.push_write(expr);
                stream.estimated_len += 24;
            }

            InterpValue::Stmt(stmt) => stream.push_tokens(stmt),
        }
    }
}

impl Generate for InterpMatch {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        let expr = &self.expr;

        stream.push_tokens(quote! { match #expr });

        stream.push_scope(|stream| {
            for arm in &self.arms {
                arm.generate(stream);
            }
        });
    }
}

impl Generate for InterpArm {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        let pat = &self.pat;

        stream.push_tokens(quote! { #pat => });
        stream.push_scope(|stream| self.expr.generate(stream));
    }
}

impl Generate for InterpArmExpr {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        match &self {
            InterpArmExpr::Group(interp_arm_group) => interp_arm_group.generate(stream),
            InterpArmExpr::Literal(lit_str) => lit_str.generate(stream),
        }
    }
}

impl Generate for InterpArmGroup {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        stream.push_scope(|stream| self.group.generate(stream));
    }
}

impl Generate for InterpFor {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        let Self {
            pat, expr, body, ..
        } = &self;

        stream.push_tokens(quote! { for #pat in #expr });
        stream.push_scope(|stream| body.generate(stream));
    }
}

impl Generate for InterpIf {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        let cond = &self.cond;

        stream.push_tokens(quote! { if #cond });
        stream.push_scope(|stream| self.then_branch.generate(stream));

        for else_if in &self.else_if {
            else_if.generate(stream);
        }

        if let Some(else_branch) = &self.else_branch {
            else_branch.generate(stream);
        }
    }
}

impl Generate for InterpElseIf {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        let cond = &self.cond;

        stream.push_tokens(quote! { else if #cond });
        stream.push_scope(|stream| self.group.generate(stream));
    }
}

impl Generate for InterpElse {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        stream.push_tokens(quote! { else });
        stream.push_scope(|stream| self.group.generate(stream));
    }
}

impl Generate for Name {
    fn generate(&self, stream: &mut Stream) {
        match self {
            Name::Lit(x) => stream.push_raw(&x.value()),
            Name::Ident(x) => stream.push_raw(&x.to_string()),
        }
    }
}

impl Generate for Attrs {
    fn generate(&self, stream: &mut Stream) {
        #[inline]
        fn is_inert(attr: &Attr) -> bool {
            attr.value.as_ref().is_none_or(|x| {
                matches!(
                    x,
                    AttrValue {
                        expr: Expr::Lit(_),
                        ..
                    }
                )
            })
        }

        for attr in self.inner.iter().filter(|x| is_inert(x)) {
            stream.push_raw(" ");
            attr.generate(stream);
        }

        for attr in self.inner.iter().filter(|x| !is_inert(x)) {
            stream.push_raw(" ");
            attr.generate(stream);
        }
    }
}

impl Generate for Attr {
    fn generate(&self, stream: &mut Stream) {
        match &self.value {
            Some(AttrValue {
                expr:
                    syn::Expr::Lit(ExprLit {
                        lit: Lit::Bool(lit),
                        ..
                    }),
                ..
            }) => {
                if lit.value {
                    self.name.generate(stream);
                    stream.push_raw("=\"");
                    self.name.generate(stream);
                    stream.push_raw("\"");
                }
            }

            Some(AttrValue {
                expr: syn::Expr::Lit(ExprLit { lit, .. }),
                ..
            }) => {
                self.name.generate(stream);
                stream.push_raw("=\"");
                lit.generate(stream);
                stream.push_raw("\"");
            }

            None => {}

            Some(AttrValue { expr, .. }) => {
                let name = match &self.name {
                    Name::Lit(x) => x.value(),
                    Name::Ident(x) => x.to_string(),
                };

                let (crate_ident, _) = stream.idents();
                stream.push_write(quote! { #crate_ident::Attr(#name, #expr) });

                stream.estimated_len += name.len();
                stream.estimated_len += 3;
            }
        }
    }
}

impl Generate for Lit {
    fn generate(&self, stream: &mut Stream) {
        match self {
            Lit::Str(x) => x.generate(stream),

            Lit::Byte(x) => stream.push_raw(&x.value().to_string()),
            Lit::Char(x) => stream.push_raw(&x.value().to_string()),
            Lit::Int(x) => stream.push_raw(x.base10_digits()),
            Lit::Float(x) => stream.push_raw(x.base10_digits()),

            Lit::Bool(x) => {
                if x.value {
                    stream.push_raw("true");
                } else {
                    stream.push_raw("false");
                }
            }

            lit => {
                stream.push_write(lit.to_token_stream());
                stream.estimated_len += 24;
            }
        }
    }
}

impl Generate for LitStr {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        stream.push_escaped(&self.value());
    }
}

impl ToTokens for Name {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Name::Lit(x) => x.to_tokens(tokens),
            Name::Ident(x) => quote! { ::core::stringify!(#x) }.to_tokens(tokens),
        }
    }
}
