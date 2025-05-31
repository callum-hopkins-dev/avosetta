use std::sync::OnceLock;

use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{ExprLit, Lit, LitStr};

use crate::ast::{
    Attr, Attrs, Element, Group, Interp, InterpArm, InterpArmExpr, InterpArmGroup, InterpElse,
    InterpElseIf, InterpFor, InterpIf, InterpMatch, InterpValue, Name, Node, Normal, Void,
};

#[derive(Debug, Clone)]
pub struct Stream {
    stream: proc_macro2::TokenStream,
    buffer: String,
}

impl Stream {
    #[inline]
    pub fn new() -> Self {
        Self {
            stream: proc_macro2::TokenStream::new(),
            buffer: String::new(),
        }
    }

    #[inline]
    pub fn push_char(&mut self, ch: char) {
        self.buffer.push(ch);
    }

    #[inline]
    pub fn push_raw(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    pub fn push_escaped(&mut self, s: &str) {
        for ch in s.chars() {
            match ch {
                '&' => self.buffer.push_str("&amp;"),

                '<' => self.buffer.push_str("&lt;"),
                '>' => self.buffer.push_str("&gt;"),

                '\'' => self.buffer.push_str("&apos;"),
                '\"' => self.buffer.push_str("&quot;"),

                ch => self.buffer.push(ch),
            }
        }
    }

    pub fn push_tokens(&mut self, expr: proc_macro2::TokenStream) {
        if !self.buffer.is_empty() {
            let literal = proc_macro2::Literal::string(&self.buffer);

            quote! { #CrateIdent::Html::write(#CrateIdent::raw(#literal), __s); }
                .to_tokens(&mut self.stream);

            self.buffer.clear();
        }

        expr.to_tokens(&mut self.stream);
    }

    #[inline]
    pub fn push_write(&mut self, expr: proc_macro2::TokenStream) {
        self.push_tokens(quote! { #CrateIdent::Html::write(#expr, __s); });
    }

    #[inline]
    pub fn push_braced<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self),
    {
        let mut stream = Self::new();
        (f)(&mut stream);
        self.push_tokens(quote! { { #stream } });
    }
}

impl ToTokens for Stream {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut stream = self.stream.to_token_stream();

        if !self.buffer.is_empty() {
            let literal = proc_macro2::Literal::string(&self.buffer);

            quote! { #CrateIdent::Html::write(#CrateIdent::raw(#literal), __s); }
                .to_tokens(&mut stream);
        }

        stream.to_tokens(tokens);
    }
}

pub trait Generate {
    fn generate(&self, stream: &mut Stream);

    #[inline]
    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        let mut stream = Stream::new();
        self.generate(&mut stream);

        quote! {{
            #[inline]
            const fn coerce<T: #CrateIdent::Html>(x: T) -> impl #CrateIdent::Html { x }
            coerce(move |__s: &mut ::std::string::String| { #stream })
        }}
    }
}

impl Generate for Group {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        for node in &self.0 {
            node.generate(stream);
        }
    }
}

impl Generate for Node {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        match self {
            Node::Element(element) => element.generate(stream),
            Node::Interp(interp) => interp.generate(stream),
            Node::Literal(lit_str) => lit_str.generate(stream),
        }
    }
}

impl Generate for Interp {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        self.value.generate(stream);
    }
}

impl Generate for InterpValue {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        match self {
            InterpValue::Match(interp_match) => {
                interp_match.generate(stream);
            }

            InterpValue::If(interp_if) => {
                interp_if.generate(stream);
            }

            InterpValue::Expr(expr) => match &expr {
                syn::Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) => {
                    stream.push_escaped(&lit_str.value());
                }

                expr => {
                    stream.push_write(quote! { #expr });
                }
            },

            InterpValue::For(interp_for) => {
                interp_for.generate(stream);
            }
        }
    }
}

impl Generate for InterpMatch {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        let expr = &self.expr;

        stream.push_tokens(quote! { match #expr });

        stream.push_braced(|stream| {
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
        stream.push_braced(|stream| self.expr.generate(stream));
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
        stream.push_braced(|stream| self.group.generate(stream));
    }
}

impl Generate for InterpFor {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        let Self {
            pat, expr, body, ..
        } = &self;

        stream.push_tokens(quote! { for #pat in #expr });
        stream.push_braced(|stream| body.generate(stream));
    }
}

impl Generate for InterpIf {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        let cond = &self.cond;

        stream.push_tokens(quote! { if #cond });
        stream.push_braced(|stream| self.then_branch.generate(stream));

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
        stream.push_braced(|stream| self.group.generate(stream));
    }
}

impl Generate for InterpElse {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        stream.push_tokens(quote! { else });
        stream.push_braced(|stream| self.group.generate(stream));
    }
}

impl Generate for Element {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        match self {
            Element::Normal(normal) => normal.generate(stream),
            Element::Void(void) => void.generate(stream),
        }
    }
}

impl Generate for Normal {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        stream.push_char('<');
        self.name.generate(stream);

        if let Some(attrs) = &self.attrs {
            attrs.generate(stream);
        }

        stream.push_char('>');

        if let Some(inner) = &self.inner {
            inner.generate(stream);
        }

        stream.push_char('<');
        stream.push_char('/');
        self.name.generate(stream);
        stream.push_char('>');
    }
}

impl Generate for Void {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        stream.push_char('<');
        self.name.generate(stream);

        if let Some(attrs) = &self.attrs {
            attrs.generate(stream);
        }

        stream.push_char('>');
    }
}

impl Generate for Name {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        match self {
            Name::Lit(lit_str) => {
                stream.push_raw(&lit_str.value());
            }

            Name::Ident(ident) => {
                stream.push_raw(&ident.to_string());
            }
        }
    }
}

impl Generate for Attrs {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        for attr in &self.inner {
            attr.generate(stream);
        }
    }
}

impl Generate for Attr {
    fn generate(&self, stream: &mut Stream) {
        if let Some(value) = &self.value {
            match &value.expr {
                syn::Expr::Lit(ExprLit {
                    lit: Lit::Str(lit_str),
                    ..
                }) => {
                    stream.push_char(' ');
                    self.name.generate(stream);
                    stream.push_char('=');
                    stream.push_char('\"');
                    stream.push_escaped(&lit_str.value());
                    stream.push_char('\"');
                }

                syn::Expr::Lit(ExprLit {
                    lit: Lit::Bool(lit_bool),
                    ..
                }) => {
                    if lit_bool.value() {
                        stream.push_char(' ');
                        self.name.generate(stream);
                    } else {
                    }
                }

                expr => {
                    let name = match &self.name {
                        Name::Lit(lit_str) => lit_str.clone(),
                        Name::Ident(ident) => LitStr::new(&ident.to_string(), Span::call_site()),
                    };

                    stream.push_char(' ');

                    stream.push_write(quote! { #CrateIdent::Attr(#name, #expr) });
                }
            }
        } else {
            stream.push_char(' ');
            self.name.generate(stream);
        }
    }
}

impl Generate for LitStr {
    #[inline]
    fn generate(&self, stream: &mut Stream) {
        stream.push_escaped(&self.value());
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CrateIdent;

impl ToTokens for CrateIdent {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        static CRATE_NAME: OnceLock<Box<str>> = OnceLock::new();

        let crate_name =
            CRATE_NAME.get_or_init(|| match proc_macro_crate::crate_name("avosetta").unwrap() {
                proc_macro_crate::FoundCrate::Itself => "avosetta".into(),

                proc_macro_crate::FoundCrate::Name(x) => x.into_boxed_str(),
            });

        proc_macro2::Ident::new(&crate_name, proc_macro2::Span::call_site()).to_tokens(tokens);
    }
}
