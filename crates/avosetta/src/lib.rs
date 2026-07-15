#![forbid(unsafe_code)]

//! Rust-native HTML templates with compile-time optimization.
//!
//! `avosetta` provides the [`asx!`] macro, a compact HTML templating syntax that
//! follows Rust's expression and control-flow conventions. A template expands to
//! an opaque value implementing [`Html`]; render it by calling [`Html::write`]
//! with a `String` buffer.
//!
//! The macro is designed to leave very little work for runtime. Adjacent static
//! output is combined into larger string writes, and HTML escaping for static
//! string literals is performed during compilation. Dynamic values are rendered
//! through [`Html`].
//!
//! # Example
//!
//! ```rust
//! use avosetta::{asx, Html};
//!
//! let name = "<Ada>";
//! let page = asx! {
//!     main[class="profile"] {
//!         h1 { "Hello, " @name }
//!         input[disabled=true];
//!     }
//! };
//!
//! let mut html = String::new();
//! page.write(&mut html);
//!
//! assert_eq!(
//!     html,
//!     r#"<main class="profile"><h1>Hello, &lt;Ada&gt;</h1><input disabled="disabled"></main>"#,
//! );
//! ```
//!
//! # Syntax reference
//!
//! ## Elements
//!
//! Write an element name followed by a braced child template:
//!
//! ```rust
//! # avosetta::asx! {
//! div {
//!     span { "content" }
//! }
//! # };
//! ```
//!
//! Void elements end with a semicolon instead of a child block:
//!
//! ```rust
//! # avosetta::asx! {
//! meta[charset="utf-8"];
//! input[type="text", required=true];
//! # };
//! ```
//!
//! Names that are not Rust identifiers, including custom elements, can be string
//! literals:
//!
//! ```rust
//! # avosetta::asx! {
//! "x-user-card" {
//!     div["x-data"="open"] { }
//! }
//! # };
//! ```
//!
//! Template formatting is not copied to the output. Add a string literal when
//! whitespace is significant.
//!
//! ## Attributes
//!
//! Attributes are written in square brackets after an element name. Separate
//! entries with commas, and use string literals for names that cannot be written
//! as Rust identifiers:
//!
//! ```rust
//! # let destination = ();
//! # let label = ();
//! # avosetta::asx! {
//! a[href=destination, class="button", "aria-label"=label] {
//!     "Open"
//! }
//! # };
//! ```
//!
//! Attribute values are Rust expressions and are rendered through [`Html`].
//! String values are escaped. Boolean and optional values are useful for
//! conditional attributes: `false` and `None` omit the attribute, while `true`
//! emits `name="name"`.
//!
//! ## Text and interpolation
//!
//! A string literal can appear directly in a template. It is escaped at compile
//! time and becomes part of the macro's static output:
//!
//! ```rust
//! # avosetta::asx! {
//! p { "5 < 8 & 8 > 5" }
//! # };
//! ```
//!
//! Prefix a Rust expression with `@` to interpolate it. The expression's result
//! must implement [`Html`]:
//!
//! ```rust
//! # let user_name = ();
//! # let count = 0;
//! # avosetta::asx! {
//! p { "Welcome, " @user_name }
//! p { @format_args!("{} items", count) }
//! # };
//! ```
//!
//! Dynamic strings and characters are escaped at runtime. To insert trusted,
//! already-rendered markup without escaping, wrap it in [`Raw`]:
//!
//! ```rust
//! # use avosetta::Raw;
//! # avosetta::asx! {
//! div { @Raw("<strong>trusted HTML</strong>") }
//! # };
//! ```
//!
//! Only use [`Raw`] for content whose origin and safety you control.
//!
//! ## Rust statements and control flow
//!
//! `@` also introduces Rust statements and control-flow forms. Their template
//! bodies use `avosetta` syntax, so interpolated Rust inside those bodies still
//! needs `@`:
//!
//! ```rust
//! # let messages = [()];
//! # avosetta::asx! {
//! @let heading = "Messages";
//!
//! h1 { @heading }
//!
//! @if messages.is_empty() {
//!     p { "No messages" }
//! } else {
//!     ul {
//!         @for message in messages {
//!             li { @message }
//!         }
//!     }
//! }
//! # };
//! ```
//!
//! `match` follows Rust's arm syntax, but template-producing arms use braces. A
//! static string literal may be used directly as an arm body:
//!
//! ```rust
//! # enum Status { Ready, Waiting }
//! # let status = Status::Ready;
//! # avosetta::asx! {
//! @match status {
//!     Status::Ready => { strong { "Ready" } }
//!     Status::Waiting => "Waiting",
//! }
//! # };
//! ```
//!
//! Local Rust items and statements may also be introduced with `@`. Values from
//! the surrounding scope can be referenced normally; the generated template
//! captures them with move semantics.
//!
//! # Rendering values
//!
//! [`Html`] is implemented for common text and numeric types, booleans,
//! [`Option`], [`Result`], formatting [`std::fmt::Arguments`], slices, boxed
//! slices, and vectors. Collections render their items in order. `None` renders
//! nothing, and a [`Result`] renders whichever variant it contains.
//!
//! Implement [`Html`] for application-specific renderable values, or compose
//! templates by returning the opaque [`Html`] value produced by [`asx!`].

use std::{
    fmt::{Arguments, Write},
    rc::Rc,
    sync::Arc,
};

#[cfg(feature = "macros")]
#[doc(hidden)]
pub use avosetta_macros::asx as __asx;

/// Builds an optimized HTML template using Rust-like syntax.
///
/// The macro returns an opaque value implementing [`Html`]. Static portions of
/// the template are escaped and combined at compile time; interpolated values
/// are rendered when [`Html::write`] is called.
///
/// See the [crate-level syntax reference](crate#syntax-reference) for elements,
/// attributes, interpolation, and control flow.
///
/// This macro is available when the `macros` crate feature is enabled.
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! asx {
    ($($tt:tt)*) => {
        $crate::__asx!($crate, $($tt)*)
    };
}

