use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    Expr, Ident, LitStr, Pat, Stmt, Token,
    punctuated::Punctuated,
    token::{Brace, Bracket},
};

pub struct Input {
    pub crate_ident: CrateIdent,
    pub _comma: Token![,],
    pub tokens: TokenStream,
}

pub enum CrateIdent {
    Crate(Token![crate]),
    Ident(Ident),
}

impl Clone for CrateIdent {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::Crate(x) => Self::Crate(Token![crate](x.span)),
            Self::Ident(x) => Self::Ident(x.clone()),
        }
    }
}

pub struct Group(pub Box<[Node]>);

#[allow(clippy::large_enum_variant)]
pub enum Node {
    Element(Element),
    Interp(Interp),
    Literal(LitStr),
}

pub enum Element {
    Normal(Normal),
    Void(Void),
}

pub struct Normal {
    pub name: Name,
    pub attrs: Attrs,
    pub _brace: Brace,
    pub inner: Group,
}

pub struct Void {
    pub name: Name,
    pub attrs: Attrs,
    pub _semi_token: Token![;],
}

pub enum Name {
    Lit(LitStr),
    Ident(Ident),
}

pub struct Attrs {
    pub _bracket: Bracket,
    pub inner: Punctuated<Attr, Token![,]>,
}

pub struct Attr {
    pub name: Name,
    pub value: Option<AttrValue>,
}

pub struct AttrValue {
    pub _eq_token: Token![=],
    pub expr: Expr,
}

pub struct Interp {
    pub _at_token: Token![@],
    pub value: InterpValue,
}

pub enum InterpValue {
    Match(InterpMatch),
    If(InterpIf),
    For(InterpFor),
    Expr(Expr),
    Stmt(Stmt),
}

pub struct InterpFor {
    pub _for_token: Token![for],
    pub pat: Pat,
    pub _in_token: Token![in],
    pub expr: Expr,
    pub _brace: Brace,
    pub body: Group,
}

pub struct InterpMatch {
    pub _match_token: Token![match],
    pub expr: Expr,
    pub _brace_token: Brace,
    pub arms: Box<[InterpArm]>,
}

pub struct InterpArm {
    pub pat: Pat,
    pub _fat_arrow_token: Token![=>],
    pub expr: InterpArmExpr,
    pub _comma_token: Option<Token![,]>,
}

pub enum InterpArmExpr {
    Group(InterpArmGroup),
    Literal(LitStr),
}

pub struct InterpArmGroup {
    pub _brace: Brace,
    pub group: Group,
}

pub struct InterpIf {
    pub _if_token: Token![if],
    pub cond: Expr,
    pub _brace: Brace,
    pub then_branch: Group,
    pub else_if: Box<[InterpElseIf]>,
    pub else_branch: Option<InterpElse>,
}

pub struct InterpElseIf {
    pub _else_token: Token![else],
    pub _if_token: Token![if],
    pub cond: Expr,
    pub _brace: Brace,
    pub group: Group,
}

pub struct InterpElse {
    pub _else_token: Token![else],
    pub _brace: Brace,
    pub group: Group,
}

impl ToTokens for CrateIdent {
    #[inline]
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            CrateIdent::Crate(x) => x.to_tokens(tokens),
            CrateIdent::Ident(x) => x.to_tokens(tokens),
        }
    }
}
