// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use once_cell::sync::OnceCell;

use crate::{Ansi, IntoAnsi};

/// Fourth iteration of a styled string type. This one uses `once_cell::Lazy` to calculate
/// the styled text only when needed. Unfortunately this does mean that the text and the
/// style would not able to be changed after initialization...or does it...?
pub struct LazyPrettyString(String, Ansi, OnceCell<String>);

impl LazyPrettyString {
    /// Creates a new [`LazyPrettyString`] with the given `text` and `style`.
    pub fn new(text: impl std::fmt::Display, style: impl IntoAnsi) -> Self {
        let text = text.to_string();
        let style = style.into_ansi();
        Self(text, style, OnceCell::new())
    }

    /// Get the "raw" (aka unstyled / original) text.
    #[must_use]
    pub fn raw(&self) -> &str {
        self.0.as_ref()
    }

    /// Update the text for this [`LazyPrettyString`] in place.
    pub fn modify_text(&mut self, f: impl FnOnce(&mut String)) {
        f(&mut self.0);
        self.2 = OnceCell::new();
    }

    /// Sets the text to `text`.
    pub fn set_text(&mut self, text: impl std::fmt::Display) {
        self.0 = text.to_string();
        self.2 = OnceCell::new();
    }

    /// Get the [`Ansi`] styling applied to this text.
    #[must_use]
    pub fn style(&self) -> &Ansi {
        &self.1
    }

    /// Modify the styling applied to this text using the given closure.
    pub fn modify_style<F: FnMut(&mut Ansi)>(&mut self, mut f: F) {
        f(&mut self.1);
        self.2 = OnceCell::new();
    }

    /// Sets the styling applied to this text to the given `style`.
    pub fn set_style(&mut self, style: impl IntoAnsi) {
        self.1 = style.into_ansi();
        self.2 = OnceCell::new();
    }

    /// Get the formatted value of this [`LazyPrettyString`].
    #[must_use]
    pub fn value(&self) -> &str {
        self.2
            .get_or_init(|| self.1.paint_text(self.raw()))
            .as_str()
    }

    /// Gets the length of the ***original text***, i.e. the VISIBLE length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks if the original / **visible** text is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Display for LazyPrettyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutability() {
        let mut lps = LazyPrettyString::new("Hello, Red!", Ansi::from_fg((255, 0, 0)));
        println!("{lps}");
        lps.modify_style(|ansi| *ansi = Ansi::from_fg((0, 255, 0)));
        lps.modify_text(|s| *s = "Hello, Green!".to_string());
        println!("{lps}");
        lps.set_style(Ansi::from_fg((0, 0, 255)));
        lps.set_text("Hello, Blue!");
        println!("{lps}");
    }

    #[test]
    fn sizeof() {
        println!(
            "Sizeof LazyPrettyString = {}",
            std::mem::size_of::<LazyPrettyString>()
        );
    }
}