/// A value that can append an HTML representation to a string buffer.
///
/// Text-oriented implementations should escape HTML-sensitive characters unless
/// the type explicitly represents trusted, already-rendered markup, as [`Raw`]
/// does. Implementations may write an empty string.
///
/// The trait consumes `self`, allowing template values to own and move captured
/// data without requiring allocation for an intermediate representation.
pub trait Html {
    /// Appends this value's HTML representation to `s`.
    fn write(self, s: &mut String);

    #[doc(hidden)]
    #[inline]
    fn is_none(&self) -> bool {
        false
    }

    #[doc(hidden)]
    #[inline]
    fn is_false(&self) -> bool {
        false
    }

    #[doc(hidden)]
    #[inline]
    fn is_true(&self) -> bool {
        false
    }
}

impl Html for () {
    #[inline]
    fn write(self, _s: &mut String) {}
}

/// Marks a string-like value as trusted HTML and writes it without escaping.
///
/// `Raw` accepts any value implementing [`AsRef<str>`]. It is useful for
/// pre-rendered markup, but it bypasses the escaping normally applied to strings.
/// Never wrap untrusted or user-controlled input in `Raw`.
///
/// # Example
///
/// ```rust
/// use avosetta::{Html, Raw};
///
/// let mut output = String::new();
/// Raw("<em>already rendered</em>").write(&mut output);
/// assert_eq!(output, "<em>already rendered</em>");
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Raw<T>(pub T);

impl<T> Html for Raw<T>
where
    T: AsRef<str>,
{
    #[inline]
    fn write(self, s: &mut String) {
        s.push_str(self.0.as_ref());
    }
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

    #[inline]
    fn is_false(&self) -> bool {
        !*self
    }

    #[inline]
    fn is_true(&self) -> bool {
        *self
    }
}

impl Html for char {
    #[inline]
    fn write(self, s: &mut String) {
        match self {
            '&' => s.push_str("&amp;"),
            '<' => s.push_str("&lt;"),
            '>' => s.push_str("&gt;"),
            '"' => s.push_str("&quot;"),
            '\'' => s.push_str("&#39;"),

            x => s.push(x),
        }
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

    #[inline]
    fn is_none(&self) -> bool {
        self.is_none()
    }

    #[inline]
    fn is_false(&self) -> bool {
        self.as_ref().is_some_and(|x| x.is_false())
    }

    #[inline]
    fn is_true(&self) -> bool {
        self.as_ref().is_some_and(|x| x.is_true())
    }
}

impl<T, E> Html for Result<T, E>
where
    T: Html,
    E: Html,
{
    #[inline]
    fn write(self, s: &mut String) {
        match self {
            Ok(x) => x.write(s),
            Err(x) => x.write(s),
        }
    }

    #[inline]
    fn is_none(&self) -> bool {
        match self {
            Ok(x) => x.is_none(),
            Err(x) => x.is_none(),
        }
    }

    #[inline]
    fn is_false(&self) -> bool {
        match self {
            Ok(x) => x.is_false(),
            Err(x) => x.is_false(),
        }
    }

    #[inline]
    fn is_true(&self) -> bool {
        match self {
            Ok(x) => x.is_true(),
            Err(x) => x.is_true(),
        }
    }
}

impl<T> Html for &T
where
    T: Html + Copy,
{
    #[inline]
    fn write(self, s: &mut String) {
        (*self).write(s);
    }
}

impl<T> Html for &[T]
where
    for<'a> &'a T: Html,
{
    #[inline]
    fn write(self, s: &mut String) {
        for x in self {
            x.write(s);
        }
    }
}

macro_rules! impl_owned_iter {
    ($ty:ty) => {
        impl<T> Html for $ty
        where
            T: Html,
        {
            #[inline]
            fn write(self, s: &mut String) {
                for x in self {
                    x.write(s);
                }
            }
        }
    };
}

impl_owned_iter!(Box<[T]>);
impl_owned_iter!(Vec<T>);

impl Html for Arguments<'_> {
    fn write(self, s: &mut String) {
        struct Writer<'a>(&'a mut String);

        impl Write for Writer<'_> {
            #[inline]
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                s.write(self.0);
                Ok(())
            }

            #[inline]
            fn write_char(&mut self, c: char) -> std::fmt::Result {
                c.write(self.0);
                Ok(())
            }
        }

        match self.as_str() {
            Some(x) => x.write(s),

            None => {
                write!(Writer(s), "{self}").unwrap();
            }
        }
    }
}

/// Escapes a string-like value for safe insertion into HTML text or an
/// attribute value.
///
/// The characters `&`, `<`, `>`, `"`, and `'` are replaced with HTML entities.
/// String types already use this behavior through their [`Html`]
/// implementations; this wrapper is useful when writing generic code over
/// [`AsRef<str>`] values.
///
/// # Example
///
/// ```rust
/// use avosetta::{Escape, Html};
///
/// let mut output = String::new();
/// Escape(r#"<a title='x'>"#).write(&mut output);
/// assert_eq!(output, "&lt;a title=&#39;x&#39;&gt;");
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Escape<T>(pub T);

impl<T> Html for Escape<T>
where
    T: AsRef<str>,
{
    fn write(self, s: &mut String) {
        s.reserve(self.0.as_ref().len());

        for x in self.0.as_ref().chars() {
            match x {
                '&' => s.push_str("&amp;"),
                '<' => s.push_str("&lt;"),
                '>' => s.push_str("&gt;"),
                '"' => s.push_str("&quot;"),
                '\'' => s.push_str("&#39;"),

                x => s.push(x),
            }
        }
    }
}

/// Renders an HTML attribute from a key and value.
///
/// This is the runtime representation used for dynamic attributes. Both the key
/// and value are rendered through [`Html`]. A `true` value produces
/// `key="key"`; `false` and [`Option::None`] omit the attribute; every other value
/// produces `key="value"`.
///
/// Attribute values that use the standard string [`Html`] implementations are
/// HTML-escaped. As with any direct [`Html`] implementation, using [`Raw`] as a
/// value bypasses escaping.
///
/// # Example
///
/// ```rust
/// use avosetta::{Attr, Html};
///
/// let mut output = String::new();
/// Attr("title", "5 < 8").write(&mut output);
/// assert_eq!(output, "title=\"5 &lt; 8\"");
/// ```

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attr<K, V>(pub K, pub V);

