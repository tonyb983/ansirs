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
// #![feature(const_mut_refs)]

use bitflags::bitflags;

bitflags! {
    /// A set of flags to determine text style with ANSI color codes.
    #[derive(Default)]
    pub struct AnsiFlags: u32 {
        /// Bold text.
        const BOLD = 1 << 1;
        /// Underlined text.
        const UNDERLINE = 1 << 2;
        /// Italicized text.
        const ITALIC = 1 << 3;
        /// Blinking text.
        const BLINK = 1 << 4;
        /// Reversed / Inverted text.
        const REVERSE = 1 << 5;
        /// Striken text.
        const STRIKE = 1 << 6;
    }
}

/// Alias for a tuple of 3 bytes representing R,G,B values.
pub type Rgb = (u8, u8, u8);

/// Type for storing the configuration of an ANSI color code.
///
/// ## Example(s)
///
/// ### Lambda Usage (Recommended):
/// ```
/// # use ansirs::{Ansi, IntoAnsi, style_text};
///
/// let body_style = Ansi::new().fg((100, 200, 100));
/// let head_style = Ansi::new().fg((50, 250, 50)).bold().underline();
///
/// let header = style_text("Some Header", head_style);
/// let body = style_text("Here is the text for my fictional body of whatever-the-fuck. Super cool.", body_style);
///
/// // println!("{}", header);
/// // println!("{}", body);
///
/// # assert_eq!(header, "\x1b[1;4;38;2;50;250;50mSome Header\x1b[0m");
/// # assert_eq!(body, "\x1b[38;2;100;200;100mHere is the text for my fictional body of whatever-the-fuck. Super cool.\x1b[0m");
/// ```
///
/// ### Raw Usage:
/// ```
/// # use ansirs::{Ansi, AnsiFlags, IntoAnsi};
/// let style1 = Ansi::new().fg((100, 200, 100)).underline();
/// let style2 = Ansi::new().bg((0, 0, 75)).italic().strike();
///
/// // println!("{}Here is text styled by #1.{}", style1, Ansi::reset());
/// // println!("{}Here is text styled by #2.{}", style2, Ansi::reset());
/// // println!("{}Here is more text styled by #1.{}", style1, Ansi::reset());
/// # assert_eq!(style1.to_string(), "\x1b[4;38;2;100;200;100m");
/// # assert_eq!(style2.to_string(), "\x1b[3;9;48;2;0;0;75m");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ansi {
    fg: Option<Rgb>,
    bg: Option<Rgb>,
    flags: AnsiFlags,
}

// "Static" Methods
impl Ansi {
    const PREFIX: &'static str = "\x1b[";
    const SUFFIX: &'static str = "m";

    /// Creates a new / empty / default Ansi instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            flags: AnsiFlags::empty(),
        }
    }

    /// Creates a new Ansi from the given foreground color.
    #[must_use]
    pub const fn from_fg(fg: Rgb) -> Self {
        Self {
            fg: Some(fg),
            bg: None,
            flags: AnsiFlags::empty(),
        }
    }

    /// Creates a new Ansi from the given background color.
    #[must_use]
    pub const fn from_bg(bg: Rgb) -> Self {
        Self {
            fg: None,
            bg: Some(bg),
            flags: AnsiFlags::empty(),
        }
    }

    /// Creates a new Ansi with a red foreground color.
    #[must_use]
    pub const fn red() -> Self {
        Self::from_fg((255, 0, 0))
    }

    /// Creates a new Ansi with a green foreground color.
    #[must_use]
    pub const fn green() -> Self {
        Self::from_fg((0, 255, 0))
    }

    /// Creates a new Ansi with a blue foreground color.
    #[must_use]
    pub const fn blue() -> Self {
        Self::from_fg((0, 0, 255))
    }

    /// Reset the terminal to default styling.
    #[must_use]
    pub const fn reset() -> &'static str {
        "\x1b[0m"
    }
}

// Member functions
impl Ansi {
    /// Clear the Ansi object entirely.
    #[must_use]
    #[allow(clippy::needless_update)]
    pub const fn clear(self) -> Self {
        Self {
            fg: None,
            bg: None,
            flags: AnsiFlags::empty(),
            ..self
        }
    }

    /// Returns `true` if this `Ansi` has no styling.
    #[must_use]
    pub const fn is_default(&self) -> bool {
        self.fg.is_none() && self.bg.is_none() && self.flags.is_empty()
    }

    /// Builder function to set the foreground color.
    #[must_use]
    pub const fn fg(self, fg: Rgb) -> Self {
        Self {
            fg: Some(fg),
            ..self
        }
    }

    /// Builder function to clear the foreground color.
    #[must_use]
    pub const fn clear_fg(self) -> Self {
        Self { fg: None, ..self }
    }

