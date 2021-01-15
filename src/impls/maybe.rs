// #![feature(generic_associated_types)]

use crate::traits::applicative::Applicative;
use crate::traits::functor::Functor;
use crate::traits::monad::Monad;

#[derive(Debug, PartialEq, Eq)]
pub enum Maybe<A> {
    Just(A),
    Nothing,
}

impl<A> Functor for Maybe<A> {
    type Unwrapped = A;
    type Container<B> = Maybe<B>;
    fn map<F: FnOnce(&A) -> B, B>(&self, f: F) -> Maybe<B> {
        match self {
            Maybe::Just(ref x) => Maybe::Just(f(x)),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

impl<A> Applicative for Maybe<A> {
    fn lift_a2<F, B, C>(&self, b: &Self::Container<B>, f: F) -> Self::Container<C>
    where
        F: FnOnce(&Self::Unwrapped, &B) -> C,
    {
        match self {
            Maybe::Nothing => return Maybe::Nothing,
            Maybe::Just(ref a) => match b {
                Maybe::Nothing => return Maybe::Nothing,
                Maybe::Just(ref b) => return Maybe::Just(f(a, b)),
            },
        }
    }
}

impl<A> Monad for Maybe<A> {
    fn bind<F, B>(&self, f: F) -> Self::Container<B>
    where
        F: FnOnce(&Self::Unwrapped) -> Self::Container<B>,
    {
        match self {
            Maybe::Just(a) => f(a),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}