impl<K, V> Html for Attr<K, V>
where
    K: Html,
    V: Html,
{
    fn write(self, s: &mut String) {
        if self.1.is_true() {
            let start = s.len();
            self.0.write(s);
            let end = s.len();

            s.push_str("=\"");
            s.extend_from_within(start..end);
            s.push('\"');
        } else if !self.1.is_none() && !self.1.is_false() {
            self.0.write(s);
            s.push_str("=\"");
            self.1.write(s);
            s.push('\"');
        }
    }
}

macro_rules! impl_integer {
    ($ty:ty) => {
        impl Html for $ty {
            #[inline]
            fn write(self, s: &mut String) {
                s.push_str(itoa::Buffer::new().format(self));
            }
        }
    };
}

impl_integer!(usize);
impl_integer!(isize);

impl_integer!(u8);
impl_integer!(i8);

impl_integer!(u16);
impl_integer!(i16);

impl_integer!(u32);
impl_integer!(i32);

impl_integer!(u64);
impl_integer!(i64);

impl_integer!(u128);
impl_integer!(i128);

macro_rules! impl_float {
    ($ty:ty) => {
        impl Html for $ty {
            #[inline]
            fn write(self, s: &mut String) {
                s.push_str(ryu::Buffer::new().format(self));
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);

macro_rules! impl_string {
    ($ty:ty) => {
        impl Html for $ty {
            #[inline]
            fn write(self, s: &mut String) {
                Escape(self).write(s);
            }
        }
    };
}

impl_string!(&str);
impl_string!(String);
impl_string!(Box<str>);
impl_string!(Rc<str>);
impl_string!(Arc<str>);

#[allow(non_camel_case_types)]
#[doc(hidden)]
#[cfg(feature = "macros")]
pub mod __completion {
    pub mod elements {
        /// The document’s root element; every other HTML element belongs beneath it.
        ///
        /// HTML tag: `<html>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/html)
        pub struct html;

        /// Sets the base URL used to resolve relative URLs in the document.
        ///
        /// HTML tag: `<base>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/base)
        pub struct base;

        /// Holds machine-readable document metadata such as the title, styles, and scripts.
        ///
        /// HTML tag: `<head>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/head)
        pub struct head;

        /// Declares a relationship between the document and an external resource.
        ///
        /// HTML tag: `<link>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link)
        pub struct link;

        /// Provides document metadata that is not expressed by another metadata element.
        ///
        /// HTML tag: `<meta>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta)
        pub struct meta;

        /// Contains CSS rules that apply to the document or part of it.
        ///
        /// HTML tag: `<style>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/style)
        pub struct style;

        /// Supplies the document title shown in browser tabs and similar user interfaces.
        ///
        /// HTML tag: `<title>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/title)
        pub struct title;

        /// Contains the visible and interactive content of an HTML document.
        ///
        /// HTML tag: `<body>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/body)
        pub struct body;

        /// Provides contact information for a person, group, or organization.
        ///
        /// HTML tag: `<address>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/address)
        pub struct address;

        /// Marks a self-contained composition that can stand alone or be reused independently.
        ///
        /// HTML tag: `<article>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/article)
        pub struct article;

        /// Contains content only indirectly connected to the surrounding primary content.
        ///
        /// HTML tag: `<aside>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/aside)
        pub struct aside;

        /// Defines footer content for its nearest sectioning ancestor or the page.
        ///
        /// HTML tag: `<footer>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/footer)
        pub struct footer;

        /// Groups introductory material or navigational aids for a page or section.
        ///
        /// HTML tag: `<header>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/header)
        pub struct header;

        /// Defines a level-one section heading, the highest heading rank.
        ///
        /// HTML tag: `<h1>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
        pub struct h1;

        /// Defines a level-two section heading.
        ///
        /// HTML tag: `<h2>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
        pub struct h2;

        /// Defines a level-three section heading.
        ///
        /// HTML tag: `<h3>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
        pub struct h3;

        /// Defines a level-four section heading.
        ///
        /// HTML tag: `<h4>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
        pub struct h4;

        /// Defines a level-five section heading.
        ///
        /// HTML tag: `<h5>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
        pub struct h5;

        /// Defines a level-six section heading, the lowest heading rank.
        ///
        /// HTML tag: `<h6>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
        pub struct h6;

        /// Groups a heading with secondary heading-related content such as a subtitle or tagline.
        ///
        /// HTML tag: `<hgroup>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/hgroup)
        pub struct hgroup;

        /// Identifies the body’s dominant content or central application functionality.
        ///
        /// HTML tag: `<main>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/main)
        pub struct main;

        /// Marks a section whose main purpose is providing navigation links.
        ///
        /// HTML tag: `<nav>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/nav)
        pub struct nav;

        /// Represents a standalone section when no more specific semantic element applies.
        ///
        /// HTML tag: `<section>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/section)
        pub struct section;

        /// Contains controls or content used to perform searching or filtering.
        ///
        /// HTML tag: `<search>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/search)
        pub struct search;

        /// Represents an extended quotation, optionally identifying its source.
        ///
        /// HTML tag: `<blockquote>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/blockquote)
        pub struct blockquote;

        /// Provides the description, definition, or value associated with a preceding term.
        ///
        /// HTML tag: `<dd>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dd)
        pub struct dd;

        /// A generic block container with no inherent semantic meaning.
        ///
        /// HTML tag: `<div>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/div)
        pub struct div;

        /// Contains groups of terms and their corresponding descriptions or values.
        ///
        /// HTML tag: `<dl>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dl)
        pub struct dl;

        /// Identifies a term or name within a description list.
        ///
        /// HTML tag: `<dt>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dt)
        pub struct dt;

        /// Provides a caption or legend for its containing figure.
        ///
        /// HTML tag: `<figcaption>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figcaption)
        pub struct figcaption;

        /// Wraps self-contained content that may include an associated caption.
        ///
        /// HTML tag: `<figure>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figure)
        pub struct figure;

        /// Marks a thematic break between paragraph-level topics or scenes.
        ///
        /// HTML tag: `<hr>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/hr)
        pub struct hr;

        /// Represents one item within an ordered, unordered, or menu list.
        ///
        /// HTML tag: `<li>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/li)
        pub struct li;

        /// Represents an unordered list of commands or other items.
        ///
        /// HTML tag: `<menu>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/menu)
        pub struct menu;

        /// Contains an ordered sequence of list items.
        ///
        /// HTML tag: `<ol>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol)
        pub struct ol;

        /// Represents a paragraph or another cohesive grouping of related phrasing content.
        ///
        /// HTML tag: `<p>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/p)
        pub struct p;

        /// Preserves and displays text formatting and whitespace as written in the source.
        ///
        /// HTML tag: `<pre>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/pre)
        pub struct pre;

        /// Contains an unordered collection of list items.
        ///
        /// HTML tag: `<ul>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ul)
        pub struct ul;

        /// Creates a hyperlink when supplied with an `href` destination.
        ///
        /// HTML tag: `<a>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a)
        pub struct a;

        /// Marks an abbreviation or acronym.
        ///
        /// HTML tag: `<abbr>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/abbr)
        pub struct abbr;

        /// Draws attention to text without assigning it additional importance.
        ///
        /// HTML tag: `<b>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/b)
        pub struct b;

        /// Isolates contained text from the bidirectional directionality of surrounding text.
        ///
        /// HTML tag: `<bdi>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdi)
        pub struct bdi;

        /// Overrides the text direction used to render its contents.
        ///
        /// HTML tag: `<bdo>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdo)
        pub struct bdo;

        /// Inserts a meaningful line break within text.
        ///
        /// HTML tag: `<br>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/br)
        pub struct br;

        /// Marks the title of a creative work.
        ///
        /// HTML tag: `<cite>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/cite)
        pub struct cite;

        /// Identifies a short fragment of computer code.
        ///
        /// HTML tag: `<code>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/code)
        pub struct code;

        /// Associates displayed content with a machine-readable value.
        ///
        /// HTML tag: `<data>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/data)
        pub struct data;

        /// Marks the term currently being defined by its surrounding context.
        ///
        /// HTML tag: `<dfn>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dfn)
        pub struct dfn;

        /// Applies stress emphasis, with nesting indicating stronger emphasis.
        ///
        /// HTML tag: `<em>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/em)
        pub struct em;

        /// Sets off text with an alternate voice, mood, taxonomy, idiom, or technical meaning.
        ///
        /// HTML tag: `<i>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/i)
        pub struct i;

        /// Represents user input from a keyboard, voice interface, or other input device.
        ///
        /// HTML tag: `<kbd>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/kbd)
        pub struct kbd;

        /// Highlights text because it is relevant to the current context.
        ///
        /// HTML tag: `<mark>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/mark)
        pub struct mark;

        /// Represents a short inline quotation.
        ///
        /// HTML tag: `<q>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q)
        pub struct q;

        /// Provides fallback punctuation around ruby annotations for unsupported renderers.
        ///
        /// HTML tag: `<rp>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rp)
        pub struct rp;

        /// Contains pronunciation, translation, or transliteration text for a ruby annotation.
        ///
        /// HTML tag: `<rt>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rt)
        pub struct rt;

        /// Groups base text with small annotations commonly used in East Asian typography.
        ///
        /// HTML tag: `<ruby>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ruby)
        pub struct ruby;

        /// Marks content that is no longer accurate or relevant, commonly rendered struck through.
        ///
        /// HTML tag: `<s>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/s)
        pub struct s;

        /// Represents sample or quoted output from a computer program.
        ///
        /// HTML tag: `<samp>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/samp)
        pub struct samp;

        /// Marks side comments or small-print information such as legal or copyright text.
        ///
        /// HTML tag: `<small>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/small)
        pub struct small;

        /// A generic inline container with no inherent semantic meaning.
        ///
        /// HTML tag: `<span>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/span)
        pub struct span;

        /// Indicates strong importance, seriousness, or urgency.
        ///
        /// HTML tag: `<strong>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/strong)
        pub struct strong;

        /// Displays inline text as typographic subscript.
        ///
        /// HTML tag: `<sub>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sub)
        pub struct sub;

        /// Displays inline text as typographic superscript.
        ///
        /// HTML tag: `<sup>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sup)
        pub struct sup;

        /// Represents a date, time, duration, or other specific temporal value.
        ///
        /// HTML tag: `<time>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/time)
        pub struct time;

        /// Marks text with a non-textual annotation, typically rendered with an underline.
        ///
        /// HTML tag: `<u>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/u)
        pub struct u;

        /// Represents a variable name in mathematics or programming.
        ///
        /// HTML tag: `<var>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/var)
        pub struct var;

        /// Marks a position where the browser may optionally break a line.
        ///
        /// HTML tag: `<wbr>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/wbr)
        pub struct wbr;

        /// Defines a clickable region within an image map.
        ///
        /// HTML tag: `<area>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area)
        pub struct area;

        /// Embeds sound content or a streamable audio source.
        ///
        /// HTML tag: `<audio>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio)
        pub struct audio;

        /// Embeds an image in the document.
        ///
        /// HTML tag: `<img>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img)
        pub struct img;

        /// Defines an image map containing clickable areas.
        ///
        /// HTML tag: `<map>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/map)
        pub struct map;

        /// Adds timed text or time-based data, such as subtitles, to media.
        ///
        /// HTML tag: `<track>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track)
        pub struct track;

        /// Embeds a media player for video and, when appropriate, audio.
        ///
        /// HTML tag: `<video>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video)
        pub struct video;

        /// Inserts externally provided content at a specific point in the document.
        ///
        /// HTML tag: `<embed>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed)
        pub struct embed;

        /// Creates a privacy-focused nested browsing context similar to an iframe.
        ///
        /// HTML tag: `<fencedframe>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/fencedframe)
        pub struct fencedframe;

        /// Embeds another HTML page as a nested browsing context.
        ///
        /// HTML tag: `<iframe>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe)
        pub struct iframe;

        /// Embeds an external resource handled as an image, browsing context, or plugin resource.
        ///
        /// HTML tag: `<object>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object)
        pub struct object;

        /// Offers alternate image sources for different display or device conditions.
        ///
        /// HTML tag: `<picture>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/picture)
        pub struct picture;

        /// Provides an alternative media resource for picture, audio, or video.
        ///
        /// HTML tag: `<source>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source)
        pub struct source;

        /// Establishes an SVG coordinate system and viewport or embeds an SVG fragment.
        ///
        /// HTML tag: `<svg>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/svg)
        pub struct svg;

        /// Contains the top-level content of a MathML expression.
        ///
        /// HTML tag: `<math>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/math)
        pub struct math;

        /// Provides a drawable bitmap surface controlled through scripting APIs.
        ///
        /// HTML tag: `<canvas>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas)
        pub struct canvas;

        /// Supplies fallback HTML when scripting is disabled or unsupported.
        ///
        /// HTML tag: `<noscript>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/noscript)
        pub struct noscript;

        /// Embeds executable code or data, or references it from an external resource.
        ///
        /// HTML tag: `<script>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/script)
        pub struct script;

        /// Marks a range of content removed from the document.
        ///
        /// HTML tag: `<del>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del)
        pub struct del;

        /// Marks a range of content added to the document.
        ///
        /// HTML tag: `<ins>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ins)
        pub struct ins;

        /// Provides the title or caption of a table.
        ///
        /// HTML tag: `<caption>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/caption)
        pub struct caption;

        /// Describes one or more columns within a table column group.
        ///
        /// HTML tag: `<col>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/col)
        pub struct col;

        /// Groups columns in a table for shared structure or styling.
        ///
        /// HTML tag: `<colgroup>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/colgroup)
        pub struct colgroup;

        /// Represents data arranged in rows and columns.
        ///
        /// HTML tag: `<table>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/table)
        pub struct table;

        /// Groups table rows that form the main body of table data.
        ///
        /// HTML tag: `<tbody>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tbody)
        pub struct tbody;

        /// Defines a data cell within a table row.
        ///
        /// HTML tag: `<td>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/td)
        pub struct td;

        /// Groups table rows that summarize or conclude table columns.
        ///
        /// HTML tag: `<tfoot>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tfoot)
        pub struct tfoot;

        /// Defines a header cell for a row, column, or group of table cells.
        ///
        /// HTML tag: `<th>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/th)
        pub struct th;

        /// Groups table rows that provide column-heading information.
        ///
        /// HTML tag: `<thead>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/thead)
        pub struct thead;

        /// Defines a row containing table data or header cells.
        ///
        /// HTML tag: `<tr>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/tr)
        pub struct tr;

        /// Creates an interactive control that performs an action when activated.
        ///
        /// HTML tag: `<button>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button)
        pub struct button;

        /// Provides suggested or permitted options for another form control.
        ///
        /// HTML tag: `<datalist>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/datalist)
        pub struct datalist;

        /// Groups related form controls and their labels.
        ///
        /// HTML tag: `<fieldset>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/fieldset)
        pub struct fieldset;

        /// Contains interactive controls for collecting and submitting information.
        ///
        /// HTML tag: `<form>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form)
        pub struct form;

        /// Creates a configurable form control for receiving user data.
        ///
        /// HTML tag: `<input>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input)
        pub struct input;

        /// Provides a caption associated with a user-interface control.
        ///
        /// HTML tag: `<label>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/label)
        pub struct label;

        /// Captions the contents of a fieldset.
        ///
        /// HTML tag: `<legend>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/legend)
        pub struct legend;

        /// Displays a scalar or fractional value within a known range.
        ///
        /// HTML tag: `<meter>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter)
        pub struct meter;

        /// Groups related options inside a select control.
        ///
        /// HTML tag: `<optgroup>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/optgroup)
        pub struct optgroup;

        /// Defines a selectable item in a select, optgroup, or datalist.
        ///
        /// HTML tag: `<option>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option)
        pub struct option;

        /// Holds the result of a calculation or user-triggered operation.
        ///
        /// HTML tag: `<output>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/output)
        pub struct output;

        /// Shows how much of a task has been completed.
        ///
        /// HTML tag: `<progress>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/progress)
        pub struct progress;

        /// Creates a control that lets the user choose from a menu of options.
        ///
        /// HTML tag: `<select>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/select)
        pub struct select;

        /// Displays the selected option’s content inside a closed customizable select.
        ///
        /// HTML tag: `<selectedcontent>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/selectedcontent)
        pub struct selectedcontent;

        /// Provides a multi-line plain-text editing control.
        ///
        /// HTML tag: `<textarea>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea)
        pub struct textarea;

        /// Creates a disclosure widget whose additional information can be expanded or collapsed.
        ///
        /// HTML tag: `<details>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details)
        pub struct details;

        /// Represents a dialog, alert, inspector, or other temporary interactive surface.
        ///
        /// HTML tag: `<dialog>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog)
        pub struct dialog;

        /// Creates a control through which the user may share location data with the page.
        ///
        /// HTML tag: `<geolocation>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/geolocation)
        pub struct geolocation;

        /// Provides the visible label that toggles a parent details disclosure.
        ///
        /// HTML tag: `<summary>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/summary)
        pub struct summary;

        /// Defines a placeholder where a web component receives assigned markup.
        ///
        /// HTML tag: `<slot>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/slot)
        pub struct slot;

        /// Stores inert HTML that scripts may instantiate later.
        ///
        /// HTML tag: `<template>`
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/template)
        pub struct template;
    }

    pub mod attrs {
        /// Lists the media types or file types that a server or file input is prepared to receive.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/accept)
        pub struct accept;

        /// Provides one or more candidate keyboard shortcuts for focusing or activating the element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/accesskey)
        pub struct accesskey;

        /// Gives the URL that receives and processes a submitted form.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/action)
        pub struct action;

        /// Defines the permissions policy applied to an embedded iframe.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/allow)
        pub struct allow;

        /// Lets a color input expose an alpha-channel or opacity control.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/alpha)
        pub struct alpha;

        /// Supplies a textual replacement for an image or image-based control.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/alt)
        pub struct alt;

        /// Controls automatic capitalization behavior for user-entered text.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/autocapitalize)
        pub struct autocapitalize;

        /// Tells the browser whether and how it may automatically complete form values.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete)
        pub struct autocomplete;

        /// Controls automatic correction of spelling errors in editable text.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/autocorrect)
        pub struct autocorrect;

        /// Requests focus when the page loads or when an enclosing dialog opens.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/autofocus)
        pub struct autofocus;

        /// Requests that audio or video begin playback automatically when permitted.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autoplay)
        pub struct autoplay;

        /// Hints that a newly captured camera or microphone file should be offered by a file input.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/capture)
        pub struct capture;

        /// Declares the document character encoding on a meta element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/charset)
        pub struct charset;

        /// Sets the initial selected state of a checkbox or radio input.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/checked)
        pub struct checked;

        /// Links a quotation or document edit to a source URL.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/cite)
        pub struct cite;

        /// Assigns one or more class names used by CSS and scripts to select the element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/class)
        pub struct class;

        /// Selects the color space used by a color input.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/colorspace)
        pub struct colorspace;

        /// Sets the visible width of a textarea in average character columns.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/cols)
        pub struct cols;

        /// Specifies how many table columns a cell spans.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/colspan)
        pub struct colspan;

        /// Provides the metadata value represented by a meta element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/content)
        pub struct content;

        /// Controls whether the user may edit the element's contents.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/contenteditable)
        pub struct contenteditable;

        /// Requests built-in playback controls for audio or video.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/controls)
        pub struct controls;

        /// Defines the coordinates of a clickable region in an image map.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/coords)
        pub struct coords;

        /// Configures whether a resource fetch uses Cross-Origin Resource Sharing.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
        pub struct crossorigin;

        /// Specifies a Content Security Policy that an embedded document must enforce.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/csp)
        pub struct csp;

        /// Provides the URL of the resource loaded by an object element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/data)
        pub struct data;

        /// Associates a machine-readable date or time with an edit or time element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/datetime)
        pub struct datetime;

        /// Hints whether image decoding should occur synchronously, asynchronously, or automatically.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/decoding)
        pub struct decoding;

        /// Marks a text track as the default track when user preferences do not select another.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/default)
        pub struct default;

        /// Delays execution of an external classic script until document parsing has completed.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/defer)
        pub struct defer;

        /// Sets or automatically determines the direction of the element's text.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/dir)
        pub struct dir;

        /// Adds the directionality of a submitted text control to the form data.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/dirname)
        pub struct dirname;

        /// Makes a supported form control unavailable for interaction and form submission.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/disabled)
        pub struct disabled;

        /// Indicates that following a hyperlink should download its target, optionally with a suggested filename.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/download)
        pub struct download;

        /// Controls whether the element may be dragged with the HTML drag-and-drop mechanism.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/draggable)
        pub struct draggable;

        /// Registers the element for Element Timing performance observations under a chosen identifier.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/elementtiming)
        pub struct elementtiming;

        /// Selects the encoding used for form data sent with a POST submission.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/enctype)
        pub struct enctype;

        /// Hints the action label or icon shown on a virtual keyboard's enter key.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/enterkeyhint)
        pub struct enterkeyhint;

        /// Re-exports named shadow parts from a nested shadow tree to an outer tree.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/exportparts)
        pub struct exportparts;

        /// Hints that a resource fetch should receive high, low, or automatic priority.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/fetchpriority)
        pub struct fetchpriority;

        /// Associates a form-related element with a form elsewhere in the same document.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
        pub struct form;

        /// Overrides the owning form's submission URL for a submit button.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/formaction)
        pub struct formaction;

        /// Overrides the owning form's submission encoding for a submit button.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/formenctype)
        pub struct formenctype;

        /// Overrides the owning form's HTTP submission method for a submit button.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/formmethod)
        pub struct formmethod;

        /// Skips constraint validation when a particular submit button submits its form.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/formnovalidate)
        pub struct formnovalidate;

        /// Overrides the browsing context used to display a submit button's form response.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/formtarget)
        pub struct formtarget;

        /// Lists the table header cells that describe a data or header cell.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/headers)
        pub struct headers;

        /// Sets the intrinsic rendered height of supported replaced or embedded elements.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/height)
        pub struct height;

        /// Marks the element as not currently relevant so it is normally not rendered.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/hidden)
        pub struct hidden;

        /// Sets the boundary above which a meter value is considered in the high range.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/high)
        pub struct high;

        /// Provides the URL of a linked resource or hyperlink destination.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/href)
        pub struct href;

        /// Hints the human language of a linked resource.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/hreflang)
        pub struct hreflang;

        /// Assigns a document-wide unique identifier to the element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/id)
        pub struct id;

        /// Makes an element and its descendants non-interactive and unavailable to focus or selection.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/inert)
        pub struct inert;

        /// Hints the type of virtual keyboard most suitable for editing the element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/inputmode)
        pub struct inputmode;

        /// Provides cryptographic hashes used to verify that a fetched resource has not been altered.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/integrity)
        pub struct integrity;

        /// Requests that a standard element behave as a registered customized built-in element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/is)
        pub struct is;

        /// Marks an image as part of a server-side image map.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/ismap)
        pub struct ismap;

        /// Provides the globally unique identifier of a microdata item.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/itemid)
        pub struct itemid;

        /// Adds one or more named properties to a microdata item.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/itemprop)
        pub struct itemprop;

        /// Associates additional, non-descendant properties with a microdata item by element ID.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/itemref)
        pub struct itemref;

        /// Creates a microdata item and establishes the scope of its properties.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/itemscope)
        pub struct itemscope;

        /// Identifies the vocabulary URL that defines a microdata item's type and properties.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/itemtype)
        pub struct itemtype;

        /// Identifies the purpose of a media text track, such as subtitles or captions.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/kind)
        pub struct kind;

        /// Provides a user-visible label for an option group, option, or text track.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/label)
        pub struct label;

        /// Declares the language of the element's content or expected user input.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/lang)
        pub struct lang;

        /// Connects an input to a datalist that supplies suggested values.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/list)
        pub struct list;

        /// Hints whether an image or iframe should load immediately or lazily.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/loading)
        pub struct loading;

        /// Sets the boundary below which a meter value is considered in the low range.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/low)
        pub struct low;

        /// Sets the greatest permitted or represented numeric, date, time, or progress value.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/max)
        pub struct max;

        /// Limits the maximum number of characters accepted by a text control.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/maxlength)
        pub struct maxlength;

        /// Describes the media conditions for which a linked resource or source is intended.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/media)
        pub struct media;

        /// Selects the HTTP method used to submit a form.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/method)
        pub struct method;

        /// Sets the smallest permitted or represented numeric, date, or time value.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/min)
        pub struct min;

        /// Sets the minimum number of characters required by a text control.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/minlength)
        pub struct minlength;

        /// Allows a control to accept or select more than one value.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/multiple)
        pub struct multiple;

        /// Sets the initial muted state of audio or video.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/muted)
        pub struct muted;

        /// Assigns a name used for form submission, lookup, grouping, or element-specific identification.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/name)
        pub struct name;

        /// Supplies a one-time cryptographic token used by Content Security Policy.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/nonce)
        pub struct nonce;

        /// Disables constraint validation when the form is submitted.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/novalidate)
        pub struct novalidate;

        /// Sets the initial open state of a details disclosure or dialog.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/open)
        pub struct open;

        /// Identifies the preferred value within a meter's range.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/optimum)
        pub struct optimum;

        /// Assigns one or more shadow-part names that may be styled through the part pseudo-element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/part)
        pub struct part;

        /// Provides a regular expression that a supported input value must match.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/pattern)
        pub struct pattern;

        /// Lists URLs that should receive hyperlink-following audit notifications.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/ping)
        pub struct ping;

        /// Displays a short hint inside an empty text-entry control.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/placeholder)
        pub struct placeholder;

        /// Requests inline video playback rather than forced full-screen playback.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/playsinline)
        pub struct playsinline;

        /// Turns the element into a popover that can be shown and hidden through invokers or script.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/popover)
        pub struct popover;

        /// Provides an image displayed before video playback begins or a frame becomes available.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/poster)
        pub struct poster;

        /// Hints how much audio or video data the browser should fetch before playback.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/preload)
        pub struct preload;

        /// Prevents the user from editing a supported form control while retaining its submitted value.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/readonly)
        pub struct readonly;

        /// Selects the referrer information sent when fetching or navigating to a resource.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/referrerpolicy)
        pub struct referrerpolicy;

        /// Describes the relationship between the current document and a linked resource or destination.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel)
        pub struct rel;

        /// Requires the user to provide or select a value before form submission.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/required)
        pub struct required;

        /// Displays an ordered list with descending item numbers.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/reversed)
        pub struct reversed;

        /// Assigns an explicit ARIA role that defines the element's accessibility semantics.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles)
        pub struct role;

        /// Sets the visible height of a textarea in text rows.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rows)
        pub struct rows;

        /// Specifies how many table rows a cell spans.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rowspan)
        pub struct rowspan;

        /// Applies a configurable set of restrictions to an embedded iframe document.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/sandbox)
        pub struct sandbox;

        /// Identifies the table cells for which a header cell provides heading information.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/scope)
        pub struct scope;

        /// Sets the initial selected state of an option.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/selected)
        pub struct selected;

        /// Selects the geometric shape used for an image-map area.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/shape)
        pub struct shape;

        /// Sets the visible character width of an input or the visible option count of a select.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/size)
        pub struct size;

        /// Describes responsive image slot sizes or icon dimensions for linked resources.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/sizes)
        pub struct sizes;

        /// Assigns an element to a named slot in a shadow tree.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/slot)
        pub struct slot;

        /// Sets how many table columns are covered by a col or colgroup.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/span)
        pub struct span;

        /// Controls whether spelling and grammar checking may be offered for editable text.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/spellcheck)
        pub struct spellcheck;

        /// Provides the URL of embedded media, script, image, frame, or other external content.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/src)
        pub struct src;

        /// Provides inline HTML content for an iframe instead of loading a separate URL.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/srcdoc)
        pub struct srcdoc;

        /// Declares the language of a media text track.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/srclang)
        pub struct srclang;

        /// Lists responsive image candidates and their width or pixel-density descriptors.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/srcset)
        pub struct srcset;

        /// Sets the first ordinal value used by an ordered list.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/start)
        pub struct start;

        /// Sets the permitted value interval or granularity for a supported input.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/step)
        pub struct step;

        /// Contains inline CSS declarations applied directly to the element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/style)
        pub struct style;

        /// Controls keyboard focusability and the element's position in sequential focus navigation.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/tabindex)
        pub struct tabindex;

        /// Selects the browsing context used for a link destination or form response.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/target)
        pub struct target;

        /// Provides advisory information that user agents may present, often as a tooltip.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/title)
        pub struct title;

        /// Controls whether the element's translatable content should be localized.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/translate)
        pub struct translate;

        /// Associates an image or object with a client-side image map.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/usemap)
        pub struct usemap;

        /// Provides an element-specific value, initial value, or submitted value.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/value)
        pub struct value;

        /// Controls whether focusing editable content automatically shows the virtual keyboard.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/virtualkeyboardpolicy)
        pub struct virtualkeyboardpolicy;

        /// Sets the intrinsic rendered width of supported replaced or embedded elements.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/width)
        pub struct width;

        /// Controls how textarea text is wrapped and represented during form submission.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/wrap)
        pub struct wrap;

        /// Controls whether browser-provided writing suggestions are enabled for the element.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/writingsuggestions)
        pub struct writingsuggestions;

        /// Runs inline handler code when the `abort` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onabort;

        /// Runs inline handler code when the `animationcancel` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onanimationcancel;

        /// Runs inline handler code when the `animationend` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onanimationend;

        /// Runs inline handler code when the `animationiteration` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onanimationiteration;

        /// Runs inline handler code when the `animationstart` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onanimationstart;

        /// Runs inline handler code when the `auxclick` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onauxclick;

        /// Runs inline handler code when the `beforeinput` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onbeforeinput;

        /// Runs inline handler code when the `beforematch` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onbeforematch;

        /// Runs inline handler code when the `beforetoggle` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onbeforetoggle;

        /// Runs inline handler code when the `blur` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onblur;

        /// Runs inline handler code when the `cancel` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncancel;

        /// Runs inline handler code when the `canplay` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncanplay;

        /// Runs inline handler code when the `canplaythrough` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncanplaythrough;

        /// Runs inline handler code when the `change` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onchange;

        /// Runs inline handler code when the `click` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onclick;

        /// Runs inline handler code when the `close` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onclose;

        /// Runs inline handler code when the `command` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncommand;

        /// Runs inline handler code when the `contentvisibilityautostatechange` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncontentvisibilityautostatechange;

        /// Runs inline handler code when the `contextlost` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncontextlost;

        /// Runs inline handler code when the `contextmenu` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncontextmenu;

        /// Runs inline handler code when the `contextrestored` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncontextrestored;

        /// Runs inline handler code when the `copy` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncopy;

        /// Runs inline handler code when the `cuechange` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncuechange;

        /// Runs inline handler code when the `cut` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oncut;

        /// Runs inline handler code when the `dblclick` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ondblclick;

        /// Runs inline handler code when the `drag` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ondrag;

        /// Runs inline handler code when the `dragend` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ondragend;

        /// Runs inline handler code when the `dragenter` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ondragenter;

        /// Runs inline handler code when the `dragleave` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ondragleave;

        /// Runs inline handler code when the `dragover` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ondragover;

        /// Runs inline handler code when the `dragstart` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ondragstart;

        /// Runs inline handler code when the `drop` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ondrop;

        /// Runs inline handler code when the `durationchange` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ondurationchange;

        /// Runs inline handler code when the `emptied` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onemptied;

        /// Runs inline handler code when the `ended` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onended;

        /// Runs inline handler code when the `error` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onerror;

        /// Runs inline handler code when the `focus` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onfocus;

        /// Runs inline handler code when the `focusin` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onfocusin;

        /// Runs inline handler code when the `focusout` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onfocusout;

        /// Runs inline handler code when the `formdata` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onformdata;

        /// Runs inline handler code when the `fullscreenchange` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onfullscreenchange;

        /// Runs inline handler code when the `fullscreenerror` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onfullscreenerror;

        /// Runs inline handler code when the `gotpointercapture` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ongotpointercapture;

        /// Runs inline handler code when the `input` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oninput;

        /// Runs inline handler code when the `invalid` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct oninvalid;

        /// Runs inline handler code when the `keydown` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onkeydown;

        /// Runs inline handler code when the `keyup` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onkeyup;

        /// Runs inline handler code when the `load` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onload;

        /// Runs inline handler code when the `loadeddata` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onloadeddata;

        /// Runs inline handler code when the `loadedmetadata` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onloadedmetadata;

        /// Runs inline handler code when the `loadstart` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onloadstart;

        /// Runs inline handler code when the `lostpointercapture` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onlostpointercapture;

        /// Runs inline handler code when the `mousedown` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onmousedown;

        /// Runs inline handler code when the `mouseenter` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onmouseenter;

        /// Runs inline handler code when the `mouseleave` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onmouseleave;

        /// Runs inline handler code when the `mousemove` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onmousemove;

        /// Runs inline handler code when the `mouseout` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onmouseout;

        /// Runs inline handler code when the `mouseover` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onmouseover;

        /// Runs inline handler code when the `mouseup` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onmouseup;

        /// Runs inline handler code when the `paste` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpaste;

        /// Runs inline handler code when the `pause` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpause;

        /// Runs inline handler code when the `play` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onplay;

        /// Runs inline handler code when the `playing` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onplaying;

        /// Runs inline handler code when the `pointercancel` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpointercancel;

        /// Runs inline handler code when the `pointerdown` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpointerdown;

        /// Runs inline handler code when the `pointerenter` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpointerenter;

        /// Runs inline handler code when the `pointerleave` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpointerleave;

        /// Runs inline handler code when the `pointermove` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpointermove;

        /// Runs inline handler code when the `pointerout` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpointerout;

        /// Runs inline handler code when the `pointerover` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpointerover;

        /// Runs inline handler code when the `pointerrawupdate` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpointerrawupdate;

        /// Runs inline handler code when the `pointerup` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onpointerup;

        /// Runs inline handler code when the `progress` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onprogress;

        /// Runs inline handler code when the `ratechange` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onratechange;

        /// Runs inline handler code when the `reset` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onreset;

        /// Runs inline handler code when the `resize` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onresize;

        /// Runs inline handler code when the `scroll` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onscroll;

        /// Runs inline handler code when the `scrollend` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onscrollend;

        /// Runs inline handler code when the `scrollsnapchange` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onscrollsnapchange;

        /// Runs inline handler code when the `scrollsnapchanging` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onscrollsnapchanging;

        /// Runs inline handler code when the `securitypolicyviolation` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onsecuritypolicyviolation;

        /// Runs inline handler code when the `seeked` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onseeked;

        /// Runs inline handler code when the `seeking` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onseeking;

        /// Runs inline handler code when the `select` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onselect;

        /// Runs inline handler code when the `selectionchange` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onselectionchange;

        /// Runs inline handler code when the `selectstart` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onselectstart;

        /// Runs inline handler code when the `slotchange` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onslotchange;

        /// Runs inline handler code when the `stalled` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onstalled;

        /// Runs inline handler code when the `submit` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onsubmit;

        /// Runs inline handler code when the `suspend` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onsuspend;

        /// Runs inline handler code when the `timeupdate` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ontimeupdate;

        /// Runs inline handler code when the `toggle` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ontoggle;

        /// Runs inline handler code when the `touchcancel` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ontouchcancel;

        /// Runs inline handler code when the `touchend` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ontouchend;

        /// Runs inline handler code when the `touchmove` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ontouchmove;

        /// Runs inline handler code when the `touchstart` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ontouchstart;

        /// Runs inline handler code when the `transitioncancel` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ontransitioncancel;

        /// Runs inline handler code when the `transitionend` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ontransitionend;

        /// Runs inline handler code when the `transitionrun` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ontransitionrun;

        /// Runs inline handler code when the `transitionstart` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct ontransitionstart;

        /// Runs inline handler code when the `volumechange` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onvolumechange;

        /// Runs inline handler code when the `waiting` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onwaiting;

        /// Runs inline handler code when the `wheel` event is dispatched.
        ///
        /// [MDN reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes#list_of_global_event_handler_attributes)
        pub struct onwheel;
    }
}
