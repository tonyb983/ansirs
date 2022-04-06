//! # Ansirs
//!
//! Simple library for working with ANSI escape codes to add pretty colors to your shitty console text.
//!
//! Copyright (c) 2022 Tony Barbitta
//!
//! This Source Code Form is subject to the terms of the Mozilla Public
//! License, v. 2.0. If a copy of the MPL was not distributed with this
//! file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)]

mod ansi;
mod color;
mod styled;

pub mod iter {
    pub use crate::color::iter::*;
}

pub use ansi::*;
pub use color::*;
pub use styled::*;

#[cfg(test)]
mod tests {
    use super::*;

    const DISPLAY_PRE: &str = "\u{1b}[";
    const DISPLAY_SUF: &str = "m";

    fn empty_style_function() -> Ansi {
        Ansi::new()
    }

    #[test]
    fn storing_styles() {
        let style1 = Ansi::new().fg((100, 200, 100)).underline();
        let style2 = Ansi::new().bg((0, 0, 75)).italic().strike();

        // println!("{}Here is text styled by #1.{}", style1, Ansi::reset());
        // println!("{}Here is text styled by #2.{}", style2, Ansi::reset());
        // println!("{}Here is more text styled by #1.{}", style1, Ansi::reset());

        assert_eq!(
            style1.to_string(),
            format!("{}4;38;2;100;200;100{}", DISPLAY_PRE, DISPLAY_SUF)
        );
        assert_eq!(
            style2.to_string(),
            format!("{}3;9;48;2;0;0;75{}", DISPLAY_PRE, DISPLAY_SUF)
        );
        assert_eq!(
            style1.to_string(),
            format!("{}4;38;2;100;200;100{}", DISPLAY_PRE, DISPLAY_SUF)
        );
        assert_eq!(
            style2.to_string(),
            format!("{}3;9;48;2;0;0;75{}", DISPLAY_PRE, DISPLAY_SUF)
        );
    }

    #[test]
    fn style_text_basic() {
        let first = "first".to_string();
        let unstyled_val = style_text(&first, Ansi::new());
        assert_eq!(unstyled_val, first);
        let unstyled_fn = style_text(&first, empty_style_function);
        assert_eq!(unstyled_fn, first);

        let manual_prefix = format!("{}{}{}", DISPLAY_PRE, "4;38;2;255;0;0", DISPLAY_SUF);
        let manual_suffix = format!("{}{}{}", DISPLAY_PRE, "0", DISPLAY_SUF);
        let manual = format!("{}{}{}", manual_prefix, first, manual_suffix);

        let styled_value = style_text(&first, Ansi::red().underline());

        assert_eq!(styled_value, manual);
    }

    #[test]
    fn style_text_inputs() {
        let first = "first".to_string();

        let st = style_text(&first, Ansi::new());
        let sf = style_text(&first, empty_style_function);
        let sc = style_text(&first, || {
            let style = Ansi::new()
                .underline()
                .italic()
                .fg((200, 100, 200))
                .bg((255, 255, 255));

            style.strike()
        });
        let _scols = style_text(&first, Colors::Yellow);
        let _scols = style_text(&first, Colors::Yellow.into_ansi());
        let yellow = Colors::Yellow.into_color();
        let _scol = style_text(&first, yellow.into_ansi());

        let manual_prefix = format!(
            "{}{}{}",
            DISPLAY_PRE, "3;4;9;38;2;200;100;200;48;2;255;255;255", DISPLAY_SUF
        );
        let manual_suffix = format!("{}{}{}", DISPLAY_PRE, "0", DISPLAY_SUF);
        let third = format!("{}{}{}", manual_prefix, first, manual_suffix);

        assert_eq!(&st, &first);
        assert_eq!(&sf, &first);
        assert_eq!(&sc, &third);
    }

    #[test]
    fn color_inputs() {
        let _red = Ansi::from_fg(Colors::Red);
        let _green = Ansi::from_fg((0, 255, 0));
        let _blue = Ansi::from_fg(Color::from_hex("#0000ff").unwrap());
    }
}
