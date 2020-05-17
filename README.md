# eaw-rs
Rust crate to get text width considering east asian width (http://www.unicode.org/reports/tr11/)

## Usage
```rust
extern crate eaw;
use eaw::EastAsianContextCharWidthSelector;
use eaw::Width;

fn main() {
    let s = "令和２年５月17日";
    let w = s.width(EastAsianContextCharWidthSelector);
    assert_eq!(w, 16);
}
```


