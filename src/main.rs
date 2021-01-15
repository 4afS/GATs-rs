#![feature(generic_associated_types)]

#[derive(Debug, PartialEq, Eq)]
enum Maybe<A> {
    Just(A),
    Nothing,
}

trait Functor {
    type Unwrapped;
    type Container<B>: Functor;

    fn map<F, B>(&self, f: F) -> Self::Container<B>
    where
        F: FnOnce(&Self::Unwrapped) -> B;
}

trait Applicative: Functor {
    fn lift_a2<F, B, C>(&self, b: &Self::Container<B>, f: F) -> Self::Container<C>
    where
        F: FnOnce(&Self::Unwrapped, &B) -> C;
}

trait Monad: Applicative {
    fn bind<F, B>(&self, f: F) -> Self::Container<B>
    where
        F: FnOnce(&Self::Unwrapped) -> Self::Container<B>;
}

impl<A> Functor for Maybe<A> {
    type Unwrapped = A;
    type Container<B> = Maybe<B>;
    fn map<F: FnOnce(&A) -> B, B>(&self, mut f: F) -> Maybe<B> {
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

fn main() {
    let a: Maybe<i32> = Maybe::Just(5);
    let b: Maybe<i32> = Maybe::Just(8);

    // Functor
    assert_eq!(a.map(|x| x * 5), Maybe::Just(25));
    assert_eq!(b.map(|x| x * 2), Maybe::Just(16));

    // Applicative
    assert_eq!(
        Maybe::Just(2).lift_a2(&Maybe::Just(3), |x, y| x + y),
        Maybe::Just(5)
    );

    // Monad
    assert_eq!(
        Maybe::Just(5)
            .bind(|x| Maybe::Just(x * 2))
            .bind(|y| Maybe::Just(y * 3)),
        Maybe::Just(30)
    );
}
