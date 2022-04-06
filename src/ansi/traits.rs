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

impl From<Color> for Ansi {
    fn from(c: Color) -> Self {
        c.into_ansi()
    }
}

impl From<Colors> for Ansi {
    fn from(c: Colors) -> Self {
        c.into_ansi()
    }
}
