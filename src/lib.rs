extern crate num_traits;

mod common;
mod solver;

#[cfg(test)]
mod tests;

pub use common::CharWidthSelector;
pub use common::EastAsianWidth;
pub use common::Width;
pub use solver::solve_eaw;

use common::EastAsianWidth::*;
use num_traits::Zero;

impl Width for str {
    fn width<T: CharWidthSelector>(&self, selector: T) -> T::Output {
        self.chars()
            .map(|c| selector.width(c, solve_eaw(c as u32)))
            .fold(T::Output::zero(), |a, b| a + b)
    }
}

pub struct EastAsianContextCharWidthSelector;
pub struct NonEastAsianContextCharWidthSelector;

impl CharWidthSelector for EastAsianContextCharWidthSelector {
    type Output = usize;
    fn width(&self, _: char, eaw: EastAsianWidth) -> usize {
        match eaw {
            Full | Wide | Ambiguous => 2,
            Half | Narrow | Neutral => 1,
        }
    }
}
impl CharWidthSelector for NonEastAsianContextCharWidthSelector {
    type Output = usize;
    fn width(&self, _: char, eaw: EastAsianWidth) -> usize {
        match eaw {
            Full | Wide => 2,
            Half | Narrow | Ambiguous | Neutral => 1,
        }
    }
}
