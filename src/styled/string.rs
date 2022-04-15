// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Remove me when implementation is written!
#![allow(unused)]

use crate::Ansi;

const SUFFIX_LENGTH:usize = 7;

pub struct PrettyString {
    formatted_text: String,
    length_original: usize,
}

impl PrettyString {
    /// Create a new [`PrettyString`] from the given text and style.
    pub fn new(text: &str, format: Ansi) -> Self {
        PrettyString {
            formatted_text: format.paint_text(text),
            length_original: text.len(),
        }
    }

    /// Get the raw (unstyled) text contained in this [`PrettyString`].
    pub fn raw(&self) -> &str {
        let (_, o) = self.get_style_and_original();
        o
    }

    /// Get the current style of this [`PrettyString`].
    pub fn style(&self) -> Option<Ansi> {
        Ansi::parse_ansi_text(&self.formatted_text)
    }

    /// Get the formatted text contained in this [`PrettyString`].
    pub fn value(&self) -> &str {
        &self.formatted_text
    }

    /// Update the style of this [`PrettyString`] using the given function `F`.
    ///
    /// `F` will receive the current style and should return the new or modified style.
    pub fn modify_style<F: FnOnce(Option<Ansi>) -> Option<Ansi>>(&mut self, f: F) {
        if let Some(f) = f(self.style()) {
            let (original_t, _) = self.get_style_and_original();
            let p = PrettyString::new(original_t, f);
            self.formatted_text = p.formatted_text;
        }
    }

    fn get_style_and_original(&self) -> (&str, &str) {
        let formatting_length = self.formatted_text.len() - SUFFIX_LENGTH - self.length_original;
        let style = &self.formatted_text[0..formatting_length];
        let original = &self.formatted_text[formatting_length..self.formatted_text.len() - SUFFIX_LENGTH];
        (style, original)
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
