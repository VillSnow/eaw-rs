use crate::Width;
use crate::*;

#[test]
fn test_solver() {
    assert_eq!(EastAsianWidth::Ambiguous, solve_eaw(0xA1));
    assert_eq!(EastAsianWidth::Narrow, solve_eaw(0xA2));
    assert_eq!(EastAsianWidth::Narrow, solve_eaw(0xA3));
    assert_eq!(EastAsianWidth::Ambiguous, solve_eaw(0xA4));
    assert_eq!(EastAsianWidth::Narrow, solve_eaw(0xA5));

    assert_eq!(EastAsianWidth::Wide, solve_eaw('あ' as u32));
    assert_eq!(EastAsianWidth::Half, solve_eaw('ｱ' as u32));
    assert_eq!(EastAsianWidth::Wide, solve_eaw('安' as u32));
    assert_eq!(EastAsianWidth::Neutral, solve_eaw(0xA0));
    assert_eq!(EastAsianWidth::Neutral, solve_eaw(0xFFFF)); // missing in the database file
}

#[test]
fn test_string_width() {
    let s = "ＡｱあA*α𓄿";
    assert_eq!(10, s.width(EastAsianContextCharWidthSelector));
    assert_eq!(9, s.width(NonEastAsianContextCharWidthSelector));

    let s1 = "ｱあA*α𓄿";
    assert_eq!(8, s1.width(EastAsianContextCharWidthSelector));
    assert_eq!(7, s1.width(NonEastAsianContextCharWidthSelector));

    let s2 = "ＡあA*α𓄿";
    assert_eq!(9, s2.width(EastAsianContextCharWidthSelector));
    assert_eq!(8, s2.width(NonEastAsianContextCharWidthSelector));

    let s3 = "ＡｱA*α𓄿";
    assert_eq!(8, s3.width(EastAsianContextCharWidthSelector));
    assert_eq!(7, s3.width(NonEastAsianContextCharWidthSelector));

    let s4 = "Ａｱあ*α𓄿";
    assert_eq!(9, s4.width(EastAsianContextCharWidthSelector));
    assert_eq!(8, s4.width(NonEastAsianContextCharWidthSelector));

    let s5 = "ＡｱあAα𓄿";
    assert_eq!(9, s5.width(EastAsianContextCharWidthSelector));
    assert_eq!(8, s5.width(NonEastAsianContextCharWidthSelector));

    let s6 = "ＡｱあA*𓄿";
    assert_eq!(8, s6.width(EastAsianContextCharWidthSelector));
    assert_eq!(8, s6.width(NonEastAsianContextCharWidthSelector));

    let s7 = "ＡｱあA*α";
    assert_eq!(9, s7.width(EastAsianContextCharWidthSelector));
    assert_eq!(8, s7.width(NonEastAsianContextCharWidthSelector));
}
