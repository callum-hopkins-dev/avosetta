use syn::{
    Expr, Ident, LitStr, Pat, Token,
    punctuated::Punctuated,
    token::{Brace, Bracket},
};

#[derive(Clone)]
pub struct Group(pub Box<[Node]>);

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Interp(Interp),
    Literal(LitStr),
}

#[derive(Clone)]
pub enum Element {
    Normal(Normal),
    Void(Void),
}

#[derive(Clone)]
pub struct Normal {
    pub name: Name,
    pub attrs: Option<Attrs>,
    pub _brace: Brace,
    pub inner: Option<Group>,
}

#[derive(Clone)]
pub struct Void {
    pub name: Name,
    pub attrs: Option<Attrs>,
    pub _semi_token: Token![;],
}

#[derive(Clone)]
pub enum Name {
    Lit(LitStr),
    Ident(Ident),
}

#[derive(Clone)]
pub struct Attrs {
    pub _bracket: Bracket,
    pub inner: Punctuated<Attr, Token![,]>,
}

#[derive(Clone)]
pub struct Attr {
    pub name: Name,
    pub value: Option<AttrValue>,
}

#[derive(Clone)]
pub struct AttrValue {
    pub _eq_token: Token![=],
    pub expr: Expr,
}

#[derive(Clone)]
pub struct Interp {
    pub _at_token: Token![@],
    pub value: InterpValue,
}

#[derive(Clone)]
pub enum InterpValue {
    Match(InterpMatch),
    If(InterpIf),
    Expr(Expr),
    For(InterpFor),
}

#[derive(Clone)]
pub struct InterpFor {
    pub _for_token: Token![for],
    pub pat: Pat,
    pub _in_token: Token![in],
    pub expr: Expr,
    pub _brace: Brace,
    pub body: Group,
}

#[derive(Clone)]
pub struct InterpMatch {
    pub _match_token: Token![match],
    pub expr: Expr,
    pub _brace_token: Brace,
    pub arms: Box<[InterpArm]>,
}

#[derive(Clone)]
pub struct InterpArm {
    pub pat: Pat,
    pub _fat_arrow_token: Token![=>],
    pub expr: InterpArmExpr,
    pub _comma_token: Option<Token![,]>,
}

#[derive(Clone)]
pub enum InterpArmExpr {
    Group(InterpArmGroup),
    Literal(LitStr),
}

#[derive(Clone)]
pub struct InterpArmGroup {
    pub _brace: Brace,
    pub group: Group,
}

#[derive(Clone)]
pub struct InterpIf {
    pub _if_token: Token![if],
    pub cond: Expr,
    pub _brace: Brace,
    pub then_branch: Group,
    pub else_if: Box<[InterpElseIf]>,
    pub else_branch: Option<InterpElse>,
}

#[derive(Clone)]
pub struct InterpElseIf {
    pub _else_token: Token![else],
    pub _if_token: Token![if],
    pub cond: Expr,
    pub _brace: Brace,
    pub group: Group,
}

#[derive(Clone)]
pub struct InterpElse {
    pub _else_token: Token![else],
    pub _brace: Brace,
    pub group: Group,
}
