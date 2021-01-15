pub trait Functor {
    type Unwrapped;
    type Container<B>: Functor;

    fn map<F, B>(&self, f: F) -> Self::Container<B>
    where
        F: FnOnce(&Self::Unwrapped) -> B;
}
