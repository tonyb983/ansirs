// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Private module so who cares
#[allow(clippy::module_inception)]
mod color;
mod colors;
mod error;
mod traits;

pub mod iter {
    use super::Colors;

    pub struct ColorsIter {
        current: Option<Colors>,
    }

    impl Default for ColorsIter {
        fn default() -> Self {
            Self {
                current: Some(Self::FIRST),
            }
        }
    }

    impl ColorsIter {
        const FIRST: Colors = Colors::AliceBlue;
        const LAST: Colors = Colors::YellowGreen;

        pub fn new() -> Self {
            Default::default()
        }

        pub fn starting_with(color: Colors) -> Self {
            Self {
                current: Some(color),
            }
        }
    }

    impl Iterator for ColorsIter {
        type Item = Colors;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.current.take();
            self.current = current.and_then(|c| {
                let next = c.next();
                if next == Self::FIRST {
                    None
                } else {
                    Some(next)
                }
            });

            current
        }
    }
}

pub use color::Color;
pub use colors::Colors;
pub use error::ColorParseError;
pub use traits::ToColor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_convert_1() {
        let color1 = Color::from_hex("#FF0000").unwrap();
        let color2 = Color::from_hex("FF0000").unwrap();
        let color3 = Color::from_hex("#f00").unwrap();
        assert_eq!(color1.rgb(), (255, 0, 0));
        assert_eq!(color2.rgb(), (255, 0, 0));
        assert_eq!(color3.rgb(), (255, 0, 0));
    }

    #[test]
    #[should_panic]
    fn hex_convert_too_small_panics() {
        let _ = Color::from_hex("#FF00").unwrap();
    }

    #[test]
    #[should_panic]
    fn hex_convert_too_big_panics() {
        let _ = Color::from_hex("#FF00000").unwrap();
    }

    #[test]
    #[should_panic]
    fn hex_convert_bad_char_panics() {
        let _ = Color::from_hex("#FF000G").unwrap();
    }
}
