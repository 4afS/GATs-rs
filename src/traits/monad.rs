use crate::traits::applicative::Applicative;

pub trait Monad: Applicative {
    fn bind<F, B>(&self, f: F) -> Self::Container<B>
    where
        F: FnOnce(&Self::Unwrapped) -> Self::Container<B>;
}