    /// Builder function to set the background color.
    #[must_use]
    pub const fn bg(self, bg: Rgb) -> Self {
        Self {
            bg: Some(bg),
            ..self
        }
    }

    /// Builder function to clear the foreground color.
    #[must_use]
    pub const fn clear_bg(self) -> Self {
        Self { bg: None, ..self }
    }

    /// Builder function to toggle whether the color is bold.
    #[must_use]
    pub fn bold(self) -> Self {
        let mut flags = self.flags;
        flags.toggle(AnsiFlags::BOLD);
        Self { flags, ..self }
    }

    /// Builder function to toggle whether the color is underlined.
    #[must_use]
    pub fn underline(self) -> Self {
        let mut flags = self.flags;
        flags.toggle(AnsiFlags::UNDERLINE);
        Self { flags, ..self }
    }

    /// Builder function to toggle whether the color is italic.
    #[must_use]
    pub fn italic(self) -> Self {
        let mut flags = self.flags;
        flags.toggle(AnsiFlags::ITALIC);
        Self { flags, ..self }
    }

    /// Builder function to toggle whether the color is blinking.
    #[must_use]
    pub fn blink(self) -> Self {
        let mut flags = self.flags;
        flags.toggle(AnsiFlags::BLINK);
        Self { flags, ..self }
    }

    /// Builder function to toggle whether the color is inversed / reversed.
    #[must_use]
    pub fn reverse(self) -> Self {
        let mut flags = self.flags;
        flags.toggle(AnsiFlags::REVERSE);
        Self { flags, ..self }
    }

    /// Builder function to toggle whether the color is striked.
    #[must_use]
    pub fn strike(self) -> Self {
        let mut flags = self.flags;
        flags.toggle(AnsiFlags::STRIKE);
        Self { flags, ..self }
    }

    /// Creates a string from this `Ansi` using a `String` to store temporary data.
    #[must_use]
    fn build_string(&self) -> String {
        if self.is_default() {
            return "".to_string();
        }

        let mut modified = false;
        let mut ansi = String::with_capacity(20);

        if self.flags.contains(AnsiFlags::BOLD) {
            ansi.push('1');
            modified = true;
        }

        if self.flags.contains(AnsiFlags::ITALIC) {
            if modified {
                ansi.push_str(";3");
            } else {
                ansi.push('3');
                modified = true;
            }
        }

        if self.flags.contains(AnsiFlags::UNDERLINE) {
            if modified {
                ansi.push_str(";4");
            } else {
                ansi.push('4');
                modified = true;
            }
        }

        if self.flags.contains(AnsiFlags::BLINK) {
            if modified {
                ansi.push_str(";5");
            } else {
                ansi.push('5');
                modified = true;
            }
        }

        if self.flags.contains(AnsiFlags::REVERSE) {
            if modified {
                ansi.push_str(";7");
            } else {
                ansi.push('7');
                modified = true;
            }
        }

        if self.flags.contains(AnsiFlags::STRIKE) {
            if modified {
                ansi.push_str(";9");
            } else {
                ansi.push('9');
                modified = true;
            }
        }

        if let Some((r, g, b)) = self.fg {
            if modified {
                ansi.push_str(";38;2;");
            } else {
                ansi.push_str("38;2;");
            }
            ansi.push_str(&format!("{};{};{}", r, g, b));
            modified = true;
        }

        if let Some((r, g, b)) = self.bg {
            if modified {
                ansi.push_str(";48;2;");
            } else {
                ansi.push_str("48;2;");
            }
            ansi.push_str(&format!("{};{};{}", r, g, b));
            modified = true;
        }

        if !modified {
            return "".to_string();
        }

        format!("{}{}{}", Self::PREFIX, ansi, Self::SUFFIX)
    }
}

impl Default for Ansi {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build_string())
    }
}

/// Trait used to enable style functions to accept value or closure.
#[allow(clippy::module_name_repetitions)]
pub trait IntoAnsi {
    /// Performs the conversion.
    fn into_ansi(self) -> Ansi;
}

impl<T> IntoAnsi for T
where
    T: Fn() -> Ansi,
{
    fn into_ansi(self) -> Ansi {
        self()
    }
}

impl IntoAnsi for Ansi {
    fn into_ansi(self) -> Ansi {
        self
    }
}

/// Styles the given [`Display`](std::fmt::Display) using the style described by `style`.
/// `S` can be either an [`Ansi`](Ansi) or a closure that returns an [`Ansi`](Ansi). This might
/// require bringing the [`IntoAnsi`](IntoAnsi) trait into scope.
pub fn style_text<S: IntoAnsi>(text: impl std::fmt::Display, style: S) -> String {
    let actual = format!("{}", text);

    if actual.is_empty() {
        actual
    } else {
        let ansi = style.into_ansi();
        if ansi.is_default() {
            actual
        } else {
            format!("{}{}{}", ansi, text, Ansi::reset())
        }
    }
}

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
}
