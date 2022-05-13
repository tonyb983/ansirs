// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// `pretty` Module
///
/// This module contains the first implementation of the styled string type. It stores the original text
/// (as an allocated [`String`]) and the [`Ansi`](crate::Ansi) style. It calculates the styled text each
/// time [`PrettyString::value`] is called.
pub mod pretty;

/// `compact` Module
///
/// This module contains the second implementation of the styled string type. It stores the formatted text
/// as an allocated [`String`], and the length of the **original** text. This means that [`CompactPrettyString::value`]
/// is cached and does not need to be calculated on each call, but it also means that [`CompactPrettyString::raw`] and
/// [`CompactPrettyString::style`] need to be calculated on each call.
pub mod compact;

/// `refstr` Module
///
/// This contains the third implementation of the styled string type. It stores a reference to the original
/// text (as a [`&str`]) and the [`Ansi`](crate::Ansi) style. It calculates the styled text each time
/// [`PrettyStr::value`] is called.
pub mod refstr;

/// `lazy` Module
///
/// This contains the fourth implementation of the styled string type. This one will use `once_cell::sync::Lazy`
/// to cache the styled text on first use. This will undoubtedly lead to a bigger struct size but will hopefully
/// be the best performance-wise.
pub mod lazy;

pub use compact::CompactPrettyString;
pub use lazy::LazyPrettyString;
pub use pretty::PrettyString;
pub use refstr::PrettyStr;

#[allow(clippy::similar_names)]
#[cfg(test)]
mod tests {
    use crate::Ansi;

    use super::*;

    #[test]
    #[ignore]
    fn compare_all() {
        sizeof();
        println!();
        performance();
    }

    #[test]
    #[ignore]
    fn sizeof() {
        println!("Sizeof Comparison:");
        println!(
            "\t{:<20} ... {:>5}",
            "PrettyString",
            std::mem::size_of::<PrettyString>()
        );
        println!(
            "\t{:<20} ... {:>5}",
            "CompactPrettyString",
            std::mem::size_of::<CompactPrettyString>()
        );
        println!(
            "\t{:<20} ... {:>5}",
            "PrettyStr<'_>",
            std::mem::size_of::<PrettyStr<'_>>()
        );
        println!(
            "\t{:<20} ... {:>5}",
            "LazyPrettyString",
            std::mem::size_of::<LazyPrettyString>()
        );
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

        println!("Comparing value() (multiple calls)...");
        compare_value_multi(text, &style);

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

        let start = Instant::now();
        for _ in 0..ITERS {
            let _pretty = PrettyString::new(text, style);
        }
        let lps_end = start.elapsed();
        let lps_ave = lps_end / ITERS_U32;

        println!("For {} iterations:", ITERS);
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "P-String", pstring_end, pstring_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "Compact P-String", cps_end, cps_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "P-Str", pstr_end, pstr_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "Lazy P-String", lps_end, lps_ave
        );
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

        let lps = LazyPrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _value = lps.value();
        }
        let lps_end = start.elapsed();
        let lps_ave = lps_end / ITERS_U32;

        let pstr = PrettyStr::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _value = pstr.value();
        }
        let pstr_end = start.elapsed();
        let pstr_ave = pstr_end / ITERS_U32;

        println!("For {} iterations:", ITERS);
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "P-String", pstring_end, pstring_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "Compact P-String", cps_end, cps_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "P-Str", pstr_end, pstr_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "Lazy P-String", lps_end, lps_ave
        );
    }

    fn compare_value_multi(text: &str, style: &Ansi) {
        use std::time::Instant;
        const ITERS: usize = 1_000_000;
        const ITERS_U32: u32 = 1_000_000;

        let ps = PrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _value1 = ps.value();
            let _value2 = ps.value();
            let _value3 = ps.value();
        }
        let pstring_end = start.elapsed();
        let pstring_ave = pstring_end / ITERS_U32;

        let cps = CompactPrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _value1 = cps.value();
            let _value2 = cps.value();
            let _value3 = cps.value();
        }
        let cps_end = start.elapsed();
        let cps_ave = cps_end / ITERS_U32;

        let lps = LazyPrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _value1 = lps.value();
            let _value2 = lps.value();
            let _value3 = lps.value();
        }
        let lps_end = start.elapsed();
        let lps_ave = lps_end / ITERS_U32;

        let pstr = PrettyStr::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _value1 = pstr.value();
            let _value2 = pstr.value();
            let _value3 = pstr.value();
        }
        let pstr_end = start.elapsed();
        let pstr_ave = pstr_end / ITERS_U32;

        println!("For {} iterations:", ITERS);
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "P-String", pstring_end, pstring_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "Compact P-String", cps_end, cps_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "P-Str", pstr_end, pstr_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "Lazy P-String", lps_end, lps_ave
        );
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

        let lps = LazyPrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _raw = lps.raw();
        }
        let lps_end = start.elapsed();
        let lps_ave = lps_end / ITERS_U32;

        println!("For {} iterations:", ITERS);
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "P-String", pstring_end, pstring_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "Compact P-String", cps_end, cps_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "P-Str", pstr_end, pstr_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "Lazy P-String", lps_end, lps_ave
        );
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

        let lps = LazyPrettyString::new(text, style);
        let start = Instant::now();
        for _ in 0..ITERS {
            let _style = lps.style();
        }
        let lps_end = start.elapsed();
        let lps_ave = lps_end / ITERS_U32;

        println!("For {} iterations:", ITERS);
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "P-String", pstring_end, pstring_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "Compact P-String", cps_end, cps_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "P-Str", pstr_end, pstr_ave
        );
        println!(
            "\t{:<20} {:>10?} ({:?} average)",
            "Lazy P-String", lps_end, lps_ave
        );
    }
}
