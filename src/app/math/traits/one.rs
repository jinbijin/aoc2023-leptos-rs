pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

impl One for usize {
    fn one() -> Self {
        1
    }

    fn is_one(&self) -> bool {
        *self == 1
    }
}