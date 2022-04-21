// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Remove me when implementation is written!
#![allow(unused)]

use crate::Ansi;

pub struct PrettyString<'a> {
    ansi: Ansi,
    original: &'a str,
}

impl<'a> PrettyString<'a> {
    /// Create a new [`PrettyString`] from the given text and style.
    pub fn new(text: &'a str, format: Ansi) -> Self {
        PrettyString {
            ansi: format,
            original: text,
        }
    }

    /// Get the raw (unstyled) text contained in this [`PrettyString`].
    pub fn raw(&self) -> &str {
        self.original
    }

    /// Get the current style of this [`PrettyString`].
    pub fn style(&self) -> Option<Ansi> {
       Some(self.ansi)
    }

    /// Get the formatted text contained in this [`PrettyString`].
    pub fn value(&self) -> String {
        self.ansi.paint_text(self.original)
    }

    /// Update the style of this [`PrettyString`] using the given function `F`.
    ///
    /// `F` will receive the current style and should return the new or modified style.
    pub fn modify_style<F: FnOnce(Option<Ansi>) -> Option<Ansi>>(&mut self, f: F) {
        
        if let Some(a) = f(self.style()) {
            self.ansi = a;
        }
    }
}

impl std::fmt::Display for PrettyString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<PrettyString<'_>> for String {
    fn from(pretty: PrettyString) -> Self {
        pretty.to_string()
    }
}
