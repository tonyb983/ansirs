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
    pub use super::colors::iter::*;
}

pub use color::Color;
pub use colors::Colors;
pub use error::ColorParseError;
pub use traits::*;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
    fn hex_convert_unicode() {
        let color1 = Color::from_hex("#💜💙💚💛💚💙💜");
        assert!(color1.is_err());
        let color2 = Color::from_hex("#1💜");
        println!("{color2:?}");
        assert!(color2.is_err());
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
