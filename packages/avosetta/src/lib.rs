//! # avosetta
//!
//! A fast, minimal html templating language for Rust.
//!
//! ## about
//!
//! `avosetta` is a minimal templating library for that utilises procedural
//! macros to generate as close to optimal code as possible for rendering html
//! content at runtime. It has no `unsafe` code, only a handful of dependencies, and
//! does allocate any values on the heap.
//!
//! We implement a terse, simple syntax for specifying templates that is
//! straightforward to parse, has little ambiguity and integrates into `Rust`
//! code better. And unlike other templating libraries such as `maud`, our syntax
//! typically only has a single way of writing various constructs, reducing
//! code-style clashing. For more information, read the full syntax reference
//! [here](#reference).
//!
//! Optimisations include automatically escaping static string literals at
//! compile-time and collapsing contiguous [`String::push_str`] calls into a single one.
//! Therefore, if your html fragment is entirely static, the generated code will
//! just be a single [`String::push_str`] with a [`&str`].
//!
//! ## getting started
//!
//! To start using `avosetta`, you'll first need to add our package to your
//! `Cargo.toml` manifest:
//!
//! ```toml
//! [dependencies]
//! avosetta = "0.1.0"
//! ```
//!
//! Then you can start writing html templates directly in your `Rust` source
//! code. We recommend that you import the `prelude` module to reduce unnecessary
//! qualifications, but that's up to you.
//!
//! ```rust
//! use avosetta::prelude::*;
//!
//! fn main() {
//!   let mut s = String::new();
//!   index().write(&mut s);
//!
//!   println!("{s}");
//! }
//!
//! fn index() -> impl Html {
//!   html! {
//!     @layout(
//!       html! {
//!         title { "avosetta" }
//!       },
//!
//!       html! {
//!         h1 { "Hello, World!" }
//!       },
//!     )
//!   }
//! }
//!
//! fn layout(
//!   head: impl Html,
//!   body: impl Html,
//! ) -> impl Html {
//!   html! {
//!     "!DOCTYPE"[html];
//!     html[lang="en"] {
//!       head {
//!         meta[charset="UTF-8"];
//!         meta[name="viewport", content="width=device-width,initial-scale=1"];
//!
//!         @head
//!       }
//!
//!       body {
//!         main {
//!           @body
//!         }
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! ## reference
//!
//! The syntax that this macro accepts is unique to `avosetta`, however, it
//! shares some major similarities with crates such as `maud` and `markup`. All of
//! these crates implement a terse, pug-like syntax which integrates into rust code
//! better and is less error-prone.
//!
//! Unlike the other crates, however, `avosetta` has a more minimal syntax.
//!
//! ### elements
//!
//! There are two types of elements that can be defined: `normal` and `void`.
//! `void` elements cannot have a body and must be terminated with a `semicolon`.
//!
//! ```rust
//! # use avosetta::prelude::*;
//! html! {
//!   // A `normal` element that will be rendered as: `<article></article>`
//!   article {}
//!
//!   // A `void` element that will be rendered as: `<br>`
//!   br;
//!
//!   // Element names can also be string literals:
//!   "x-custom-element" {}
//!
//!   "x-custom-element";
//! };
//! ```
//!
//! ### attributes
//!
//! Elements can also have attributes, which is a `comma` delimited list of
//! `key=value` pairs. Note, that attribute values can be dynamic (interpolated at
//! runtime), where as attribute names must be known at compile time.
//!
//! ```rust
//! # use avosetta::prelude::*;
//! html! {
//!   // A `meta` element with an attribute:
//!   meta[charset="UTF-8"];
//!
//!   // Elements can have multiple attributes:
//!   meta[name="viewport", content="width=device-width,initial-scale=1"];
//!
//!   // Attribute names can also be string literals:
//!   div["x-data"="Hello, World!"] {}
//!
//!   // Attribute values can be any `Rust` expression:
//!   input[value={4 + 3}];
//!
//!   // Attributes without a value are implicitly a `true` boolean attribute:
//!   input["type"="checkbox", checked];
//!   input["type"="checkbox", checked=true]; // These two elements are equivalent.
//! };
//! ```
//!
//! ### interpolation
//!
//! The process of "injecting" or writing dynamic content into the html is
//! called `interpolation`. This might be used for displaying a local variable
//! containing a username, or for performing a conditional `if` check before
//! rendering some sub-content.
//!
//! All interpolations start with an `@`, however, depending on the context,
//! different interpolations will be generated in the [`Html`] implementation.
//!
//! ```rust
//! # use avosetta::prelude::*;
//! let x = 9;
//!
//! html! {
//!   // The most basic interpolation of a simple expression:
//!   @x
//!
//!   // More complicated expressions can also be interpolated:
//!   @x + 2
//!
//!   // Depending on what you're interpolating, it may remove ambiguity to use
//!   // a block expression:
//!   @{ x + 2 }
//!
//!   // `Html` is implemented for `()`, therefore, expressions don't need to
//!   // return any html content:
//!   @{
//!     // This will be executed when the [`Html`] is written to a [`String`].
//!     println!("Hello, World!");
//!   }
//!
//!   // You can conditionally render content using the normal `Rust` syntax:
//!   @if x > 8 {
//!     // Notice how these arms take [`html!`] syntax and not `Rust` syntax.
//!     h1 { "Hello, World!" }
//!   } else if x < 2 {
//!     h2 { "Hello, World!" }
//!   } else {
//!     h3 { "Hello, World!" }
//!   }
//!
//!   // The same concept applies to both the `match` and `for` keywords:
//!   @match x {
//!     // Each arm must be wrapped in braces.
//!     8 => {
//!       h1 { "Hello, World!" }
//!     }
//!
//!     // Except for simple string literals.
//!     _ => "Hello, World!"
//!   }
//!
//!   @for i in 0..24 {
//!     // Nested interpolation works as you'd expect.
//!     span { @i }
//!   }
//! };
//! ```
//!
//! ### string literals
//!
//! Whilst most content can be interpolated into the html at runtime, there
//! is a specific optimisation made for static string literals. When used without an
//! `@`, the string literals are automatically escaped at compile-time and rendered
//! using `avosetta::raw`.
//!
//! ```rust
//! # use avosetta::prelude::*;
//! html! {
//!   // Both of these elements will render to the same html, however,
//!   // the first one will escape the string at runtime because it is an
//!   // interpolated expression.
//!   h1 { @"Hello, World!" }
//!
//!   // This one will be pre-escaped at compile time, avoiding the performance
//!   // cost of runtime escaping.
//!   h1 { "Hello, World!" }
//! };
//! ```

