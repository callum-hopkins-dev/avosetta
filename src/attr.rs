use crate::html::{Chain, DynamicWrite, End, Html, Segments, Text};

mod sealed {
    pub trait SegmentKind {}
}

#[doc(hidden)]
pub trait SegmentKind: sealed::SegmentKind {
    type PrependStatic: SegmentKind;

    type PrependDynamic: SegmentKind;

    type Attribute<N: Html, V: Html, T: Segments>: Segments;

    type GroupAttribute<N: Html, V: Html, T: Segments>: Segments;

    type JoinClass<L: Html, R: Html, T: Segments>: Segments;

    fn attribute<N: Html, V: Html, T: Segments>(x: Attr<N, V>, tail: T)
    -> Self::Attribute<N, V, T>;

    fn group_attribute<N: Html, V: Html, T: Segments>(
        x: Attr<N, V>,
        tail: T,
    ) -> Self::GroupAttribute<N, V, T>;

    fn join_class<L: Html, R: Html, T: Segments>(
        left: L,
        right: R,
        tail: T,
    ) -> Self::JoinClass<L, R, T>;
}

/// An HTML attribute name and value pair.
///
/// The complete attribute is omitted when the value is not present.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attr<Name, Value>(pub Name, pub Value);

impl<Name, Value> Html for Attr<Name, Value>
where
    Name: Html,
    Value: Html,
{
    type Segments<T: Segments> = <GroupKind<Value> as SegmentKind>::Attribute<Name, Value, T>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        <GroupKind<Value> as SegmentKind>::attribute(self, x)
    }
}

