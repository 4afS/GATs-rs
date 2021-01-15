use crate::traits::applicative::Applicative;
use crate::traits::functor::Functor;
use crate::traits::monad::Monad;

#[derive(Debug, PartialEq, Eq)]
pub enum Either<E: Clone, A> {
    Left(E),
    Right(A),
}

impl<E: Clone, A> Functor for Either<E, A> {
    type Unwrapped = A;
    type Container<B> = Either<E, B>;
    fn map<F: FnOnce(&A) -> B, B>(&self, f: F) -> Either<E, B> {
        match self {
            Either::Right(ref x) => Either::Right(f(x)),
            Either::Left(e) => Either::Left(e.clone()),
        }
    }
}

impl<E: Clone, A> Applicative for Either<E, A> {
    fn lift_a2<F, B, C>(&self, b: &Self::Container<B>, f: F) -> Self::Container<C>
    where
        F: FnOnce(&Self::Unwrapped, &B) -> C,
    {
        match self {
            Either::Left(e) => Either::Left(e.clone()),
            Either::Right(ref a) => match b {
                Either::Left(e) => Either::Left(e.clone()),
                Either::Right(ref b) => Either::Right(f(a, b)),
            },
        }
    }
}

impl<E: Clone, A> Monad for Either<E, A> {
    fn bind<F, B>(&self, f: F) -> Self::Container<B>
    where
        F: FnOnce(&Self::Unwrapped) -> Self::Container<B>,
    {
        match self {
            Either::Right(a) => f(a),
            Either::Left(e) => Either::Left(e.clone()),
        }
    }
}