pub use ::avosetta_macros::html;

pub mod prelude {
    pub use crate::{Html, html};
}

/// Represents a fragment of valid html that can be written to a `String`.
///
/// This trait is the backbone of `avosetta` and is implemented for most
/// primitives, such as integers, floats and strings. In some sense, this is a
/// specialized equivalent of the [`std::fmt::Display`] but without the overhead
/// associated with the [`std::fmt`] family of functions.
pub trait Html {
    fn write(self, s: &mut String);
}

/// Text that should not be escaped at runtime, such as dynamically generated
/// html, or some content that has already been escaped.
///
/// Note, that string literals are automatically escaped at compile-time when
/// used within [`html!`], therefore, one should not wrap static content with `Raw` in
/// an attempt to improve performance.
///
/// ```rust
/// # use avosetta::prelude::*;
/// html! {
///    @avosetta::raw("Hello, World!")
/// };
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Raw<T>(pub T);

/// Text that should not be escaped at runtime, such as dynamically generated
/// html, or some content that has already been escaped.
///
/// Note, that string literals are automatically escaped at compile-time when
/// used within [`html!`], therefore, one should not wrap static content with `Raw` in
/// an attempt to improve performance.
///
/// ```rust
/// # use avosetta::prelude::*;
/// html! {
///    @avosetta::raw("Hello, World!")
/// };
/// ```
#[inline]
pub const fn raw<T>(x: T) -> Raw<T> {
    Raw(x)
}

impl Html for () {
    #[inline]
    fn write(self, _: &mut String) {}
}

impl Html for bool {
    #[inline]
    fn write(self, s: &mut String) {
        if self {
            s.push_str("true");
        } else {
            s.push_str("false");
        }
    }
}

impl Html for char {
    #[inline]
    fn write(self, s: &mut String) {
        match self {
            '&' => s.push_str("&amp;"),

            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),

            '\'' => s.push_str("&apos;"),
            '\"' => s.push_str("&quot;"),

            ch => {
                s.push(ch);
            }
        }
    }
}

impl Html for &str {
    #[inline]
    fn write(self, s: &mut String) {
        for ch in self.chars() {
            ch.write(s);
        }
    }
}

impl Html for Raw<&str> {
    #[inline]
    fn write(self, s: &mut String) {
        s.push_str(self.0);
    }
}

impl<T> Html for Option<T>
where
    T: Html,
{
    #[inline]
    fn write(self, s: &mut String) {
        if let Some(x) = self {
            x.write(s);
        }
    }
}

impl<T> Html for T
where
    T: FnOnce(&mut String),
{
    #[inline]
    fn write(self, s: &mut String) {
        (self)(s)
    }
}

