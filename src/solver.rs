use crate::common::EastAsianWidth;
use crate::common::EastAsianWidth::*;

pub fn solve_eaw(code_point: u32) -> EastAsianWidth {
    if code_point < 0x25E6 {
        if code_point < 0x203B {
            if code_point < 0x16C {
                if code_point < 0xF2 {
                    if code_point < 0xBC {
                        if code_point < 0xAA {
                            if code_point < 0xA2 {
                                if code_point < 0x7F {
                                    if code_point < 0x20 {
                                        return Neutral;
                                    } else {
                                        return Narrow;
                                    }
                                } else {
                                    if code_point < 0xA1 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0xA5 {
                                    if code_point < 0xA4 {
                                        return Narrow;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0xA7 {
                                        return Narrow;
                                    } else {
                                        if code_point < 0xA9 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0xAF {
                                if code_point < 0xAC {
                                    if code_point < 0xAB {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0xAD {
                                        return Narrow;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0xB5 {
                                    if code_point < 0xB0 {
                                        return Narrow;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0xB6 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0xBB {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0xE2 {
                            if code_point < 0xD0 {
                                if code_point < 0xC6 {
                                    if code_point < 0xC0 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0xC7 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0xD7 {
                                    if code_point < 0xD1 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0xD9 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0xDE {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0xEB {
                                if code_point < 0xE7 {
                                    if code_point < 0xE6 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0xE8 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0xEE {
                                    if code_point < 0xEC {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0xF0 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0xF1 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if code_point < 0x12B {
                        if code_point < 0x102 {
                            if code_point < 0xFC {
                                if code_point < 0xF7 {
                                    if code_point < 0xF4 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0xFB {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0xFE {
                                    if code_point < 0xFD {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0xFF {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x101 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x114 {
                                if code_point < 0x112 {
                                    if code_point < 0x111 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x113 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x11C {
                                    if code_point < 0x11B {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x126 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x128 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x145 {
                            if code_point < 0x138 {
                                if code_point < 0x131 {
                                    if code_point < 0x12C {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x134 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x13F {
                                    if code_point < 0x139 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x143 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x144 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x152 {
                                if code_point < 0x14C {
                                    if code_point < 0x148 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x14D {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x14E {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0x166 {
                                    if code_point < 0x154 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x168 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x16B {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if code_point < 0x2E0 {
                    if code_point < 0x252 {
                        if code_point < 0x1D6 {
                            if code_point < 0x1D1 {
                                if code_point < 0x1CF {
                                    if code_point < 0x1CE {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x1D0 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x1D3 {
                                    if code_point < 0x1D2 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x1D4 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x1D5 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x1DA {
                                if code_point < 0x1D8 {
                                    if code_point < 0x1D7 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1D9 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x1DC {
                                    if code_point < 0x1DB {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1DD {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x251 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x2CD {
                            if code_point < 0x2C5 {
                                if code_point < 0x262 {
                                    if code_point < 0x261 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2C4 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x2C8 {
                                    if code_point < 0x2C7 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2C9 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2CC {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x2D8 {
                                if code_point < 0x2D0 {
                                    if code_point < 0x2CE {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2D1 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x2DD {
                                    if code_point < 0x2DC {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2DE {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x2DF {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if code_point < 0x1160 {
                        if code_point < 0x3C3 {
                            if code_point < 0x3A2 {
                                if code_point < 0x370 {
                                    if code_point < 0x300 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x391 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x3AA {
                                    if code_point < 0x3A3 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x3B1 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x3C2 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x410 {
                                if code_point < 0x401 {
                                    if code_point < 0x3CA {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x402 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x451 {
                                    if code_point < 0x450 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x452 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x1100 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x2020 {
                            if code_point < 0x2017 {
                                if code_point < 0x2011 {
                                    if code_point < 0x2010 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2013 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x201A {
                                    if code_point < 0x2018 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x201C {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x201E {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x2031 {
                                if code_point < 0x2024 {
                                    if code_point < 0x2023 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2028 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x2030 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0x2034 {
                                    if code_point < 0x2032 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2035 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2036 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if code_point < 0x222D {
                if code_point < 0x2170 {
                    if code_point < 0x2109 {
                        if code_point < 0x2085 {
                            if code_point < 0x2074 {
                                if code_point < 0x203E {
                                    if code_point < 0x203C {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x203F {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x207F {
                                    if code_point < 0x2075 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2080 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x2081 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x20AD {
                                if code_point < 0x20AA {
                                    if code_point < 0x20A9 {
                                        return Neutral;
                                    } else {
                                        return Half;
                                    }
                                } else {
                                    if code_point < 0x20AC {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x2104 {
                                    if code_point < 0x2103 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2105 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2106 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x2127 {
                            if code_point < 0x2116 {
                                if code_point < 0x2113 {
                                    if code_point < 0x210A {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2114 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x2121 {
                                    if code_point < 0x2117 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2123 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x2126 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x2155 {
                                if code_point < 0x212C {
                                    if code_point < 0x212B {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2153 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x215F {
                                    if code_point < 0x215B {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2160 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x216C {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if code_point < 0x2207 {
                        if code_point < 0x21D3 {
                            if code_point < 0x2190 {
                                if code_point < 0x2189 {
                                    if code_point < 0x217A {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x218A {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x21B8 {
                                    if code_point < 0x219A {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x21BA {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x21D2 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x21E8 {
                                if code_point < 0x21D5 {
                                    if code_point < 0x21D4 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x21E7 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x2201 {
                                    if code_point < 0x2200 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2202 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2204 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x2216 {
                            if code_point < 0x220F {
                                if code_point < 0x220B {
                                    if code_point < 0x2209 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x220C {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x2211 {
                                    if code_point < 0x2210 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2212 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x2215 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x2223 {
                                if code_point < 0x221B {
                                    if code_point < 0x221A {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x221D {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2221 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0x2225 {
                                    if code_point < 0x2224 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2226 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x2227 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if code_point < 0x2329 {
                    if code_point < 0x226C {
                        if code_point < 0x224C {
                            if code_point < 0x2238 {
                                if code_point < 0x222F {
                                    if code_point < 0x222E {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2234 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x223E {
                                    if code_point < 0x223C {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2248 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2249 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x2260 {
                                if code_point < 0x2252 {
                                    if code_point < 0x224D {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2253 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x2264 {
                                    if code_point < 0x2262 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2268 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x226A {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x2299 {
                            if code_point < 0x2284 {
                                if code_point < 0x2270 {
                                    if code_point < 0x226E {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2282 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x2288 {
                                    if code_point < 0x2286 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2295 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2296 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x22C0 {
                                if code_point < 0x22A5 {
                                    if code_point < 0x229A {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x22A6 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x22BF {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0x2313 {
                                    if code_point < 0x2312 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x231A {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x231C {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if code_point < 0x25A0 {
                        if code_point < 0x24EA {
                            if code_point < 0x23F0 {
                                if code_point < 0x23E9 {
                                    if code_point < 0x232B {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x23ED {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x23F3 {
                                    if code_point < 0x23F1 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x23F4 {
                                        return Wide;
                                    } else {
                                        if code_point < 0x2460 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x2574 {
                                if code_point < 0x254C {
                                    if code_point < 0x24EB {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2550 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x2590 {
                                    if code_point < 0x2580 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2592 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2596 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x25BE {
                            if code_point < 0x25B2 {
                                if code_point < 0x25A3 {
                                    if code_point < 0x25A2 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x25AA {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x25B6 {
                                    if code_point < 0x25B4 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x25B8 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x25BC {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x25CB {
                                if code_point < 0x25C2 {
                                    if code_point < 0x25C0 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x25C6 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x25C9 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0x25CE {
                                    if code_point < 0x25CC {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x25D2 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x25E2 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        if code_point < 0xFE68 {
            if code_point < 0x273D {
                if code_point < 0x2694 {
                    if code_point < 0x2641 {
                        if code_point < 0x260E {
                            if code_point < 0x25FF {
                                if code_point < 0x25F0 {
                                    if code_point < 0x25EF {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x25FD {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x2607 {
                                    if code_point < 0x2605 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2609 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x260A {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x261C {
                                if code_point < 0x2614 {
                                    if code_point < 0x2610 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2616 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x261E {
                                    if code_point < 0x261D {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x261F {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x2640 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x2667 {
                            if code_point < 0x2654 {
                                if code_point < 0x2643 {
                                    if code_point < 0x2642 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2648 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x2662 {
                                    if code_point < 0x2660 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2663 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2666 {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x266F {
                                if code_point < 0x266C {
                                    if code_point < 0x266B {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x266E {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x267F {
                                    if code_point < 0x2670 {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2680 {
                                        return Wide;
                                    } else {
                                        if code_point < 0x2693 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if code_point < 0x26E4 {
                        if code_point < 0x26C0 {
                            if code_point < 0x26A2 {
                                if code_point < 0x26A0 {
                                    if code_point < 0x269E {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x26A1 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x26AC {
                                    if code_point < 0x26AA {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x26BD {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x26BF {
                                            return Wide;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x26CF {
                                if code_point < 0x26C6 {
                                    if code_point < 0x26C4 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x26CE {
                                        return Ambiguous;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x26D5 {
                                    if code_point < 0x26D4 {
                                        return Ambiguous;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x26E2 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x26E3 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x26FB {
                            if code_point < 0x26F2 {
                                if code_point < 0x26EA {
                                    if code_point < 0x26E8 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x26EB {
                                        return Wide;
                                    } else {
                                        return Ambiguous;
                                    }
                                }
                            } else {
                                if code_point < 0x26F5 {
                                    if code_point < 0x26F4 {
                                        return Wide;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x26F6 {
                                        return Wide;
                                    } else {
                                        if code_point < 0x26FA {
                                            return Ambiguous;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x2706 {
                                if code_point < 0x26FE {
                                    if code_point < 0x26FD {
                                        return Ambiguous;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x2700 {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x2705 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0x270C {
                                    if code_point < 0x270A {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x2728 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2729 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if code_point < 0x3000 {
                    if code_point < 0x27E6 {
                        if code_point < 0x2758 {
                            if code_point < 0x274E {
                                if code_point < 0x274C {
                                    if code_point < 0x273E {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x274D {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x2753 {
                                    if code_point < 0x274F {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2756 {
                                        return Wide;
                                    } else {
                                        if code_point < 0x2757 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x2798 {
                                if code_point < 0x2780 {
                                    if code_point < 0x2776 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x2795 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x27B1 {
                                    if code_point < 0x27B0 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x27BF {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x27C0 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x2B56 {
                            if code_point < 0x2B1B {
                                if code_point < 0x2985 {
                                    if code_point < 0x27EE {
                                        return Narrow;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2987 {
                                        return Narrow;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x2B50 {
                                    if code_point < 0x2B1D {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2B51 {
                                        return Wide;
                                    } else {
                                        if code_point < 0x2B55 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x2EF4 {
                                if code_point < 0x2E80 {
                                    if code_point < 0x2B5A {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x2E9A {
                                        return Wide;
                                    } else {
                                        if code_point < 0x2E9B {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0x2FD6 {
                                    if code_point < 0x2F00 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x2FF0 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x2FFC {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if code_point < 0x4DC0 {
                        if code_point < 0x3131 {
                            if code_point < 0x3097 {
                                if code_point < 0x303F {
                                    if code_point < 0x3001 {
                                        return Full;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x3041 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x3100 {
                                    if code_point < 0x3099 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x3105 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x3130 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x31F0 {
                                if code_point < 0x3190 {
                                    if code_point < 0x318F {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x31E4 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x3220 {
                                    if code_point < 0x321F {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x3248 {
                                        return Wide;
                                    } else {
                                        if code_point < 0x3250 {
                                            return Ambiguous;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0xE000 {
                            if code_point < 0xA4C7 {
                                if code_point < 0xA48D {
                                    if code_point < 0x4E00 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0xA490 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0xA97D {
                                    if code_point < 0xA960 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0xAC00 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0xD7A4 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0xFE1A {
                                if code_point < 0xFB00 {
                                    if code_point < 0xF900 {
                                        return Ambiguous;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0xFE00 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0xFE10 {
                                            return Ambiguous;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0xFE53 {
                                    if code_point < 0xFE30 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0xFE54 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0xFE67 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if code_point < 0x1F3CF {
                if code_point < 0x1B2FC {
                    if code_point < 0xFFFE {
                        if code_point < 0xFFD2 {
                            if code_point < 0xFFBF {
                                if code_point < 0xFF01 {
                                    if code_point < 0xFE6C {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0xFF61 {
                                        return Full;
                                    } else {
                                        return Half;
                                    }
                                }
                            } else {
                                if code_point < 0xFFC8 {
                                    if code_point < 0xFFC2 {
                                        return Neutral;
                                    } else {
                                        return Half;
                                    }
                                } else {
                                    if code_point < 0xFFCA {
                                        return Neutral;
                                    } else {
                                        if code_point < 0xFFD0 {
                                            return Half;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0xFFE0 {
                                if code_point < 0xFFDA {
                                    if code_point < 0xFFD8 {
                                        return Half;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0xFFDD {
                                        return Half;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0xFFE8 {
                                    if code_point < 0xFFE7 {
                                        return Full;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0xFFEF {
                                        return Half;
                                    } else {
                                        if code_point < 0xFFFD {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x18D00 {
                            if code_point < 0x16FF2 {
                                if code_point < 0x16FE5 {
                                    if code_point < 0x16FE0 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x16FF0 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x187F8 {
                                    if code_point < 0x17000 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x18800 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x18CD6 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x1B150 {
                                if code_point < 0x1B000 {
                                    if code_point < 0x18D09 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1B11F {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x1B164 {
                                    if code_point < 0x1B153 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1B168 {
                                        return Wide;
                                    } else {
                                        if code_point < 0x1B170 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if code_point < 0x1F203 {
                        if code_point < 0x1F130 {
                            if code_point < 0x1F0D0 {
                                if code_point < 0x1F005 {
                                    if code_point < 0x1F004 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1F0CF {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x1F10B {
                                    if code_point < 0x1F100 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x1F110 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x1F12E {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x1F18F {
                                if code_point < 0x1F170 {
                                    if code_point < 0x1F16A {
                                        return Ambiguous;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1F18E {
                                        return Ambiguous;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x1F19B {
                                    if code_point < 0x1F191 {
                                        return Ambiguous;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1F1AD {
                                        return Ambiguous;
                                    } else {
                                        if code_point < 0x1F200 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x1F300 {
                            if code_point < 0x1F249 {
                                if code_point < 0x1F23C {
                                    if code_point < 0x1F210 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1F240 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x1F252 {
                                    if code_point < 0x1F250 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1F260 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x1F266 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x1F37D {
                                if code_point < 0x1F32D {
                                    if code_point < 0x1F321 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1F336 {
                                        return Wide;
                                    } else {
                                        if code_point < 0x1F337 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0x1F394 {
                                    if code_point < 0x1F37E {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1F3A0 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x1F3CB {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if code_point < 0x1F6FD {
                    if code_point < 0x1F57A {
                        if code_point < 0x1F441 {
                            if code_point < 0x1F3F4 {
                                if code_point < 0x1F3E0 {
                                    if code_point < 0x1F3D4 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1F3F1 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x1F3F8 {
                                    if code_point < 0x1F3F5 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1F43F {
                                        return Wide;
                                    } else {
                                        if code_point < 0x1F440 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x1F53E {
                                if code_point < 0x1F4FD {
                                    if code_point < 0x1F442 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1F4FF {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x1F54F {
                                    if code_point < 0x1F54B {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1F550 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x1F568 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x1F6C6 {
                            if code_point < 0x1F5A4 {
                                if code_point < 0x1F595 {
                                    if code_point < 0x1F57B {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1F597 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x1F5FB {
                                    if code_point < 0x1F5A5 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1F650 {
                                        return Wide;
                                    } else {
                                        if code_point < 0x1F680 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x1F6D5 {
                                if code_point < 0x1F6CD {
                                    if code_point < 0x1F6CC {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1F6D0 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x1F6D3 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0x1F6EB {
                                    if code_point < 0x1F6D8 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1F6ED {
                                        return Wide;
                                    } else {
                                        if code_point < 0x1F6F4 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if code_point < 0x1FA87 {
                        if code_point < 0x1F97A {
                            if code_point < 0x1F93B {
                                if code_point < 0x1F7EC {
                                    if code_point < 0x1F7E0 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1F90C {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x1F946 {
                                    if code_point < 0x1F93C {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1F947 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x1F979 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0x1FA70 {
                                if code_point < 0x1F9CD {
                                    if code_point < 0x1F9CC {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1FA00 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                }
                            } else {
                                if code_point < 0x1FA78 {
                                    if code_point < 0x1FA75 {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x1FA7B {
                                        return Wide;
                                    } else {
                                        if code_point < 0x1FA80 {
                                            return Neutral;
                                        } else {
                                            return Wide;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if code_point < 0x20000 {
                            if code_point < 0x1FAB7 {
                                if code_point < 0x1FAA9 {
                                    if code_point < 0x1FA90 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1FAB0 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                }
                            } else {
                                if code_point < 0x1FAC3 {
                                    if code_point < 0x1FAC0 {
                                        return Neutral;
                                    } else {
                                        return Wide;
                                    }
                                } else {
                                    if code_point < 0x1FAD0 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x1FAD7 {
                                            return Wide;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        } else {
                            if code_point < 0xE01F0 {
                                if code_point < 0x30000 {
                                    if code_point < 0x2FFFE {
                                        return Wide;
                                    } else {
                                        return Neutral;
                                    }
                                } else {
                                    if code_point < 0x3FFFE {
                                        return Wide;
                                    } else {
                                        if code_point < 0xE0100 {
                                            return Neutral;
                                        } else {
                                            return Ambiguous;
                                        }
                                    }
                                }
                            } else {
                                if code_point < 0xFFFFE {
                                    if code_point < 0xF0000 {
                                        return Neutral;
                                    } else {
                                        return Ambiguous;
                                    }
                                } else {
                                    if code_point < 0x100000 {
                                        return Neutral;
                                    } else {
                                        if code_point < 0x10FFFE {
                                            return Ambiguous;
                                        } else {
                                            return Neutral;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
