use crate::traits::applicative::Applicative;
use crate::traits::functor::Functor;
use crate::traits::monad::Monad;

#[derive(Debug, PartialEq, Eq)]
pub enum Either<E, A> {
    Left(E),
    Right(A),
}

impl<E, A> Functor for Either<E, A> {
    type Unwrapped = A;
    type Container<E, B> = Maybe<E, B>;
    fn map<F: FnOnce(&A) -> B, B>(&self, f: F) -> Either<E, B> {
        match self {
            Either::Right(ref x) => Either::Right(f(x)),
            Either::Left(e) => Either::Left(e),
        }
    }
}
