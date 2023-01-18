// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Ansi, Color, ColorParseError, Colors, IntoAnsi};

/// Trait used to facilitate converting various types to a color.
pub trait ToColor: std::fmt::Debug {
    /// Perform the conversion.
    fn to_color(&self) -> Color;
}

impl ToColor for Colors {
    fn to_color(&self) -> Color {
        self.rgb().into()
    }
}

impl ToColor for &Colors {
    fn to_color(&self) -> Color {
        (*self).rgb().into()
    }
}

impl ToColor for Color {
    fn to_color(&self) -> Color {
        *self
    }
}

impl ToColor for &Color {
    fn to_color(&self) -> Color {
        **self
    }
}

impl IntoAnsi for Colors {
    fn into_ansi(self) -> Ansi {
        Ansi::from_fg(self)
    }
}

impl IntoAnsi for &Colors {
    fn into_ansi(self) -> Ansi {
        Ansi::from_fg(*self)
    }
}

impl IntoAnsi for Color {
    fn into_ansi(self) -> Ansi {
        Ansi::from_fg(self)
    }
}

impl IntoAnsi for &Color {
    fn into_ansi(self) -> Ansi {
        Ansi::from_fg(*self)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Color::from_rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl ToColor for (u8, u8, u8) {
    fn to_color(&self) -> Color {
        Color::from_rgb(self.0, self.1, self.2)
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorParseError;

    /// Attempts to parse the given string as a hex string into a [`Color`].
    fn try_from(input: &str) -> Result<Self, ColorParseError> {
        Color::from_hex(input)
    }
}
