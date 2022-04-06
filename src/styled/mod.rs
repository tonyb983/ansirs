// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Ansi, IntoAnsi};

/// Styles the given [`Display`](std::fmt::Display) using the style described by `style`.
/// `S` can be either an [`Ansi`](Ansi) or a closure that returns an [`Ansi`](Ansi). This might
/// require bringing the [`IntoAnsi`](IntoAnsi) trait into scope.
pub fn style_text<S: IntoAnsi>(text: impl std::fmt::Display, style: S) -> String {
    let actual = format!("{}", text);

    if actual.is_empty() {
        actual
    } else {
        let ansi = style.into_ansi();
        if ansi.is_default() {
            actual
        } else {
            format!("{}{}{}", ansi, text, Ansi::reset())
        }
    }
}

pub fn styled_print<S: IntoAnsi>(text: impl std::fmt::Display, style: S) {
    print!("{}", style_text(text, style));
}

pub fn styled_println<S: IntoAnsi>(text: impl std::fmt::Display, style: S) {
    println!("{}", style_text(text, style));
}

pub trait Styled {
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
