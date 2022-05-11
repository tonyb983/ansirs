// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// `compact` Module
///
/// This module contains the second implementation of the styled string type. It stores the formatted text
/// as an allocated [`String`], and the length of the **original** text. This means that [`CompactPrettyString::value`]
/// is cached and does not need to be calculated on each call, but it also means that [`CompactPrettyString::raw`] and
/// [`CompactPrettyString::style`] need to be calculated on each call.
pub mod compact;
/// `pretty` Module
///
/// This module contains the first implementation of the styled string type. It stores the original text
/// (as an allocated [`String`]) and the [`Ansi`](crate::Ansi) style. It calculates the styled text each
/// time [`PrettyString::value`] is called.
pub mod pretty;
/// `refstr` Module
///
/// This contains the third implementation of the styled string type. It stores a reference to the original
/// text (as a [`&str`]) and the [`Ansi`](crate::Ansi) style. It calculates the styled text each time
/// [`PrettyStr::value`] is called.
pub mod refstr;

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
        println!(
            "   Size of PrettyStr: {}",
            std::mem::size_of::<PrettyStr<'_>>()
        );
        println!("Size of &str: {}", std::mem::size_of::<&str>());
        println!("Size of Ansi : {}", std::mem::size_of::<Ansi>());
        println!("Size of Ansi?: {}", std::mem::size_of::<Option<Ansi>>());
    }

    #[test]
    #[ignore]
    fn performance() {
        const ANSI_STR: &str = "\u{1b}[3;4;9;38;2;200;100;200;48;2;255;255;255m";
        let text = "Hello there world, glad to meet ya!";
        let style = Ansi::from_fg((50, 150, 250)).bold().underline().italic();

        println!("Comparing ctors...");
        compare_ctor(text, &style);

        println!("Comparing value()...");
        compare_value(text, &style);

        println!("Comparing raw()...");
        compare_raw(text, &style);

        println!("Comparing style()...");
        compare_style(text, &style);
    }

    fn compare_ctor(text: &str, style: &Ansi) {
        use std::time::Instant;
        const ITERS: usize = 1_000_000;
        const ITERS_U32: u32 = 1_000_000;

        let start = Instant::now();
        for _ in 0..ITERS {
            let _pretty = PrettyString::new(text, style);
        }
        let pstring_end = start.elapsed();
        let pstring_ave = pstring_end / ITERS_U32;

        let start = Instant::now();
        for _ in 0..ITERS {
            let _pretty = CompactPrettyString::new(text, style);
        }
        let cps_end = start.elapsed();
        let cps_ave = cps_end / ITERS_U32;

        let start = Instant::now();
        for _ in 0..ITERS {
            let _pretty = PrettyStr::new(text, style);
        }
        let pstr_end = start.elapsed();
        let pstr_ave = pstr_end / ITERS_U32;

        println!("For {} iterations:", ITERS);
        println!(
            "{:<15} {:>10?} ({:?} average)",
            "P-String", pstring_end, pstring_ave
        );
        println!(
            "{:<15} {:>10?} ({:?} average)",
            "CompactP-String", cps_end, cps_ave
        );
        println!("{:<15} {:>10?} ({:?} average)", "P-Str", pstr_end, pstr_ave);
    }

    fn compare_value(text: &str, style: &Ansi) {
        use std::time::Instant;
        const ITERS: usize = 1_000_000;
        const ITERS_U32: u32 = 1_000_000;

        let ps = PrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _value = ps.value();
        }
        let pstring_end = start.elapsed();
        let pstring_ave = pstring_end / ITERS_U32;

        let cps = CompactPrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _value = cps.value();
        }
        let cps_end = start.elapsed();
        let cps_ave = cps_end / ITERS_U32;

        let pstr = PrettyStr::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _value = pstr.value();
        }
        let pstr_end = start.elapsed();
        let pstr_ave = pstr_end / ITERS_U32;

        println!("For {} iterations:", ITERS);
        println!(
            "{:<15} {:>10?} ({:?} average)",
            "P-String", pstring_end, pstring_ave
        );
        println!(
            "{:<15} {:>10?} ({:?} average)",
            "CompactP-String", cps_end, cps_ave
        );
        println!("{:<15} {:>10?} ({:?} average)", "P-Str", pstr_end, pstr_ave);
    }

    fn compare_raw(text: &str, style: &Ansi) {
        use std::time::Instant;
        const ITERS: usize = 1_000_000;
        const ITERS_U32: u32 = 1_000_000;

        let ps = PrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _raw = ps.raw();
        }
        let pstring_end = start.elapsed();
        let pstring_ave = pstring_end / ITERS_U32;

        let cps = CompactPrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _raw = cps.raw();
        }
        let cps_end = start.elapsed();
        let cps_ave = cps_end / ITERS_U32;

        let pstr = PrettyStr::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _raw = pstr.raw();
        }
        let pstr_end = start.elapsed();
        let pstr_ave = pstr_end / ITERS_U32;

        println!("For {} iterations:", ITERS);
        println!(
            "{:<15} {:>10?} ({:?} average)",
            "P-String", pstring_end, pstring_ave
        );
        println!(
            "{:<15} {:>10?} ({:?} average)",
            "CompactP-String", cps_end, cps_ave
        );
        println!("{:<15} {:>10?} ({:?} average)", "P-Str", pstr_end, pstr_ave);
    }

    fn compare_style(text: &str, style: &Ansi) {
        use std::time::Instant;
        const ITERS: usize = 1_000_000;
        const ITERS_U32: u32 = 1_000_000;

        let ps = PrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _style = ps.style();
        }
        let pstring_end = start.elapsed();
        let pstring_ave = pstring_end / ITERS_U32;

        let cps = CompactPrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _style = cps.style();
        }
        let cps_end = start.elapsed();
        let cps_ave = cps_end / ITERS_U32;

        let pstr = PrettyStr::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _style = pstr.style();
        }
        let pstr_end = start.elapsed();
        let pstr_ave = pstr_end / ITERS_U32;

        println!("For {} iterations:", ITERS);
        println!(
            "{:<15} {:>10?} ({:?} average)",
            "P-String", pstring_end, pstring_ave
        );
        println!(
            "{:<15} {:>10?} ({:?} average)",
            "CompactP-String", cps_end, cps_ave
        );
        println!("{:<15} {:>10?} ({:?} average)", "P-Str", pstr_end, pstr_ave);
    }
}
