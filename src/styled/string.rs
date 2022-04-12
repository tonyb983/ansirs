// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Remove me when implementation is written!
#![allow(unused)]

use crate::Ansi;

pub struct PrettyString(/* todo!() */);

impl PrettyString {
    /// Create a new [`PrettyString`] from the given text and style.
    pub fn new(text: &str, format: Ansi) -> Self {
        todo!()
    }

    /// Get the raw (unstyled) text contained in this [`PrettyString`].
    pub fn raw(&self) -> &str {
        todo!()
    }

    /// Get the current style of this [`PrettyString`].
    pub fn style(&self) -> &Ansi {
        todo!()
    }

    /// Get the formatted text contained in this [`PrettyString`].
    pub fn value(&self) -> &str {
        todo!()
    }

    /// Update the style of this [`PrettyString`] using the given function `F`.
    ///
    /// `F` will receive the current style and should return the new or modified style.
    pub fn modify_style<F: FnOnce(Option<Ansi>) -> Option<Ansi>>(&mut self, f: F) {
        todo!()
    }
}

impl std::fmt::Display for PrettyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<PrettyString> for String {
    fn from(pretty: PrettyString) -> Self {
        todo!()
    }
}
