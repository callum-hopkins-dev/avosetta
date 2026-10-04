use std::marker::PhantomData;

use crate::{Attributes, Html};

/// Marks a fragment function parameter as user-defined properties.
///
/// Fragment functions must unwrap this marker before using its value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Props<T>(pub T);

/// Marks a fragment function parameter as HTML attributes.
///
/// Fragment functions must unwrap this marker before using its value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Attrs<T>(pub T);

/// Marks a fragment function parameter as child markup.
///
/// Fragment functions must unwrap this marker before using its value.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Children<T>(pub T);

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Omitted;

impl Html for Omitted {
    type Segments<T: crate::html::Segments> = T;

    #[inline(always)]
    fn segments<T: crate::html::Segments>(self, x: T) -> Self::Segments<T> {
        x
    }

    #[inline(always)]
    fn is_present(&self) -> bool {
        false
    }
}

impl Attributes for Omitted {
    type Class = ();
    type Style = ();
    type Other = ();

    #[inline(always)]
    fn into_parts(self) -> (Self::Class, Self::Style, Self::Other) {
        ((), (), ())
    }
}

/// The complete set of inputs passed to a [`Fragment`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context<F>
where
    F: Fragment,
{
    pub attrs: F::Attrs,
    pub props: F::Props,
    pub children: F::Children,
}

/// A reusable unit of markup that can be called from [`asx!`](crate::asx).
///
/// Ordinary functions are adapted to this trait automatically when their
/// parameters use [`Props`], [`Attrs`], and [`Children`].
pub trait Fragment: Sized {
    /// The HTML attributes accepted by this fragment.
    type Attrs: Attributes;

    /// The user-defined properties accepted by this fragment.
    type Props: Default;

    /// The child markup accepted by this fragment.
    type Children: crate::Html;

    /// Converts this fragment and its inputs into renderable HTML.
    fn html(self, cx: Context<Self>) -> impl Html;

    #[inline(always)]
    #[doc(hidden)]
    fn props(self) -> (Self, Self::Props) {
        (self, Default::default())
    }
}

#[doc(hidden)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FnFragment<F, K>(pub F, pub PhantomData<K>);

macro_rules! __impl_fn_fragment {
    (
        impl$(<$($generics:ident),* $(,)?>)? Fragment {
            type Attrs = $attrs:ty;

            type Props = $props:ty;

            type Children = $children:ty;

            fn call($($args_wrapper:ident($args_ident:ident): $args_ty:ty),* $(,)?);
        }
    ) => {
        impl<F, H, $($($generics),*)?> Fragment for FnFragment<F, (H, $($args_ty),*)>
        where
            F: FnOnce($($args_ty),*) -> H,
            H: Html,
            $attrs: Attributes,
            $props: Default,
            $children: crate::Html,
        {
            type Attrs = $attrs;

            type Props = $props;

            type Children = $children;

            #[inline(always)]
            #[allow(unused_variables)]
            fn html(self, cx: Context<Self>) -> impl Html {
                (self.0)($($args_wrapper(cx.$args_ident)),*)
            }
        }
    };
}

__impl_fn_fragment! {
    impl Fragment {
        type Attrs = Omitted;

        type Props = ();

        type Children = Omitted;

        fn call();
    }
}

__impl_fn_fragment! {
    impl<P> Fragment {
        type Attrs = Omitted;

        type Props = P;

        type Children = Omitted;

        fn call(Props(props): Props<P>);
    }
}

__impl_fn_fragment! {
    impl<A> Fragment {
        type Attrs = A;

        type Props = ();

        type Children = Omitted;

        fn call(Attrs(attrs): Attrs<A>);
    }
}

__impl_fn_fragment! {
    impl<C> Fragment {
        type Attrs = Omitted;

        type Props = ();

        type Children = C;

        fn call(Children(children): Children<C>);
    }
}

__impl_fn_fragment! {
    impl<P, A> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = Omitted;

        fn call(Props(props): Props<P>, Attrs(attrs): Attrs<A>);
    }
}

__impl_fn_fragment! {
    impl<A, P> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = Omitted;

        fn call(Attrs(attrs): Attrs<A>, Props(props): Props<P>);
    }
}

__impl_fn_fragment! {
    impl<P, C> Fragment {
        type Attrs = Omitted;

        type Props = P;

        type Children = C;

        fn call(Props(props): Props<P>, Children(children): Children<C>);
    }
}

__impl_fn_fragment! {
    impl<C, P> Fragment {
        type Attrs = Omitted;

        type Props = P;

        type Children = C;

        fn call(Children(children): Children<C>, Props(props): Props<P>);
    }
}

__impl_fn_fragment! {
    impl<A, C> Fragment {
        type Attrs = A;

        type Props = ();

        type Children = C;

        fn call(Attrs(attrs): Attrs<A>, Children(children): Children<C>);
    }
}

__impl_fn_fragment! {
    impl<C, A> Fragment {
        type Attrs = A;

        type Props = ();

        type Children = C;

        fn call(Children(children): Children<C>, Attrs(attrs): Attrs<A>);
    }
}

__impl_fn_fragment! {
    impl<P, A, C> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(Props(props): Props<P>, Attrs(attrs): Attrs<A>, Children(children): Children<C>);
    }
}

__impl_fn_fragment! {
    impl<P, C, A> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(Props(props): Props<P>, Children(children): Children<C>, Attrs(attrs): Attrs<A>);
    }
}

__impl_fn_fragment! {
    impl<A, P, C> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(Attrs(attrs): Attrs<A>, Props(props): Props<P>, Children(children): Children<C>);
    }
}

__impl_fn_fragment! {
    impl<A, C, P> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(Attrs(attrs): Attrs<A>, Children(children): Children<C>, Props(props): Props<P>);
    }
}

__impl_fn_fragment! {
    impl<C, P, A> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(Children(children): Children<C>, Props(props): Props<P>, Attrs(attrs): Attrs<A>);
    }
}

__impl_fn_fragment! {
    impl<C, A, P> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(Children(children): Children<C>, Attrs(attrs): Attrs<A>, Props(props): Props<P>);
    }
}
