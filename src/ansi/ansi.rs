// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{AnsiFlags, Color, ToColor};

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
    fg: Option<Color>,
    bg: Option<Color>,
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
    pub fn from_fg(fg: impl ToColor) -> Self {
        Self {
            fg: Some(fg.to_color()),
            bg: None,
            flags: AnsiFlags::empty(),
        }
    }

    /// Creates a new Ansi from the given background color.
    #[must_use]
    pub fn from_bg(bg: impl ToColor) -> Self {
        Self {
            fg: None,
            bg: Some(bg.to_color()),
            flags: AnsiFlags::empty(),
        }
    }

    /// Creates a new Ansi with a red foreground color.
    #[must_use]
    pub fn red() -> Self {
        Self::from_fg((255, 0, 0))
    }

    /// Creates a new Ansi with a green foreground color.
    #[must_use]
    pub fn green() -> Self {
        Self::from_fg((0, 255, 0))
    }

    /// Creates a new Ansi with a blue foreground color.
    #[must_use]
    pub fn blue() -> Self {
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
    pub fn fg(self, fg: impl ToColor) -> Self {
        Self {
            fg: Some(fg.to_color()),
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
    pub fn bg(self, bg: impl ToColor) -> Self {
        Self {
            bg: Some(bg.to_color()),
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
    pub const fn bold(self) -> Self {
        Self {
            flags: self.flags.toggle_to(AnsiFlags::BOLD),
            ..self
        }
    }

    /// Builder function to toggle whether the color is underlined.
    #[must_use]
    pub const fn underline(self) -> Self {
        Self {
            flags: self.flags.toggle_to(AnsiFlags::UNDERLINE),
            ..self
        }
    }

    /// Builder function to toggle whether the color is italic.
    #[must_use]
    pub fn italic(self) -> Self {
        Self {
            flags: self.flags.toggle_to(AnsiFlags::ITALIC),
            ..self
        }
    }

    /// Builder function to toggle whether the color is blinking.
    #[must_use]
    pub const fn blink(self) -> Self {
        Self {
            flags: self.flags.toggle_to(AnsiFlags::BLINK),
            ..self
        }
    }

    /// Builder function to toggle whether the color is inverted / reversed.
    #[must_use]
    pub fn reverse(self) -> Self {
        Self {
            flags: self.flags.toggle_to(AnsiFlags::REVERSE),
            ..self
        }
    }

    /// Builder function to toggle whether the color is strike-d.
    #[must_use]
    pub fn strike(self) -> Self {
        Self {
            flags: self.flags.toggle_to(AnsiFlags::STRIKE),
            ..self
        }
    }

    /// Creates a string from this `Ansi` using a `String` to store temporary data.
    /// TODO: This can probably be renamed to `build` now. Previously I was testing
    ///       performance of using a [`Vec`] vs a [`String`] to store the temporarily
    ///       store the data, but [`String`] won hands down.
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

        if let Some(color) = self.fg {
            let (r, g, b) = color.rgb();
            if modified {
                ansi.push_str(";38;2;");
            } else {
                ansi.push_str("38;2;");
            }
            ansi.push_str(&format!("{};{};{}", r, g, b));
            modified = true;
        }

        if let Some(c) = self.bg {
            let (r, g, b) = c.rgb();
            if modified {
                ansi.push_str(";48;2;");
            } else {
                ansi.push_str("48;2;");
            }
            ansi.push_str(&format!("{};{};{}", r, g, b));
            modified = true;
        }

        // This seems like it will be unnecessary, I can't even get the branch to hit during testing.
        if !modified {
            return "".to_string();
        }

        format!("{}{}{}", Self::PREFIX, ansi, Self::SUFFIX)
    }

    /// Convenience function that uses this [`Ansi`] to style the given [`text`],
    /// sandwiching the text between the color code generated by this [`Ansi`] and
    /// [`Ansi::reset`].
    pub fn paint_text(&self, text: &str) -> String {
        if self.is_default() {
            return text.to_string();
        }

        format!("{}{}{}", self.build_string(), text, Self::reset())
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn static_colors() {
        let r = Ansi::red();
        let g = Ansi::green();
        let b = Ansi::blue();

        assert!(r.to_string().contains("255;0;0"));
        assert!(g.to_string().contains("0;255;0"));
        assert!(b.to_string().contains("0;0;255"));
    }

    #[test]
    fn the_works() {
        let mut ansi = Ansi::new()
            .fg((50, 250, 150))
            .bg((25, 25, 25))
            .bold()
            .blink()
            .italic()
            .reverse()
            .strike()
            .underline();

        assert_eq!(
            ansi.to_string(),
            "\u{1b}[1;3;4;5;7;9;38;2;50;250;150;48;2;25;25;25m"
        );

        ansi = ansi.clear();
        assert!(ansi.to_string().is_empty());
    }

    #[test]
    fn default_is_empty() {
        let ansi = Ansi::default();
        assert!(ansi.to_string().is_empty());
    }

    #[test]
    fn solo_styles() {
        let a = Ansi::new().blink();
        assert_eq!(a.to_string(), "\u{1b}[5m");
        let a = Ansi::new().bold();
        assert_eq!(a.to_string(), "\u{1b}[1m");
        let a = Ansi::new().italic();
        assert_eq!(a.to_string(), "\u{1b}[3m");
        let a = Ansi::new().underline();
        assert_eq!(a.to_string(), "\u{1b}[4m");
        let a = Ansi::new().reverse();
        assert_eq!(a.to_string(), "\u{1b}[7m");
        let a = Ansi::new().strike();
        assert_eq!(a.to_string(), "\u{1b}[9m");
        let a = Ansi::from_fg((255, 255, 255));
        assert_eq!(a.to_string(), "\u{1b}[38;2;255;255;255m");
        let a = Ansi::from_bg((255, 255, 255));
        assert_eq!(a.to_string(), "\u{1b}[48;2;255;255;255m");
    }

    #[test]
    fn set_and_clear() {
        let mut a = Ansi::new();
        assert!(a.to_string().is_empty());
        a = a.fg((255, 255, 255));
        assert_eq!(a.to_string(), "\u{1b}[38;2;255;255;255m");
        a = a.clear_fg();
        assert!(a.to_string().is_empty());
        a = a.bg((255, 255, 255));
        assert_eq!(a.to_string(), "\u{1b}[48;2;255;255;255m");
        a = a.clear_bg();
        assert!(a.to_string().is_empty());
        a = a.bold();
        assert_eq!(a.to_string(), "\u{1b}[1m");
        a = a.bold();
        assert!(a.to_string().is_empty());
    }

    #[test]
    fn derives() {
        assert!(Ansi::new() != Ansi::from_fg((255, 255, 255)));
        assert!(Ansi::new() != Ansi::from_bg((255, 255, 255)));
        assert!(Ansi::new() == Ansi::from_fg((255, 255, 255)).clear());
        assert_ne!(
            Ansi::from_fg((255, 255, 255)),
            Ansi::from_bg((255, 255, 255))
        );
    }

    #[test]
    fn paint_text() {
        let ansi = Ansi::from_fg((255, 255, 255)).bold().underline();
        let painted = ansi.paint_text("Hello world!");
        assert_eq!(painted, "\u{1b}[1;4;38;2;255;255;255mHello world!\u{1b}[0m");

        let ansi = Ansi::new();
        let painted = ansi.paint_text("Hello world!");
        assert_eq!(painted, "Hello world!");
    }
}
