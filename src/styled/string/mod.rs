// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod compact;
mod pretty;
mod refstr;

pub use compact::CompactPrettyString;
pub use pretty::PrettyString;
pub use refstr::PrettyStr;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ansi;
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

    #[test]
    fn sizeof() {
        println!(
            "Size of PrettyString: {}",
            std::mem::size_of::<PrettyString>()
        );
        println!(
            "   Size of CompactPS: {}",
            std::mem::size_of::<CompactPrettyString>()
        );
        println!("   Size of PrettyStr: {}", std::mem::size_of::<PrettyStr>());
        println!("Size of &str: {}", std::mem::size_of::<&str>());
        println!("Size of Ansi : {}", std::mem::size_of::<Ansi>());
        println!("Size of Ansi?: {}", std::mem::size_of::<Option<Ansi>>());
    }
}
