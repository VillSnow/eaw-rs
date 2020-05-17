#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EastAsianWidth {
    Full,
    Half,
    Wide,
    Narrow,
    Ambiguous,
    Neutral,
}

pub trait CharWidthSelector {
    type Output: num_traits::Zero + std::ops::Add;
    fn width(&self, c: char, eaw: EastAsianWidth) -> Self::Output;
}

pub trait Width {
    fn width<T: CharWidthSelector>(&self, selector: T) -> T::Output;
}
