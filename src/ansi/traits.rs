// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Ansi, Color, Colors};

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

impl IntoAnsi for &Ansi {
    fn into_ansi(self) -> Ansi {
        *self
    }
}

impl From<Color> for Ansi {
    fn from(c: Color) -> Self {
        c.into_ansi()
    }
}

impl From<&Color> for Ansi {
    fn from(c: &Color) -> Self {
        c.into_ansi()
    }
}

impl From<Colors> for Ansi {
    fn from(c: Colors) -> Self {
        c.into_ansi()
    }
}

impl From<&Colors> for Ansi {
    fn from(c: &Colors) -> Self {
        c.into_ansi()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ansi_from_color() {
        let ansi: Ansi = Color::from_rgb(250, 250, 250).into();
        assert_eq!(ansi, Ansi::from_fg((250, 250, 250)));
    }

    #[test]
    fn ansi_from_ref_color() {
        let ansi: Ansi = Color::from_rgb(250, 250, 250).into();
        let c = Color::from_rgb(250, 250, 250);
        assert_eq!(ansi, Ansi::from(&c));
    }

    #[test]
    fn ansi_from_colors() {
        let colors = Colors::Maroon;
        let ansi: Ansi = Color::from_rgb(128, 0, 0).into();
        assert_eq!(ansi, Ansi::from(colors));
    }

    #[test]
    fn ansi_from_ref_colors() {
        let colors = Colors::Maroon;
        let ansi: Ansi = Color::from_rgb(128, 0, 0).into();
        assert_eq!(ansi, Ansi::from(&colors));
    }

    #[test]
    fn ansi_ref_intoansi() {
        let ansi: Ansi = Ansi::from_fg((100, 250, 100));
        let ref_ansi: &Ansi = &ansi;
        assert_eq!(ref_ansi.into_ansi(), ansi);
    }
}