macro_rules! __impl_html_integer {
    ($ty:ty) => {
        impl $crate::Html for $ty {
            #[inline]
            fn write(self, s: &mut ::std::string::String) {
                s.push_str(::itoa::Buffer::new().format(self));
            }
        }
    };
}

__impl_html_integer!(usize);
__impl_html_integer!(isize);

__impl_html_integer!(u8);
__impl_html_integer!(i8);

__impl_html_integer!(u16);
__impl_html_integer!(i16);

__impl_html_integer!(u32);
__impl_html_integer!(i32);

__impl_html_integer!(u64);
__impl_html_integer!(i64);

__impl_html_integer!(u128);
__impl_html_integer!(i128);

macro_rules! __impl_html_float {
    ($ty:ty) => {
        impl $crate::Html for $ty {
            #[inline]
            fn write(self, s: &mut ::std::string::String) {
                s.push_str(::ryu::Buffer::new().format(self));
            }
        }
    };
}

__impl_html_float!(f32);
__impl_html_float!(f64);

/// A html attribute, containing both a key and value.
///
/// For most value types, this struct simply renders out `K=\"V\"`, where
/// `K` is written as a [`&str`] directly, without any escaping and `V` is converted
/// to its [`Html`] representation. However, there are two edge case types that are
/// handled differently:
///
/// - [`bool`]: If the value is [`false`], then the entire attribute is ommitted from the
/// output html. If the value is [`true`], then just the `K` is written out.
///
/// - [`Option<T>`]: This struct will omit the attribute, if the provided [`Option<T>`] is [`None`],
/// and will use the [`Attr<&str, T>`] implementation if the provided value is [`Some`].
///
/// Note: This struct only implements [`Html`] for [`&str`] keys, since html
/// attributes have very stringent requirements on what constitutes a valid name.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attr<K, V>(pub K, pub V);

impl Html for Attr<&str, &str> {
    #[inline]
    fn write(self, s: &mut String) {
        s.push_str(self.0);
        s.push_str("=\"");
        self.1.write(s);
        s.push_str("\"");
    }
}

impl Html for Attr<&str, Raw<&str>> {
    #[inline]
    fn write(self, s: &mut String) {
        s.push_str(self.0);
        s.push_str("=\"");
        self.1.write(s);
        s.push_str("\"");
    }
}

impl Html for Attr<&str, char> {
    #[inline]
    fn write(self, s: &mut String) {
        s.push_str(self.0);
        s.push_str("=\"");
        self.1.write(s);
        s.push_str("\"");
    }
}

impl Html for Attr<&str, bool> {
    #[inline]
    fn write(self, s: &mut String) {
        if self.1 {
            s.push_str(self.0);
        }
    }
}

impl<T> Html for Attr<&str, Option<T>>
where
    for<'a> Attr<&'a str, T>: Html,
{
    #[inline]
    fn write(self, s: &mut String) {
        if let Some(x) = self.1 {
            Attr(self.0, x).write(s);
        }
    }
}

impl<T> Html for Attr<&str, T>
where
    T: FnOnce(&mut String),
{
    #[inline]
    fn write(self, s: &mut String) {
        s.push_str(self.0);
        s.push_str("=\"");
        (self.1)(s);
        s.push_str("\"");
    }
}

macro_rules! __impl_attr_integer {
    ($ty:ty) => {
        impl $crate::Html for $crate::Attr<&str, $ty> {
            #[inline]
            fn write(self, s: &mut ::std::string::String) {
                s.push_str(self.0);
                s.push_str("=\"");
                s.push_str(::itoa::Buffer::new().format(self.1));
                s.push_str("\"");
            }
        }
    };
}

__impl_attr_integer!(usize);
__impl_attr_integer!(isize);

__impl_attr_integer!(u8);
__impl_attr_integer!(i8);

__impl_attr_integer!(u16);
__impl_attr_integer!(i16);

__impl_attr_integer!(u32);
__impl_attr_integer!(i32);

__impl_attr_integer!(u64);
__impl_attr_integer!(i64);

__impl_attr_integer!(u128);
__impl_attr_integer!(i128);

macro_rules! __impl_attr_float {
    ($ty:ty) => {
        impl $crate::Html for $crate::Attr<&str, $ty> {
            #[inline]
            fn write(self, s: &mut ::std::string::String) {
                s.push_str(self.0);
                s.push_str("=\"");
                s.push_str(::ryu::Buffer::new().format(self.1));
                s.push_str("\"");
            }
        }
    };
}

__impl_attr_float!(f32);
__impl_attr_float!(f64);
