/// Creates a lazily rendered HTML value from ASX syntax.
///
/// ASX builds lazy [`Html`](crate::Html) values from element calls, component
/// calls, text, attributes, expressions, and Rust-like control flow.
///
/// # A small example
///
/// Elements use braces for children and square brackets for attributes:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// article[id: "welcome", class: "card"] {
///     h1 { "Hello" }
///     p { "Welcome to Avosetta." }
/// }
/// # }
/// ```
///
/// Built-in HTML, SVG, and MathML elements are available by name. A qualified
/// path calls a component function instead:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// main { ui::ProfileCard { "Profile content" } }
/// # }
/// ```
///
/// # Elements with and without children
///
/// These four shapes are available for element and component paths:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// div { "children" }
/// div[id: "content"] { "children and attributes" }
/// br;
/// input[type: "email"];
/// # }
/// ```
///
/// A semicolon means that no children are supplied. The called element still
/// decides how that is rendered: `br;` renders `<br>`, while `div;` renders an
/// ordinary element with an empty body.
///
/// String-literal element names support custom elements without requiring a
/// predefined function:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// "user-card"[theme: "dark"] { "Custom element body" }
/// "custom-void";
/// # }
/// ```
///
/// A string literal followed immediately by `{`, `[`, or `;` is an element
/// name. A literal in any other child position is text.
///
/// # Text and expression injection
///
/// Bare literals are escaped and emitted as compile-time static text. Adjacent
/// static literals and markup can be combined by the HTML representation:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// "There are "
/// 3
/// " messages."
/// # }
/// ```
///
/// Use `@(...)` or `@{...}` to inject a Rust expression implementing
/// [`Html`](crate::Html):
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// "Hello, "
/// @{user.name}
/// @(format_args!("You have {} messages.", count))
/// # }
/// ```
///
/// A single literal inside `@{...}` is treated as static text too:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// @{"static text"}
/// @{42}
/// # }
/// ```
///
/// Other expressions remain lazy. They are evaluated when the resulting HTML
/// value is written, so they may move or capture values from the local scope.
///
/// # Attribute names and values
///
/// Identifier names cover normal attributes. Quoted names cover attributes
/// that are not Rust identifiers:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// button[
///     id: "save",
///     {"data-action"}: "save",
///     {"aria-label"}: "Save document"
/// ] { "Save" }
/// # }
/// ```
///
/// String literals are static attribute values. Wrap other values and general
/// Rust expressions in braces:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// input[
///     value: {current_value},
///     tabindex: {tab_index},
///     checked: {is_checked}
/// ];
/// # }
/// ```
///
/// Attribute values that report themselves as absent omit the complete
/// name/value pair. This is useful with `Option`, `false`, and `()`:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// input[
///     value: {Some("present")},
///     title: {None::<&str>},
///     disabled: {false}
/// ];
/// # }
/// ```
///
/// The example above emits `value`, but omits both `title` and `disabled`.
///
/// # Class and style merging
///
/// `class` and `style` are special accumulators. Every class source is merged
/// into one attribute with a single space between present values. Every style
/// source is merged into one attribute, and each present declaration receives
/// a trailing semicolon:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// div[
///     class: "card",
///     class: {size_class},
///     class: {is_selected.then_some("selected")},
///     style: "display: block",
///     style: {color_style}
/// ] {}
/// # }
/// ```
///
/// With `size_class = "large"`, `is_selected = true`, and
/// `color_style = "color: red"`, this produces one `class` and one `style`
/// attribute: `<div class="card large selected"
/// style="display: block;color: red;"></div>`.
///
/// Class and style values that report themselves as absent contribute nothing,
/// including their separator. The final class and style attributes are emitted
/// before ordinary attributes, regardless of where their inputs appeared in
/// the source list.
///
/// Other attributes are deliberately not deduplicated or merged. Repeating an
/// ordinary attribute emits it repeatedly and preserves its relative order:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// div[id: "first", id: "second", {"data-tag"}: "a", {"data-tag"}: "b"] {}
/// # }
/// ```
///
/// This produces both `id` attributes and both `data-tag` attributes. Avosetta
/// does not choose a winner for ordinary attributes.
///
/// # Attribute projection
///
/// `..attrs` projects an [`Attrs`](crate::Attrs) value into another element or
/// component. Projection must be the final entry in an attribute list:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// section[class: "panel", style: "padding: 1rem", ..attrs] {
///     @{children}
/// }
/// # }
/// ```
///
/// Projected classes join the existing class accumulator. Projected styles join
/// the existing style accumulator. Projected ordinary attributes are appended
/// and remain duplicated if the destination already contains the same name.
/// This makes forwarding transparent rather than giving projected or local
/// attributes implicit precedence.
///
/// A component that accepts [`Attrs`](crate::Attrs) can therefore forward them
/// while adding its own class or style:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// ui::Panel[class: "raised", id: "settings"] {
///     "Settings"
/// }
/// # }
/// ```
///
/// # Component properties
///
/// A dot-prefixed name initializes a field on the component properties value.
/// Properties begin with `Default::default()`, then each supplied field is
/// assigned in source order:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// ui::Button[
///     .kind: {Kind::Primary},
///     .disabled: false,
///     id: "save"
/// ] { "Save" }
/// # }
/// ```
///
/// Property literals may be written directly. General property expressions use
/// braces. Dot-prefixed properties are not HTML attributes and are never
/// rendered automatically.
///
/// Component functions are ordinary functions whose parameters use any
/// supported combination and order of [`Props`](crate::Props),
/// [`Attrs`](crate::Attrs), and [`Children`](crate::Children). A component may
/// omit `Attrs` to reject arbitrary attributes, or omit `Children` to reject a
/// body. Props-only calls and calls with no attributes pass `()` as the
/// attribute input.
///
/// # Escaping and trusted HTML
///
/// Static text, strings, characters, formatting arguments, and attribute values
/// escape ampersands, angle brackets, quotation marks, and apostrophes:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// p[title: {user_input}] { @{user_input} }
/// # }
/// ```
///
/// Use [`Raw`](crate::Raw) only for trusted content that should bypass escaping:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// "Escaped: " @{html_source}
/// " Raw: " @{Raw(trusted_html)}
/// # }
/// ```
///
/// `Raw` also bypasses escaping inside an attribute, so wrapping untrusted input
/// can break the surrounding markup.
///
/// # Conditional content
///
/// `@if` supports an optional `else` body. Both bodies contain ASX syntax:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// @if user.is_admin() {
///     strong { "Administrator" }
/// } else {
///     "Member"
/// }
/// # }
/// ```
///
/// # Iteration
///
/// `@for`, `@while`, and `@loop` mirror their Rust counterparts:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// ul {
///     @for item in items {
///         li { @{item} }
///     }
/// }
/// # }
/// ```
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// @while cursor.has_more() {
///     @{cursor.next()}
/// }
///
/// @loop {
///     @({
///         if finished() {
///             break;
///         }
///     })
/// }
/// # }
/// ```
///
/// Rust statements, including `break`, can be placed inside an injected block.
///
/// # Local bindings
///
/// `@let` introduces a Rust binding that is available to all following syntax
/// in the same body:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// @let heading = format!("Hello, {}", user.name);
/// h1 { @{heading} }
/// # }
/// ```
///
/// The statement ends at the semicolon, just like a Rust `let` statement.
///
/// # Pattern matching
///
/// `@match` accepts Rust patterns and optional guards. Each arm body must be an
/// ASX block:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// @match status {
///     Status::Ready(item) if item.is_featured() => {
///         strong { @{item.name} }
///     },
///     Status::Ready(item) => {
///         @{item.name}
///     },
///     Status::Loading => {
///         "Loading..."
///     },
/// }
/// # }
/// ```
#[macro_export]
macro_rules! asx {
    ($($tt:tt)*) => {{
        #[allow(unused_imports)]
        use $crate::elements::*;

        #[inline(always)]
        const fn __coerce<T: $crate::Html>(x: T) -> impl $crate::Html { x }

        __coerce($crate::FnHtml(
            move || $crate::__asx_expand!($($tt)*),
            ::core::marker::PhantomData,
        ))
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __asx_autocomplete_element {
    ($ident:ident) => {
        #[cfg(any())]
        use $crate::elements::$ident;
    };

    ($($tt:tt)*) => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __asx_if {
    ([$($condition:tt)*] { $($body:tt)* } else { $($else_body:tt)* } $($rest:tt)*) => {
        $crate::Chain(
            $crate::WriteHtml(move |s: &mut ::std::string::String| {
                if $($condition)* {
                    $crate::Html::write($crate::__asx_expand!($($body)*), s);
                } else {
                    $crate::Html::write($crate::__asx_expand!($($else_body)*), s);
                }
            }),
            $crate::__asx_expand!($($rest)*),
        )
    };

    ([$($condition:tt)*] { $($body:tt)* } $($rest:tt)*) => {
        $crate::Chain(
            $crate::WriteHtml(move |s: &mut ::std::string::String| {
                if $($condition)* {
                    $crate::Html::write($crate::__asx_expand!($($body)*), s);
                }
            }),
            $crate::__asx_expand!($($rest)*),
        )
    };

    ([$($condition:tt)*] $token:tt $($rest:tt)*) => {
        $crate::__asx_if!([$($condition)* $token] $($rest)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __asx_for {
    ([$($header:tt)*] { $($body:tt)* } $($rest:tt)*) => {
        $crate::Chain(
            $crate::WriteHtml(move |s: &mut ::std::string::String| {
                for $($header)* {
                    $crate::Html::write($crate::__asx_expand!($($body)*), s);
                }
            }),
            $crate::__asx_expand!($($rest)*),
        )
    };

    ([$($header:tt)*] $token:tt $($rest:tt)*) => {
        $crate::__asx_for!([$($header)* $token] $($rest)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __asx_while {
    ([$($condition:tt)*] { $($body:tt)* } $($rest:tt)*) => {
        $crate::Chain(
            $crate::WriteHtml(move |s: &mut ::std::string::String| {
                while $($condition)* {
                    $crate::Html::write($crate::__asx_expand!($($body)*), s);
                }
            }),
            $crate::__asx_expand!($($rest)*),
        )
    };

    ([$($condition:tt)*] $token:tt $($rest:tt)*) => {
        $crate::__asx_while!([$($condition)* $token] $($rest)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __asx_let {
    ([$($statement:tt)*] ; $($rest:tt)*) => {{
        let $($statement)*;

        $crate::__asx_expand!($($rest)*)
    }};

    ([$($statement:tt)*] $token:tt $($rest:tt)*) => {
        $crate::__asx_let!([$($statement)* $token] $($rest)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __asx_match {
    ([$($value:tt)*] { $($arms:tt)* } $($rest:tt)*) => {
        $crate::Chain(
            $crate::WriteHtml(move |s: &mut ::std::string::String| {
                $crate::__asx_match_write!(s, [$($value)*], { $($arms)* });
            }),
            $crate::__asx_expand!($($rest)*),
        )
    };

    ([$($value:tt)*] $token:tt $($rest:tt)*) => {
        $crate::__asx_match!([$($value)* $token] $($rest)*)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __asx_match_write {
    (
        $s:ident,
        [$($value:tt)*],
        {
            $(
                $pattern:pat $(if $guard:expr)? => { $($body:tt)* }
            ),* $(,)?
        }
    ) => {
        match $($value)* {
            $(
                $pattern $(if $guard)? => {
                    $crate::Html::write($crate::__asx_expand!($($body)*), $s);
                }
            ),*
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __asx_expand {
    () => {
        ()
    };

    (@($expr:expr) $($rest:tt)*) => {
        $crate::Chain(
            $expr,
            $crate::__asx_expand!($($rest)*),
        )
    };

    (@{$literal:literal} $($rest:tt)*) => {
        $crate::Chain(
            $crate::__static_text!(::core::concat!($literal)),
            $crate::__asx_expand!($($rest)*),
        )
    };

    (@{$expr:expr} $($rest:tt)*) => {
        $crate::__asx_expand! {
            @($expr)
            $($rest)*
        }
    };

    (@if $($tokens:tt)*) => {
        $crate::__asx_if!([] $($tokens)*)
    };

    (@for $($tokens:tt)*) => {
        $crate::__asx_for!([] $($tokens)*)
    };

    (@while $($tokens:tt)*) => {
        $crate::__asx_while!([] $($tokens)*)
    };

    (@loop { $($body:tt)* } $($rest:tt)*) => {
        $crate::Chain(
            $crate::WriteHtml(move |s: &mut ::std::string::String| {
                loop {
                    $crate::Html::write($crate::__asx_expand!($($body)*), s);
                }
            }),
            $crate::__asx_expand!($($rest)*),
        )
    };

    (@let $($tokens:tt)*) => {
        $crate::__asx_let!([] $($tokens)*)
    };

    (@match $($tokens:tt)*) => {
        $crate::__asx_match!([] $($tokens)*)
    };

    (
        $element:literal[$($attrs:tt)*] { $($body:tt)* } $($rest:tt)*
    ) => {
        $crate::Chain(
            $crate::elements::Element {
                name: $crate::__static_text!(@raw $element),

                attrs: $crate::__attrs!((), { $($attrs)* }).1.0,
                children: $crate::__asx_expand! { $($body)* }
            },
            $crate::__asx_expand!($($rest)*),
        )
    };

    (
        $element:literal { $($body:tt)* } $($rest:tt)*
    ) => {
        $crate::Chain(
            $crate::elements::Element {
                name: $crate::__static_text!(@raw $element),

                attrs: (),
                children: $crate::__asx_expand! { $($body)* }
            },
            $crate::__asx_expand!($($rest)*),
        )
    };

    (
        $element:literal[$($attrs:tt)*]; $($rest:tt)*
    ) => {
        $crate::Chain(
            $crate::elements::Void {
                name: $crate::__static_text!(@raw $element),
                attrs: $crate::__attrs!((), { $($attrs)* }).1.0,
            },
            $crate::__asx_expand!($($rest)*),
        )
    };

    (
        $element:literal; $($rest:tt)*
    ) => {
        $crate::Chain(
            $crate::elements::Void {
                name: $crate::__static_text!(@raw $element),
                attrs: (),
            },
            $crate::__asx_expand!($($rest)*),
        )
    };

    ($literal:literal $($rest:tt)*) => {
        $crate::Chain(
            $crate::__static_text!(::core::concat!($literal)),
            $crate::__asx_expand!($($rest)*),
        )
    };
    (
        $path:path[$(.$name:ident: $value:tt),* $(,)?] { $($body:tt)* } $($rest:tt)*
    ) => {{
        $crate::__asx_autocomplete_element!($path);

        $crate::Chain(
            {
                let (fragment, props) = $crate::Fragment::props($path);
                let (props, _attrs) = $crate::__attrs!(
                    props,
                    { $(.$name: $value),* }
                );

                $crate::Fragment::call(
                    fragment,
                    (),
                    props.0,
                    $crate::__asx_expand! { $($body)* },
                )
            },
            $crate::__asx_expand!($($rest)*),
        )
    }};

    (
        $path:path[$(.$name:ident: $value:tt),* $(,)?]; $($rest:tt)*
    ) => {{
        $crate::__asx_autocomplete_element!($path);

        $crate::Chain(
            {
                let (fragment, props) = $crate::Fragment::props($path);
                let (props, _attrs) = $crate::__attrs!(
                    props,
                    { $(.$name: $value),* }
                );

                $crate::Fragment::call(fragment, (), props.0, ())
            },
            $crate::__asx_expand!($($rest)*),
        )
    }};

    (
        $path:path[$($attrs:tt)*] { $($body:tt)* } $($rest:tt)*
    ) => {{
        $crate::__asx_autocomplete_element!($path);

        $crate::Chain(
            {
                let (fragment, props) = $crate::Fragment::props($path);
                let (props, attrs) = $crate::__attrs!(props, { $($attrs)* });

                $crate::Fragment::call(
                    fragment,
                    attrs.0,
                    props.0,
                    $crate::__asx_expand! { $($body)* },
                )
            },
            $crate::__asx_expand!($($rest)*),
        )
    }};

    (
        $path:path { $($body:tt)* } $($rest:tt)*
    ) => {{
        $crate::__asx_autocomplete_element!($path);

        $crate::__asx_expand! {
            $path[] { $($body)* } $($rest)*
        }
    }};

    (
        $path:path[$($attrs:tt)*]; $($rest:tt)*
    ) => {{
        $crate::__asx_autocomplete_element!($path);

        $crate::Chain(
            {
                let (fragment, props) = $crate::Fragment::props($path);
                let (props, attrs) = $crate::__attrs!(props, { $($attrs)* });

                $crate::Fragment::call(fragment, attrs.0, props.0, ())
            },
            $crate::__asx_expand!($($rest)*),
        )
    }};

    (
        $path:path; $($rest:tt)*
    ) => {{
        $crate::__asx_autocomplete_element!($path);

        $crate::__asx_expand! {
            $path[];
            $($rest)*
        }
    }};


}