impl<Name, Value> DynamicWrite for Attr<Name, Value>
where
    Name: Html,
    Value: Html,
{
    #[inline(always)]
    fn write(self, s: &mut String) {
        if self.1.is_attribute_present() {
            s.push(' ');
            self.0.write(s);
            s.push_str("=\"");
            self.1.write(s);
            s.push('"');
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EmptySegments;

impl sealed::SegmentKind for EmptySegments {}

impl SegmentKind for EmptySegments {
    type PrependStatic = StaticSegments;

    type PrependDynamic = ConditionalSegments;

    type Attribute<N: Html, V: Html, T: Segments> = T;

    type GroupAttribute<N: Html, V: Html, T: Segments> = T;

    type JoinClass<L: Html, R: Html, T: Segments> = R::Segments<T>;

    #[inline(always)]
    fn attribute<N: Html, V: Html, T: Segments>(
        _x: Attr<N, V>,
        tail: T,
    ) -> Self::Attribute<N, V, T> {
        tail
    }

    #[inline(always)]
    fn group_attribute<N: Html, V: Html, T: Segments>(
        _x: Attr<N, V>,
        tail: T,
    ) -> Self::GroupAttribute<N, V, T> {
        tail
    }

    #[inline(always)]
    fn join_class<L: Html, R: Html, T: Segments>(
        _left: L,
        right: R,
        tail: T,
    ) -> Self::JoinClass<L, R, T> {
        right.segments(tail)
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StaticSegments;

impl sealed::SegmentKind for StaticSegments {}

impl SegmentKind for StaticSegments {
    type PrependStatic = Self;

    type PrependDynamic = PresentSegments;

    type Attribute<N: Html, V: Html, T: Segments> = ExpandedAttribute<N, V, T>;

    type GroupAttribute<N: Html, V: Html, T: Segments> = ExpandedAttribute<N, V, T>;

    type JoinClass<L: Html, R: Html, T: Segments> =
        <Chain<L, Chain<Text<1>, R>> as Html>::Segments<T>;

    #[inline(always)]
    fn attribute<N: Html, V: Html, T: Segments>(
        x: Attr<N, V>,
        tail: T,
    ) -> Self::Attribute<N, V, T> {
        expand_attribute(x, tail)
    }

    #[inline(always)]
    fn group_attribute<N: Html, V: Html, T: Segments>(
        x: Attr<N, V>,
        tail: T,
    ) -> Self::GroupAttribute<N, V, T> {
        expand_attribute(x, tail)
    }

    #[inline(always)]
    fn join_class<L: Html, R: Html, T: Segments>(
        left: L,
        right: R,
        tail: T,
    ) -> Self::JoinClass<L, R, T> {
        Chain(left, Chain(crate::__static_text!(@raw " "), right)).segments(tail)
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PresentSegments;

impl sealed::SegmentKind for PresentSegments {}

impl SegmentKind for PresentSegments {
    type PrependStatic = Self;

    type PrependDynamic = Self;

    type Attribute<N: Html, V: Html, T: Segments> = ExpandedAttribute<N, V, T>;

    type GroupAttribute<N: Html, V: Html, T: Segments> = ExpandedAttribute<N, V, T>;

    type JoinClass<L: Html, R: Html, T: Segments> =
        <Chain<L, Chain<Text<1>, R>> as Html>::Segments<T>;

    #[inline(always)]
    fn attribute<N: Html, V: Html, T: Segments>(
        x: Attr<N, V>,
        tail: T,
    ) -> Self::Attribute<N, V, T> {
        expand_attribute(x, tail)
    }

    #[inline(always)]
    fn group_attribute<N: Html, V: Html, T: Segments>(
        x: Attr<N, V>,
        tail: T,
    ) -> Self::GroupAttribute<N, V, T> {
        expand_attribute(x, tail)
    }

    #[inline(always)]
    fn join_class<L: Html, R: Html, T: Segments>(
        left: L,
        right: R,
        tail: T,
    ) -> Self::JoinClass<L, R, T> {
        Chain(left, Chain(crate::__static_text!(@raw " "), right)).segments(tail)
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionalSegments;

impl sealed::SegmentKind for ConditionalSegments {}

impl SegmentKind for ConditionalSegments {
    type PrependStatic = PresentSegments;

    type PrependDynamic = Self;

    type Attribute<N: Html, V: Html, T: Segments> = T::PrependDynamic<Attr<N, V>>;

    type GroupAttribute<N: Html, V: Html, T: Segments> = ExpandedAttribute<N, V, T>;

    type JoinClass<L: Html, R: Html, T: Segments> =
        <Chain<L, Chain<Text<1>, R>> as Html>::Segments<T>;

    #[inline(always)]
    fn attribute<N: Html, V: Html, T: Segments>(
        x: Attr<N, V>,
        tail: T,
    ) -> Self::Attribute<N, V, T> {
        tail.prepend_dynamic(x)
    }

    #[inline(always)]
    fn group_attribute<N: Html, V: Html, T: Segments>(
        x: Attr<N, V>,
        tail: T,
    ) -> Self::GroupAttribute<N, V, T> {
        expand_attribute(x, tail)
    }

    #[inline(always)]
    fn join_class<L: Html, R: Html, T: Segments>(
        left: L,
        right: R,
        tail: T,
    ) -> Self::JoinClass<L, R, T> {
        Chain(left, Chain(crate::__static_text!(@raw " "), right)).segments(tail)
    }
}

/// A collection of HTML attributes that can be rendered or projected.
///
/// Class and style values are kept separate from ordinary attributes so that
/// projected collections can merge them into one `class` and one `style`
/// attribute. Ordinary attributes retain their original order and may repeat.
pub trait Attributes: Html {
    #[doc(hidden)]
    type Class: Html;

    #[doc(hidden)]
    type Style: Html;

    #[doc(hidden)]
    type Other: Html;

    #[doc(hidden)]
    fn into_parts(self) -> (Self::Class, Self::Style, Self::Other);
}

impl Attributes for () {
    type Class = ();
    type Style = ();
    type Other = ();

    #[inline(always)]
    fn into_parts(self) -> (Self::Class, Self::Style, Self::Other) {
        ((), (), ())
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttributeSet<C, S, O> {
    pub class: C,
    pub style: S,
    pub other: O,
}

impl<C, S, O> Attributes for AttributeSet<C, S, O>
where
    C: Html,
    S: Html,
    O: Html,
    Class<C>: Html,
    Style<S>: Html,
{
    type Class = C;
    type Style = S;
    type Other = O;

    #[inline(always)]
    fn into_parts(self) -> (Self::Class, Self::Style, Self::Other) {
        (self.class, self.style, self.other)
    }
}

impl<C, S, O> Html for AttributeSet<C, S, O>
where
    C: Html,
    S: Html,
    O: Html,
    Class<C>: Html,
    Style<S>: Html,
{
    type Segments<T: Segments> = <Chain<Class<C>, Chain<Style<S>, O>> as Html>::Segments<T>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        Chain(Class(self.class), Chain(Style(self.style), self.other)).segments(x)
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Class<T>(pub T);

impl<V> Html for Class<V>
where
    V: Html,
{
    type Segments<T: Segments> = <GroupKind<V> as SegmentKind>::GroupAttribute<Text<5>, V, T>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        <GroupKind<V> as SegmentKind>::group_attribute(
            Attr(crate::__static_text!(@raw "class"), self.0),
            x,
        )
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassChain<L, R>(pub L, pub R);

impl<L, R> Html for ClassChain<L, R>
where
    L: Html,
    R: Html,
{
    type Segments<T: Segments> = <GroupKind<L> as SegmentKind>::JoinClass<L, R, T>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        <GroupKind<L> as SegmentKind>::join_class(self.0, self.1, x)
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Style<T>(pub T);

impl<V> Html for Style<V>
where
    V: Html,
{
    type Segments<T: Segments> = <GroupKind<V> as SegmentKind>::GroupAttribute<Text<5>, V, T>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        <GroupKind<V> as SegmentKind>::group_attribute(
            Attr(crate::__static_text!(@raw "style"), self.0),
            x,
        )
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleValue<V>(pub V);

impl<V> Html for StyleValue<V>
where
    V: Html,
{
    type Segments<T: Segments> = <Chain<V, Text<1>> as Html>::Segments<T>;

    #[inline(always)]
    fn segments<T: Segments>(self, x: T) -> Self::Segments<T> {
        Chain(self.0, crate::__static_text!(@raw ";")).segments(x)
    }
}

type GroupKind<T> = <<T as Html>::Segments<End> as Segments>::Kind;

type ExpandedAttribute<N, V, T> =
    <Chain<Text<1>, Chain<N, Chain<Text<2>, Chain<V, Text<1>>>>> as Html>::Segments<T>;

#[inline(always)]
fn expand_attribute<N, V, T>(x: Attr<N, V>, tail: T) -> ExpandedAttribute<N, V, T>
where
    N: Html,
    V: Html,
    T: Segments,
{
    Chain(
        crate::__static_text!(@raw " "),
        Chain(
            x.0,
            Chain(
                crate::__static_text!(@raw "=\""),
                Chain(x.1, crate::__static_text!(@raw "\"")),
            ),
        ),
    )
    .segments(tail)
}

#[macro_export]
#[doc(hidden)]
macro_rules! __attrs {
    ($props:expr, { $(.$name:ident: $value:tt),* $(,)? }) => {{
        #[allow(unused_mut)]
        let mut props = $props;

        let attrs = $crate::Omitted;

        $crate::__attrs_expand!(props, attrs, $(.$name: $value),*);

        (props, attrs)
    }};

    ($props:expr, { $($tokens:tt)* }) => {{
        #[allow(unused_mut)]
        let mut props = $props;

        let attrs = $crate::AttributeSet {
            class: (),
            style: (),
            other: (),
        };

        $crate::__attrs_expand!(props, attrs, $($tokens)*);

        (props, attrs)
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __attrs_expand {
    ($props:ident, $attrs:ident,) => {};

    ($props:ident, $attrs:ident, .$name:ident: $value:literal $(, $($rest:tt)*)?) => {
        $props.$name = $value;
        $crate::__attrs_expand!($props, $attrs, $($($rest)*)?);
    };

    ($props:ident, $attrs:ident, .$name:ident: {$value:expr} $(, $($rest:tt)*)?) => {
        $props.$name = $value;
        $crate::__attrs_expand!($props, $attrs, $($($rest)*)?);
    };

    ($props:ident, $attrs:ident, ..$value:expr) => {
        let (class, style, other) = $crate::Attributes::into_parts($value);

        let $attrs = $crate::AttributeSet {
            class: $crate::ClassChain($attrs.class, class),
            style: $crate::Chain($attrs.style, style),
            other: $crate::Chain($attrs.other, other),
        };
    };

    ($props:ident, $attrs:ident, class: $value:literal $(, $($rest:tt)*)?) => {
        $crate::__attrs_expand!(
            $props,
            $attrs,
            class: {$crate::__static_text!($value)}
            $(, $($rest)*)?
        );
    };

    ($props:ident, $attrs:ident, {"class"}: $value:literal $(, $($rest:tt)*)?) => {
        $crate::__attrs_expand!($props, $attrs, class: $value $(, $($rest)*)?);
    };

    ($props:ident, $attrs:ident, class: {$value:expr} $(, $($rest:tt)*)?) => {
        let $attrs = $crate::AttributeSet {
            class: $crate::ClassChain($attrs.class, $value),
            style: $attrs.style,
            other: $attrs.other,
        };

        $crate::__attrs_expand!($props, $attrs, $($($rest)*)?);
    };

    ($props:ident, $attrs:ident, {"class"}: {$value:expr} $(, $($rest:tt)*)?) => {
        $crate::__attrs_expand!($props, $attrs, class: {$value} $(, $($rest)*)?);
    };

    ($props:ident, $attrs:ident, style: $value:literal $(, $($rest:tt)*)?) => {
        $crate::__attrs_expand!(
            $props,
            $attrs,
            style: {$crate::__static_text!($value)}
            $(, $($rest)*)?
        );
    };

    ($props:ident, $attrs:ident, {"style"}: $value:literal $(, $($rest:tt)*)?) => {
        $crate::__attrs_expand!($props, $attrs, style: $value $(, $($rest)*)?);
    };

    ($props:ident, $attrs:ident, style: {$value:expr} $(, $($rest:tt)*)?) => {
        let $attrs = $crate::AttributeSet {
            class: $attrs.class,
            style: $crate::Chain(
                $attrs.style,
                $crate::StyleValue($value),
            ),
            other: $attrs.other,
        };

        $crate::__attrs_expand!($props, $attrs, $($($rest)*)?);
    };

    ($props:ident, $attrs:ident, {"style"}: {$value:expr} $(, $($rest:tt)*)?) => {
        $crate::__attrs_expand!($props, $attrs, style: {$value} $(, $($rest)*)?);
    };

    ($props:ident, $attrs:ident, {$name:literal}: $value:literal $(, $($rest:tt)*)?) => {
        $crate::__attrs_expand!(
            $props,
            $attrs,
            {$crate::__static_text!(@raw $name)}: {$crate::__static_text!($value)}
            $(, $($rest)*)?
        );
    };

    ($props:ident, $attrs:ident, $name:ident: $value:literal $(, $($rest:tt)*)?) => {
        $crate::__attrs_expand!(
            $props,
            $attrs,
            {$crate::__static_text!(@raw ::core::stringify!($name))}: {$crate::__static_text!($value)}
            $(, $($rest)*)?
        );
    };

    ($props:ident, $attrs:ident, {$name:literal}: {$value:expr} $(, $($rest:tt)*)?) => {
        $crate::__attrs_expand!(
            $props,
            $attrs,
            {$crate::__static_text!(@raw $name)}: {$value}
            $(, $($rest)*)?
        );
    };

    ($props:ident, $attrs:ident, $name:ident: {$value:expr} $(, $($rest:tt)*)?) => {
        $crate::__attrs_expand!(
            $props,
            $attrs,
            {$crate::__static_text!(@raw ::core::stringify!($name))}: {$value}
            $(, $($rest)*)?
        );
    };

    ($props:ident, $attrs:ident, {$name:expr}: {$value:expr} $(, $($rest:tt)*)?) => {
        let $attrs = $crate::AttributeSet {
            class: $attrs.class,
            style: $attrs.style,
            other: $crate::Chain(
                $attrs.other,
                $crate::Attr($name, $value),
            ),
        };

        $crate::__attrs_expand!($props, $attrs, $($($rest)*)?);
    };
}
