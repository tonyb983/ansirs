// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Ansi, ColorParseError};

/// Wrapper struct around a (u8, u8, u8) tuple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Color(u8, u8, u8);

impl Color {
    /// Create a new color from the given RGB values.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    /// Attempt to create a new color from the given hexadecimal string.
    pub fn from_hex<S: AsRef<str>>(input: S) -> Result<Self, ColorParseError> {
        fn convert(input: &str) -> Result<u8, ColorParseError> {
            u8::from_str_radix(input, 16).map_err(ColorParseError::ParseIntError)
        }

        let mut string = input.as_ref();

        if string.starts_with('#') {
            string = &string[1..];
        }

        if string.len() != 3 && string.len() != 6 {
            return Err(ColorParseError::WrongLength);
        }

        if !string.is_ascii() {
            return Err(ColorParseError::BadChars);
        }

        let is_double = string.len() == 6;

        let mut chars = string.chars();

        let mut rgb = [0u8, 0u8, 0u8];
        for idx in &mut rgb {
            *idx = if is_double {
                let f = chars.next().ok_or_else(|| {
                    ColorParseError::Unknown("Unexpected end of string!".to_string())
                })?;
                let s = chars.next().ok_or_else(|| {
                    ColorParseError::Unknown("Unexpected end of string!".to_string())
                })?;

                convert(&format!("{}{}", f, s))?
            } else {
                let c = chars.next().ok_or_else(|| {
                    ColorParseError::Unknown("Unexpected end of string!".to_string())
                })?;

                convert(&format!("{}{}", c, c))?
            };
        }

        Ok(Self(rgb[0], rgb[1], rgb[2]))
    }

    /// Create a hex string from this color.
    pub fn as_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    /// Get the RGB tuple of this color.
    pub const fn rgb(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    /// Get the **Red** value of this color.
    pub const fn r(&self) -> u8 {
        self.0
    }

    /// Get the **Green** value of this color.
    pub const fn g(&self) -> u8 {
        self.1
    }

    /// Get the **Blue** value of this color.
    pub const fn b(&self) -> u8 {
        self.2
    }

    pub fn into_ansi(self) -> Ansi {
        Ansi::from_fg(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn hex() {
        let color = Color::from_rgb(25, 100, 250);
        assert_eq!(color.as_hex(), "#1964FA");
    }

    #[test]
    fn components() {
        let color = Color::from_rgb(25, 100, 250);
        assert_eq!(color.r(), 25);
        assert_eq!(color.g(), 100);
        assert_eq!(color.b(), 250);
    }
}
