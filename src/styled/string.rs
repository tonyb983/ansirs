// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Ansi, IntoAnsi};

/// A string and some styling.
///
/// Some stats:
/// - Sizeof usize or & = 8
/// - Sizeof Ansi = 9
/// - Sizeof String = 24
/// - Sizeof (String, usize) = 32
/// - Sizeof Stage 1 = 40
/// - Sizeof Stage 2 = 40
///
/// First stage was: ([`String`], [`Ansi`])
/// Sizeof = 40 bytes
/// Adjusted this so that if style is empty or default, we can save space by not storing it.
///
/// Second stage was ([`String`], [`Option<Ansi>`])
/// Sizeof = 40 bytes
/// This is taking up the same amount of space as Stage 1
///
/// Potential third stage is (String, usize)
/// Sizeof = 32 bytes
/// This would represent the text (including the formatting) and the length of the original text.
/// We can always retrieve the original text using the stored length, and the known offset of the reset text (Ansi::SUFFIX).
/// This would require an Ansi function that can parse a string of ansi codes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrettyString(String, Option<Ansi>);

impl PrettyString {
    pub fn plain(s: impl Into<String>) -> Self {
        Self(s.into(), None)
    }

    pub fn new(text: impl AsRef<str>, format: impl IntoAnsi) -> Self {
        let text = text.as_ref().to_string();
        let style = format.into_ansi();
        Self(
            text,
            if style.is_default() {
                None
            } else {
                Some(style)
            },
        )
    }

    pub fn raw(&self) -> &str {
        self.0.as_str()
    }

    pub fn style(&self) -> Option<&Ansi> {
        self.1.as_ref()
    }

    pub fn modify_style<F: FnMut(Option<&Ansi>) -> Option<Ansi>>(&mut self, mut f: F) {
        self.1 = f(self.1.as_ref());
    }

    pub fn value(&self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for PrettyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            Some(ref style) => write!(f, "{}", style.paint_text(self.0.as_str())),
            None => write!(f, "{}", self.0),
        }
    }
}

impl From<PrettyString> for String {
    fn from(pretty: PrettyString) -> Self {
        pretty.0
    }
}

impl From<&PrettyString> for String {
    fn from(pretty: &PrettyString) -> Self {
        pretty.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactPrettyString(String, usize);

impl CompactPrettyString {
    pub fn new(text: impl std::fmt::Display, format: impl IntoAnsi) -> Self {
        let text = text.to_string();
        let style = format.into_ansi();
        let len = text.len();
        let styled = style.paint_text(text.as_str());
        Self(styled, len)
    }

    pub fn raw(&self) -> &str {
        let total_size = self.0.len();
        if self.0.is_empty() || total_size == self.1 {
            return self.0.as_str();
        }

        let total_size = self.0.len();
        let reset_size = Ansi::reset().len();
        let string_end = total_size - reset_size;
        let string_start = total_size - reset_size - self.1;
        &self.0[string_start..string_end]
    }

    pub fn style(&self) -> Option<Ansi> {
        Ansi::parse_ansi_text(self.0.as_str())
    }

    pub fn modify_style<F: FnOnce(Option<Ansi>) -> Option<Ansi>>(&mut self, f: F) {
        let current = self.style();
        let style = f(current);
        let text = self.raw();
        assert_eq!(text.len(), self.1);
        self.0 = match style {
            Some(style) => style.paint_text(text),
            None => text.to_string(),
        };
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
    }
}

impl From<CompactPrettyString> for String {
    fn from(pretty: CompactPrettyString) -> Self {
        pretty.raw().to_string()
    }
}

impl From<&CompactPrettyString> for String {
    fn from(pretty: &CompactPrettyString) -> Self {
        pretty.raw().to_string()
    }
}

impl std::fmt::Display for CompactPrettyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<String> for CompactPrettyString {
    fn eq(&self, other: &String) -> bool {
        self.raw() == other.as_str()
    }
}
impl PartialEq<&String> for CompactPrettyString {
    fn eq(&self, other: &&String) -> bool {
        self.raw() == other.as_str()
    }
}
impl PartialEq<String> for &CompactPrettyString {
    fn eq(&self, other: &String) -> bool {
        self.raw() == other.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn basic_usage() {
        let style = Ansi::new().fg((25, 100, 250)).bold().underline();
        let text = "Hello, World!";
        let pretty = PrettyString::new(text, style);
        assert_eq!(pretty.raw(), text);
        assert_eq!(pretty.style(), Some(&style));
        assert_eq!(
            pretty.to_string(),
            format!("{}{}{}", style, text, Ansi::reset())
        );
    }

    #[test]
    fn plain() {
        assert_eq!(
            PrettyString::plain("Hello, World!").to_string(),
            "Hello, World!"
        );
        let style = Ansi::new().fg((25, 100, 250)).bold().underline();
        let text = "Hello, World!";
        let mut pretty = PrettyString::new(text, style);
        assert_ne!(pretty.to_string(), text);
        pretty.modify_style(|_| None);
        assert_eq!(pretty.to_string(), text);
        assert_eq!(PrettyString::plain(text), pretty);
        assert_eq!(PrettyString::new(text, Ansi::default()), pretty);
        assert_eq!(
            PrettyString::new(text, Ansi::default()),
            PrettyString::plain(text)
        );
    }

    #[test]
    fn modify_style() {
        let style1 = Ansi::new().fg((25, 100, 250)).bold().underline();
        let style2 = Ansi::new().bg((25, 100, 250)).italic().strike();
        let text = "Hello, World!";
        let mut pretty = PrettyString::new(text, style1);
        assert_eq!(pretty.raw(), text);
        assert_eq!(pretty.style(), Some(&style1));
        assert_eq!(
            pretty.to_string(),
            format!("{}{}{}", style1, text, Ansi::reset())
        );

        pretty.modify_style(|_| Some(style2));
        assert_eq!(pretty.raw(), text);
        assert_eq!(pretty.style(), Some(&style2));
        assert_eq!(
            pretty.to_string(),
            format!("{}{}{}", style2, text, Ansi::reset())
        );
    }

    #[test]
    fn conversion() {
        use std::borrow::Borrow;
        let style = Ansi::new().fg((25, 250, 75)).blink();
        let pretty = PrettyString::new("Hello", style);
        let string: String = pretty.borrow().into();
        assert_eq!(string, "Hello");
    }
}
