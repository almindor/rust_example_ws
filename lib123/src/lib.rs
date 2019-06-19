use core::fmt::Debug;

mod internal {
    use core::fmt::Debug;

    pub trait UsedByCrateOnly
    {
        fn do_something(&self);
    }
    
    pub(crate) struct ImplementorCrateOnly<X>
    where X: Sized + Debug
    {
        x: X
    }
    
    impl<X> UsedByCrateOnly for ImplementorCrateOnly<X>
    where X: Sized + Debug
    {
        fn do_something(&self) {
            println!("{:?}", self.x);
        }
    }
    
    impl<X> ImplementorCrateOnly<X>
    where X: Sized + Debug
    {
        pub(crate) fn new(x: X) -> Self {
            ImplementorCrateOnly {
                x
            }
        }
    }
}

pub struct VisibleToUsers<I>
where I: internal::UsedByCrateOnly
{
    i: I
}

impl<I> VisibleToUsers<I>
where I: internal::UsedByCrateOnly
{
    pub fn use_internal(&self) {
        self.i.do_something();
    }
    
    // internal constructor to be used by externalizer impls (see below)
    fn new(i: I) -> Self {
        VisibleToUsers {
            i
        }
    }
}

impl<X> VisibleToUsers<internal::ImplementorCrateOnly<X>>
where X: Sized + Debug
{
    pub fn from_args_for_specific_impl(x: X) -> Self {
        VisibleToUsers::new(internal::ImplementorCrateOnly::new(x))
    }    
}
