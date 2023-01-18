// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Ansi, IntoAnsi};

/// `string` Module
///
/// This module contains some experiments with storing a formatted string, i.e. a
/// wrapped [`String`] that also contains an [`Ansi`](crate::Ansi) style.
#[cfg(feature = "strings")]
pub mod string;

/// Styles the given [`Display`](std::fmt::Display) using the style described by `style`.
/// `S` can be either an [`Ansi`](Ansi) or a closure that returns an [`Ansi`](Ansi). This might
/// require bringing the [`IntoAnsi`](IntoAnsi) trait into scope.
#[cfg_attr(feature = "trace", tracing::instrument(skip(text, style), fields(text = %text, style_ansi)))]
pub fn style_text<S: IntoAnsi>(text: impl std::fmt::Display, style: S) -> String {
    let actual = format!("{text}");

    if actual.is_empty() {
        actual
    } else {
        let ansi: Ansi = style.into_ansi();
        #[cfg(feature = "trace")]
        {
            let style = format!("{ansi:?}");
            tracing::Span::current().record("style_ansi", style.as_str());
        }
        if ansi.is_default() {
            actual
        } else {
            format!("{}{}{}", ansi, text, Ansi::reset())
        }
    }
}

/// Shortcut to call `print!` with the output of `style_text`.
pub fn styled_print<S: IntoAnsi>(text: impl std::fmt::Display, style: S) {
    print!("{}", style_text(text, style));
}

/// Shortcut to call `println!` with the output of `style_text`.
#[cfg_attr(feature = "trace", tracing::instrument(skip(text, style), fields(text = %text, styled)))]
pub fn styled_println<S: IntoAnsi>(text: impl std::fmt::Display, style: S) {
    let styled = style_text(text, style);
    #[cfg(feature = "trace")]
    {
        tracing::Span::current().record("styled", styled.as_str());
    }
    println!("{styled}");
}

/// Trait used to add a `style` "extension method" to any type that implements [`Display`](std::fmt::Display)
/// as a convenience to call `style_text`.
pub trait Styled {
    /// Style this value using the given `style`.
    fn style(&self, style: impl IntoAnsi) -> String;
}

impl<T> Styled for T
where
    T: std::fmt::Display,
{
    fn style(&self, style: impl IntoAnsi) -> String {
        style_text(self.to_string(), style)
    }
}

/// Trait that is currently only used to try and normalize the interface between my variations
/// of styled strings.
pub trait StyledString {
    /// Get the "raw" (aka unstyled / original) text.
    #[must_use]
    fn raw(&self) -> &str;

    /// Get the [`Ansi`] styling applied to this text.
    #[must_use]
    fn style(&self) -> Option<&Ansi>;

    /// Modify the styling applied to this text using the given closure.
    fn modify_style<F: FnMut(Option<&Ansi>) -> Option<Ansi>>(&mut self, f: F);

    /// Get the formatted value of this [`StyledString`].
    #[must_use]
    fn value(&self) -> String;

    /// Gets the length of the ***original text***, i.e. the VISIBLE length.
    #[must_use]
    fn len(&self) -> usize;

    /// Checks if the original / **visible** text is empty
    #[must_use]
    fn is_empty(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const DISPLAY_PRE: &str = "\u{1b}[";
    const DISPLAY_SUF: &str = "m";

    fn empty_style_function() -> Ansi {
        Ansi::new()
    }

    #[test]
    fn storing_styles() {
        let style1 = Ansi::new().fg((100, 200, 100)).underline();
        let style2 = Ansi::new().bg((0, 0, 75)).italic().strike();

        assert_eq!(
            style1.to_string(),
            format!("{DISPLAY_PRE}4;38;2;100;200;100{DISPLAY_SUF}")
        );
        assert_eq!(
            style2.to_string(),
            format!("{DISPLAY_PRE}3;9;48;2;0;0;75{DISPLAY_SUF}")
        );
        assert_eq!(
            style1.to_string(),
            format!("{DISPLAY_PRE}4;38;2;100;200;100{DISPLAY_SUF}")
        );
        assert_eq!(
            style2.to_string(),
            format!("{DISPLAY_PRE}3;9;48;2;0;0;75{DISPLAY_SUF}")
        );
    }

    #[test]
    fn style_text_basic() {
        let first = "first".to_string();
        let unstyled_val = style_text(&first, Ansi::new());
        assert_eq!(unstyled_val, first);
        let unstyled_fn = style_text(&first, empty_style_function);
        assert_eq!(unstyled_fn, first);

        let manual_prefix = format!("{}{}{}", DISPLAY_PRE, "4;38;2;255;0;0", DISPLAY_SUF);
        let manual_suffix = format!("{}{}{}", DISPLAY_PRE, "0", DISPLAY_SUF);
        let manual = format!("{manual_prefix}{first}{manual_suffix}");

        let styled_value = style_text(&first, Ansi::red().underline());

        assert_eq!(styled_value, manual);
    }

    #[test]
    fn style_text_inputs() {
        let first = "first".to_string();

        let st = style_text(&first, Ansi::new());
        let sf = style_text(&first, empty_style_function);
        let sc = style_text(&first, || {
            let style = Ansi::new()
                .underline()
                .italic()
                .fg((200, 100, 200))
                .bg((255, 255, 255));

            style.strike()
        });
        // Why the fuck cant i get this to work in another project.
        let _styled_colors = style_text(&first, crate::Colors::Yellow.into_ansi());
        let _styled_colors = style_text(&first, crate::Colors::Yellow);
        let yellow = crate::Colors::Yellow.into_color();
        let _styled_color = style_text(&first, yellow.into_ansi());

        let manual_prefix = format!(
            "{}{}{}",
            DISPLAY_PRE, "3;4;9;38;2;200;100;200;48;2;255;255;255", DISPLAY_SUF
        );
        let manual_suffix = format!("{}{}{}", DISPLAY_PRE, "0", DISPLAY_SUF);
        let third = format!("{manual_prefix}{first}{manual_suffix}");

        assert_eq!(&st, &first);
        assert_eq!(&sf, &first);
        assert_eq!(&sc, &third);
    }
}
