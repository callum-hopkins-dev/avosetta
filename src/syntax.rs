/// Builds a lazy [`Html`](crate::Html) value from ASX markup.
///
/// ASX combines HTML-like element syntax with ordinary Rust expressions and
/// control flow. The resulting value is not rendered until it is written with
/// [`Html::write`](crate::Html::write) or
/// [`Html::to_string`](crate::Html::to_string).
///
/// # Quick start
///
/// Use braces for children and square brackets for attributes:
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
/// Unprefixed names resolve through [`elements`](crate::elements), so local
/// bindings cannot shadow built-in HTML, SVG, or MathML elements. Prefix a
/// path with `@` to call a fragment function:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// main {
///     @ui::ProfileCard { "Profile content" }
/// }
/// # }
/// ```
///
/// # Elements
///
/// Elements and `@`-prefixed fragment paths support the same four basic forms:
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
/// Braces supply a body. A semicolon omits it. Ordinary elements render an
/// empty body when none is supplied, while void elements reject braces at
/// compile time, even when the body is empty:
///
/// ```compile_fail
/// use avosetta::asx;
///
/// let _ = asx! { br {} };
/// ```
///
/// Use a string literal as the element name for custom or otherwise unknown
/// elements:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// "user-card"[theme: "dark"] { "Custom element body" }
/// "custom-void";
/// # }
/// ```
///
/// A string literal followed by `{`, `[`, or `;` is parsed as an element name.
/// In every other child position, it is parsed as text.
///
/// # Text and expressions
///
/// Bare literals become escaped static text. Adjacent static regions can be
/// joined at compile time:
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
/// Inject a Rust expression that implements [`Html`](crate::Html) with
/// `@(...)`. Use `@{...}` for a Rust block, including local statements:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// "Hello, "
/// @(user.name)
/// @{
///     let count = messages.len();
///     format!("You have {count} messages.")
/// }
/// # }
/// ```
///
/// A block returns its final expression in the usual Rust manner. A single
/// literal inside `@{...}` remains static:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// @{"static text"}
/// @{42}
/// # }
/// ```
///
/// Other expressions stay lazy and are evaluated only when the final HTML
/// value is rendered. They may therefore move or capture local values.
///
/// # Attributes
///
/// Write ordinary Rust identifiers directly. Put names that are not valid Rust
/// identifiers in braces as string literals:
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
/// An attribute name without a value is shorthand for the boolean value
/// `true`. This works with identifier names, quoted names, and dynamic name
/// expressions:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// input[required, {"data-ready"}, {dynamic_name}];
/// # }
/// ```
///
/// String literals are static attribute values. Wrap dynamic values and other
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
/// A value that is not present omits the complete name-value pair. In
/// particular, `None`, `false`, and `()` are absent:
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
/// A present boolean attribute repeats its name as its value. This emits
/// `value`, emits `checked="checked"` for a true `checked` value, and omits
/// both `title` and `disabled`.
///
/// ## Class and style
///
/// Repeated `class` values are merged into one attribute, with one space
/// between each supplied value. Repeated `style` values are also merged, and
/// every supplied declaration receives a trailing semicolon:
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
/// If `size_class` is `"large"`, `is_selected` is `true`, and `color_style`
/// is `"color: red"`, the result contains:
///
/// ```text
/// class="card large selected" style="display: block;color: red;"
/// ```
///
/// Each class and style value uses ordinary [`Html`](crate::Html) interpolation.
/// Separators are unconditional, so an empty interpolation can leave harmless
/// whitespace in `class` or an empty declaration in `style`. The final `class`
/// and `style` attributes are written before ordinary
/// attributes, regardless of their source order.
///
/// Ordinary attributes are not merged or deduplicated. Repetition is preserved
/// in source order:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// div[id: "first", id: "second", {"data-tag"}: "a", {"data-tag"}: "b"] {}
/// # }
/// ```
///
/// ## Attribute projection
///
/// Use `..attrs` to project any value implementing
/// [`Attributes`](crate::Attributes) into an element or fragment. A projection
/// must be the final entry in the list. When forwarding an [`Attrs`](crate::Attrs)
/// fragment parameter, unwrap it before projection.
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
/// Projected classes and styles join the local accumulators. Projected ordinary
/// attributes are appended and remain duplicated when the destination already
/// contains the same name. Neither source receives implicit precedence.
///
/// # Fragment functions
///
/// An `@`-prefixed path calls an ordinary Rust function. Its parameters may
/// use any supported combination and order of [`Props`](crate::Props),
/// [`Attrs`](crate::Attrs), and [`Children`](crate::Children).
///
/// Dot-prefixed entries initialize fields on the value inside `Props`. That
/// value starts at `Default::default()`, and assignments occur in source order:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// @ui::Button[
///     .kind: {Kind::Primary},
///     .disabled: false,
///     id: "save"
/// ] { "Save" }
/// # }
/// ```
///
/// A literal property value may be written directly; other expressions use
/// braces. Properties are never emitted as HTML attributes.
///
/// Omitting an `Attrs` parameter rejects HTML attributes. Omitting a `Children`
/// parameter rejects a body. These constraints are checked at compile time
/// instead of silently discarding unsupported input:
///
/// ```compile_fail
/// use avosetta::asx;
///
/// mod ui {
///     pub fn label() {}
/// }
///
/// let _ = asx! { @ui::label[id: "label"]; };
/// ```
///
/// ```compile_fail
/// use avosetta::asx;
///
/// mod ui {
///     pub fn label() {}
/// }
///
/// let _ = asx! { @ui::label { "body" } };
/// ```
///
/// An empty body `{}` is still a supplied body. Use a semicolon to omit the
/// body entirely.
///
/// # Escaping and raw HTML
///
/// Static text, strings, characters, formatting arguments, and attribute
/// values escape ampersands, angle brackets, quotation marks, and apostrophes:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// p[title: {user_input}] { @{user_input} }
/// # }
/// ```
///
/// Wrap trusted content in [`Raw`](crate::Raw) to bypass escaping:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// "Escaped: " @{html_source}
/// " Raw: " @{Raw(trusted_html)}
/// # }
/// ```
///
/// `Raw` also bypasses escaping in attributes. Never wrap untrusted input.
///
/// # Control flow
///
/// ASX control-flow forms begin with `@` and contain ASX markup in their
/// bodies.
///
/// ## Conditions
///
/// `@if` accepts an optional `else` branch:
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
/// ## Loops
///
/// `@for`, `@while`, and `@loop` follow their Rust counterparts:
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
/// Injected Rust blocks may contain statements such as `break`.
///
/// ## Local bindings
///
/// `@let` introduces a Rust binding that remains available to the following ASX
/// markup in the same body:
///
/// ```rust
/// # #[cfg(any())]
/// # asx! {
/// @let heading = format!("Hello, {}", user.name);
/// h1 { @{heading} }
/// # }
/// ```
///
/// ## Pattern matching
///
/// `@match` accepts Rust patterns and optional guards. Each arm contains an ASX
/// block:
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
///
/// Every control-flow form requires its complete header and delimiters.
/// Incomplete input is rejected by the ASX parser:
///
/// ```compile_fail
/// use avosetta::asx;
///
/// let status = Some("ready");
/// let _ = asx! { @match status };
/// ```
#[macro_export]
macro_rules! asx {
    ($($tt:tt)*) => {{
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

    ([$($condition:tt)*]) => {
        ::core::compile_error!("`@if` requires a condition and body")
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

    ([$($header:tt)*]) => {
        ::core::compile_error!("`@for` requires an iterator expression and body")
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

    ([$($condition:tt)*]) => {
        ::core::compile_error!("`@while` requires a condition and body")
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

    ([$($statement:tt)*]) => {
        ::core::compile_error!("`@let` requires a semicolon")
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

    ([$($value:tt)*]) => {
        ::core::compile_error!("`@match` requires a value and arm body")
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

    ($($invalid:tt)*) => {
        ::core::compile_error!("invalid `@match` arms; expected `pattern => { ... }`")
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
            $crate::text!(::core::concat!($literal)),
            $crate::__asx_expand!($($rest)*),
        )
    };

    (@{$($tokens:tt)*} $($rest:tt)*) => {
        $crate::__asx_expand! {
            @({ $($tokens)* })
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
                name: $crate::text!(@raw $element),

                attrs: $crate::__attrs!((), { $($attrs)* }).1,
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
                name: $crate::text!(@raw $element),

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
                name: $crate::text!(@raw $element),
                attrs: $crate::__attrs!((), { $($attrs)* }).1,
            },
            $crate::__asx_expand!($($rest)*),
        )
    };

    (
        $element:literal; $($rest:tt)*
    ) => {
        $crate::Chain(
            $crate::elements::Void {
                name: $crate::text!(@raw $element),
                attrs: (),
            },
            $crate::__asx_expand!($($rest)*),
        )
    };

    ($literal:literal $($rest:tt)*) => {
        $crate::Chain(
            $crate::text!(::core::concat!($literal)),
            $crate::__asx_expand!($($rest)*),
        )
    };
    (
        @$path:path[$(.$name:ident: $value:tt),* $(,)?] { $($body:tt)* } $($rest:tt)*
    ) => {{
        $crate::Chain(
            {
                let (fragment, props) = $crate::Fragment::props($crate::FnFragment(
                    $path,
                    ::core::marker::PhantomData,
                ));
                let (props, attrs) = $crate::__attrs!(
                    props,
                    { $(.$name: $value),* }
                );

                $crate::Fragment::html(
                    fragment,
                    $crate::Context {
                        attrs,
                        props,
                        children: $crate::__asx_expand! { $($body)* },
                    },
                )
            },
            $crate::__asx_expand!($($rest)*),
        )
    }};

    (
        @$path:path[$(.$name:ident: $value:tt),* $(,)?]; $($rest:tt)*
    ) => {{
        $crate::Chain(
            {
                let (fragment, props) = $crate::Fragment::props($crate::FnFragment(
                    $path,
                    ::core::marker::PhantomData,
                ));
                let (props, attrs) = $crate::__attrs!(
                    props,
                    { $(.$name: $value),* }
                );

                $crate::Fragment::html(
                    fragment,
                    $crate::Context {
                        attrs,
                        props,
                        children: $crate::Omitted,
                    },
                )
            },
            $crate::__asx_expand!($($rest)*),
        )
    }};

    (
        @$path:path[$($attrs:tt)*] { $($body:tt)* } $($rest:tt)*
    ) => {{
        $crate::Chain(
            {
                let (fragment, props) = $crate::Fragment::props($crate::FnFragment(
                    $path,
                    ::core::marker::PhantomData,
                ));
                let (props, attrs) = $crate::__attrs!(props, { $($attrs)* });

                $crate::Fragment::html(
                    fragment,
                    $crate::Context {
                        attrs,
                        props,
                        children: $crate::__asx_expand! { $($body)* },
                    },
                )
            },
            $crate::__asx_expand!($($rest)*),
        )
    }};

    (
        @$path:path { $($body:tt)* } $($rest:tt)*
    ) => {{
        $crate::__asx_expand! {
            @$path[] { $($body)* } $($rest)*
        }
    }};

    (
        @$path:path[$($attrs:tt)*]; $($rest:tt)*
    ) => {{
        $crate::Chain(
            {
                let (fragment, props) = $crate::Fragment::props($crate::FnFragment(
                    $path,
                    ::core::marker::PhantomData,
                ));
                let (props, attrs) = $crate::__attrs!(props, { $($attrs)* });

                $crate::Fragment::html(
                    fragment,
                    $crate::Context {
                        attrs,
                        props,
                        children: $crate::Omitted,
                    },
                )
            },
            $crate::__asx_expand!($($rest)*),
        )
    }};

    (
        @$path:path; $($rest:tt)*
    ) => {{
        $crate::__asx_expand! {
            @$path[];
            $($rest)*
        }
    }};

    (
        $element:ident[$($attrs:tt)*] { $($body:tt)* } $($rest:tt)*
    ) => {
        $crate::__asx_expand! {
            @$crate::elements::$element[$($attrs)*] { $($body)* } $($rest)*
        }
    };

    (
        $element:ident { $($body:tt)* } $($rest:tt)*
    ) => {
        $crate::__asx_expand! {
            @$crate::elements::$element { $($body)* } $($rest)*
        }
    };

    (
        $element:ident[$($attrs:tt)*]; $($rest:tt)*
    ) => {
        $crate::__asx_expand! {
            @$crate::elements::$element[$($attrs)*]; $($rest)*
        }
    };

    (
        $element:ident; $($rest:tt)*
    ) => {
        $crate::__asx_expand! {
            @$crate::elements::$element; $($rest)*
        }
    };

    ($($invalid:tt)+) => {
        ::core::compile_error!("invalid ASX syntax")
    };
}
