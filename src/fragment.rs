use crate::Attributes;

/// Marks a component function argument as custom properties.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Props<T>(pub T);

/// Marks a component function argument as HTML attributes.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Attrs<T>(pub T);

/// Marks a component function argument as child HTML.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Children<T>(pub T);

#[doc(hidden)]
pub trait Fragment<Html, Discriminant>
where
    Html: crate::Html,
    Self: Sized,
{
    type Attrs: Attributes;
    type Props: Default;
    type Children: crate::Html;
    fn props(self) -> (Self, Self::Props);
    fn call(self, attrs: Self::Attrs, props: Self::Props, children: Self::Children) -> Html;
}

macro_rules! __impl_fragment {
    (
        impl$(<$($generics:ident),* $(,)?>)? Fragment {
            type Attrs = $attrs:ty;

            type Props = $props:ty;

            type Children = $children:ty;

            fn call($($args_ident:ident: $args_ty:ty),* $(,)?);
        }
    ) => {
        impl<F, Html, $($($generics),*)?> Fragment<Html, fn($($args_ty),*)> for F
        where
            F: FnOnce($($args_ty),*) -> Html,
            Html: crate::Html,
            $attrs: Attributes,
            $props: Default,
            $children: crate::Html,
        {
            type Attrs = $attrs;

            type Props = $props;

            type Children = $children;

            #[inline(always)]
            fn props(self) -> (Self, Self::Props) {
                (self, Default::default())
            }

            #[inline(always)]
            fn call(self, attrs: Self::Attrs, props: Self::Props, children: Self::Children) -> Html {
                #[allow(dead_code)]
                struct Args<T0, T1, T2> {
                    attrs: T0,
                    props: T1,
                    children: T2,
                }

                #[allow(unused_variables)]
                let args = Args {
                    attrs: Attrs(attrs),
                    props: Props(props),
                    children: Children(children),
                };

                (self)($(args.$args_ident),*)
            }

        }

    };
}

__impl_fragment! {
    impl<P, A, C> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(props: Props<P>, attrs: Attrs<A>, children: Children<C>);
    }
}

__impl_fragment! {
    impl Fragment {
        type Attrs = ();

        type Props = ();

        type Children = ();

        fn call();
    }
}

__impl_fragment! {
    impl<P> Fragment {
        type Attrs = ();

        type Props = P;

        type Children = ();

        fn call(props: Props<P>);
    }
}

__impl_fragment! {
    impl<A> Fragment {
        type Attrs = A;

        type Props = ();

        type Children = ();

        fn call(attrs: Attrs<A>);
    }
}

__impl_fragment! {
    impl<C> Fragment {
        type Attrs = ();

        type Props = ();

        type Children = C;

        fn call(children: Children<C>);
    }
}

__impl_fragment! {
    impl<P, A> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = ();

        fn call(props: Props<P>, attrs: Attrs<A>);
    }
}

__impl_fragment! {
    impl<A, P> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = ();

        fn call(attrs: Attrs<A>, props: Props<P>);
    }
}

__impl_fragment! {
    impl<P, C> Fragment {
        type Attrs = ();

        type Props = P;

        type Children = C;

        fn call(props: Props<P>, children: Children<C>);
    }
}

__impl_fragment! {
    impl<C, P> Fragment {
        type Attrs = ();

        type Props = P;

        type Children = C;

        fn call(children: Children<C>, props: Props<P>);
    }
}

__impl_fragment! {
    impl<A, C> Fragment {
        type Attrs = A;

        type Props = ();

        type Children = C;

        fn call(attrs: Attrs<A>, children: Children<C>);
    }
}

__impl_fragment! {
    impl<C, A> Fragment {
        type Attrs = A;

        type Props = ();

        type Children = C;

        fn call(children: Children<C>, attrs: Attrs<A>);
    }
}

__impl_fragment! {
    impl<P, C, A> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(props: Props<P>, children: Children<C>, attrs: Attrs<A>);
    }
}

__impl_fragment! {
    impl<A, P, C> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(attrs: Attrs<A>, props: Props<P>, children: Children<C>);
    }
}

__impl_fragment! {
    impl<A, C, P> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(attrs: Attrs<A>, children: Children<C>, props: Props<P>);
    }
}

__impl_fragment! {
    impl<C, P, A> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(children: Children<C>, props: Props<P>, attrs: Attrs<A>);
    }
}

__impl_fragment! {
    impl<C, A, P> Fragment {
        type Attrs = A;

        type Props = P;

        type Children = C;

        fn call(children: Children<C>, attrs: Attrs<A>, props: Props<P>);
    }
}
