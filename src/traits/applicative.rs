use crate::traits::functor::Functor;

pub trait Applicative: Functor {
    fn lift_a2<F, B, C>(&self, b: &Self::Container<B>, f: F) -> Self::Container<C>
    where
        F: FnOnce(&Self::Unwrapped, &B) -> C;
}
