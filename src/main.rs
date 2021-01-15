#![feature(generic_associated_types)]
#![allow(incomplete_features)]

pub mod impls;
pub mod traits;

use crate::impls::maybe::Maybe;
use crate::traits::applicative::Applicative;
use crate::traits::functor::Functor;
use crate::traits::monad::Monad;

fn main() {
    // Functor
    assert_eq!(Maybe::Just(5).map(|x| x * 5), Maybe::Just(25));
    assert_eq!(Maybe::Just(8).map(|x| x * 2), Maybe::Just(16));

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
