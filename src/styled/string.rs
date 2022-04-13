// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Remove me when implementation is written!
#![allow(unused)]

use crate::Ansi;

pub struct PrettyString {
    text: String,
    format: Ansi,
    formatted_text: String,
}

impl PrettyString {
    /// Create a new [`PrettyString`] from the given text and style.
    pub fn new(text: &str, format: Ansi) -> Self {
        PrettyString {
            text: text.to_string(),
            format,
            formatted_text: format.paint_text(text),
        }
    }

    /// Get the raw (unstyled) text contained in this [`PrettyString`].
    pub fn raw(&self) -> &str {
        &self.text
    }

    /// Get the current style of this [`PrettyString`].
    pub fn style(&self) -> &Ansi {
        &self.format
    }

    /// Get the formatted text contained in this [`PrettyString`].
    pub fn value(&self) -> &str {
        &self.formatted_text
    }

    /// Update the style of this [`PrettyString`] using the given function `F`.
    ///
    /// `F` will receive the current style and should return the new or modified style.
    pub fn modify_style<F: FnOnce(Option<Ansi>) -> Option<Ansi>>(&mut self, f: F) {
        if let Some(f) = f(Some(self.format)) {
            self.format = f;
        }
    }
}

impl std::fmt::Display for PrettyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl From<PrettyString> for String {
    fn from(pretty: PrettyString) -> Self {
        pretty.to_string()
    }
}
