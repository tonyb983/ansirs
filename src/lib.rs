// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # Ansirs
//!
//! Simple library for working with ANSI escape codes to add pretty colors to your shitty console text.

// Activate ALL THE WARNINGS. I want clippy to be as absolutely annoying as fucking possible.
#![warn(
    clippy::pedantic,
    clippy::all,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    rustdoc::all
)]
#![allow(dead_code, clippy::module_name_repetitions)]

mod ansi;
mod color;
mod styled;

/// Contains code for iterating over named colors.
pub mod iter {
    pub use crate::color::iter::*;
}

pub use ansi::*;
pub use color::*;
pub use styled::*;

pub(crate) mod macros {
    /// `flame_guard!` - creates a flamegraph scope guard using the `flame` crate.
    /// ### Input(s):
    /// ```ignore
    /// flame_guard!("feature_name", ["path", "segments", "for", "fn"]);
    /// flame_guard!(["path", "segments", "for", "fn"]);
    /// flame_guard!("path", "segments", "for", "fn");
    /// ```
    /// (The last two are equivalent, and use the default value for `feature_name` aka `flame_on`.)
    ///
    /// ### Output:
    /// - `flame_guard!("feature_name", ["path1", "path2", ...]);`:
    /// ```ignore
    /// #[cfg(feature = "feature_name")]
    /// const FG_PATH: &[&str] = &["path1", "path2", ...];
    /// #[cfg(feature = "feature_name")]
    /// let _fg = ::flame::start_guard(FG_PATH.join("::"));
    /// ```
    /// - `flame_guard!(["path1", "path2", ...]);` or `flame_guard!("path1", "path2", ...);`:
    /// ```ignore
    /// #[cfg(feature = "flame_on")]
    /// const FG_PATH: &[&str] = &["path1", "path2", ...];
    /// #[cfg(feature = "flame_on")]
    /// let _fg = ::flame::start_guard(FG_PATH.join("::"));
    /// ```
    #[macro_export]
    macro_rules! flame_guard {
        ($($path_seg:expr),+ $(,)?) => {
            #[cfg(feature = "flame_on")]
            const FG_PATH: &[&str] = &[$($path_seg,)*];
            #[cfg(feature = "flame_on")]
            let _fg = ::flame::start_guard(FG_PATH.join("::"));
        };
    }

    /// `flame_dump!` - creates a call to dump the current flamegraph data to the indicated output.
    /// ### Input(s):
    /// ```ignore
    /// flame_dump!();
    /// flame_dump!(html);
    /// flame_dump!(html, "filename");
    /// flame_dump!(json);
    /// flame_dump!(json, "filename");
    /// flame_dump!(stdout);
    /// ```
    /// `html`, `json`, and `stdout` are literals. The first two statements are equivalent (aka the default is `html`).
    ///
    /// ### Output:
    /// - `flame_dump!();` or `flame_dump!(html);` or `flame_dump!(html, "filename");`:
    /// ```ignore
    /// #[cfg(feature = "flame_on")]
    /// {
    ///     let secs = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    ///     ::flame::dump_html(
    ///         ::std::fs::File::create(
    ///             format!("flames/{}.{}.html", FG_PATH.join(".") /* or $filename */, secs)
    ///         ).unwrap()
    ///     ).unwrap();
    /// }
    /// ```
    /// - `flame_dump!(json);` or `flame_dump!(json, "filename");`:
    /// ```ignore
    /// #[cfg(feature = "flame_on")]
    /// {
    ///     let secs = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    ///     ::flame::dump_json(
    ///         &mut ::std::fs::File::create(
    ///             format!("flames/{}.{}.json", FG_PATH.join(".") /* or $filename */, secs)
    ///         ).unwrap()
    ///     ).unwrap();
    /// }
    /// ```
    /// - `flame_dump!(stdout);`:
    /// ```ignore
    /// #[cfg(feature = "flame_on")]
    /// {
    ///     ::flame::dump_stdout();
    /// }
    /// ```
    #[macro_export]
    macro_rules! flame_dump {
        () => {
            #[cfg(feature = "flame_on")]
            {
                ::flame::dump_html(
                    std::fs::File::create(format!(
                        "flames/{}.{}.html",
                        FG_PATH.join("."),
                        std::time::UNIX_EPOCH
                            .elapsed()
                            .expect("Unable to get time since epoch")
                            .as_secs()
                    ))
                    .expect("Unable to create new html file"),
                )
                .expect("Unable to dump flamegraph data to html");
            }
        };
        (html) => {
            #[cfg(feature = "flame_on")]
            {
                let secs = std::time::UNIX_EPOCH
                            .elapsed()
                            .expect("Unable to get time since epoch")
                            .as_secs();
                ::flame::dump_html(
                    std::fs::File::create(format!(
                        "flames/{}.{}.html",
                        FG_PATH.join("."),
                        secs
                    ))
                    .expect("Unable to create new html file"),
                )
                .expect("Unable to dump flamegraph data to html");
            }
        };
        (html, $filename:expr) => {
            #[cfg(feature = "flame_on")]
            {
                let secs = std::time::UNIX_EPOCH
                            .elapsed()
                            .expect("Unable to get time since epoch")
                            .as_secs();
                ::flame::dump_html(
                    std::fs::File::create(format!(
                        "flames/{}.{}.html",
                        $filename,
                        secs
                    ))
                    .expect("Unable to create new html file"),
                )
                .expect("Unable to dump flamegraph data to html");
            }
        };
        (json) => {
            #[cfg(feature = "flame_on")]
            {
                let secs = std::time::UNIX_EPOCH
                            .elapsed()
                            .expect("Unable to get time since epoch")
                            .as_secs();
                ::flame::dump_json(
                    &mut std::fs::File::create(format!(
                        "flames/{}.{}.json",
                        FG_PATH.join("."),
                        secs,
                    ))
                    .expect("Unable to create new json file"),
                )
                .expect("Unable to dump flamegraph data to json");
            }
        };
        (json, $filename:expr) => {
            #[cfg(feature = "flame_on")]
            {
                let secs = std::time::UNIX_EPOCH
                            .elapsed()
                            .expect("Unable to get time since epoch")
                            .as_secs();
                ::flame::dump_json(
                    &mut std::fs::File::create(format!(
                        "flames/{}.{}.json",
                        $filename,
                        secs,
                    ))
                    .expect("Unable to create new json file"),
                )
                .expect("Unable to dump flamegraph data to json");
            }
        };
        (stdout) => {
            #[cfg(feature = "flame_on")]
            {
                ::flame::dump_stdout();
            }
        };
    }

    /// `flame_all_tests!(["Path1", "Path2", .., "PathN"], Test1, Test2, .. TestN)`
    ///
    /// Creates a test function named `flame_all_tests` which will call the listed test functions within
    /// their own flamegraph scope, and dump the resulting html. Easier than adding a scope or attribute to each test function.
    /// **Does not support `should_panic` tests and test failures will abort the function call!**
    ///
    /// ### Input(s)
    /// - `Path1 ... PathN` - A bracketed list of strings that will make up the output filename and **main** flamegraph scope.
    /// - `Test1 ... TestN` - Any number of test functions that will be invoked with their own flamegraph scope within the main test scope.
    ///
    /// ### Example Usage
    /// ```ignore
    /// #[cfg(test)]
    /// mod tests {
    ///     #[test]
    ///     fn test_function_1() { assert!(true); }
    ///     fn test_function_2() { assert!(true); }
    ///     fn test_function_3() { assert!(true); }
    ///     
    ///     flame_all_tests!(["my_mod", "SubjectUnderTest"], test_function_1, test_function_2, ..., test_function_n);
    /// }
    /// ```
    ///
    /// ### Output
    /// ```ignore
    /// // For the example invocation
    /// flame_all_tests!(["example", "SomeStruct", "tests"], test_function_1, test_function_2, test_function_3);
    /// // Output would be:
    /// #[cfg(feature = "flame_on")]
    /// #[cfg_attr(feature = "flame_on", test)]
    /// fn flame_all_tests() {
    ///     flame_guard!("example", "SomeStruct", "tests");
    ///     {
    ///         flame_guard!("test", "test_function_1");
    ///         test_function_1();
    ///     }
    ///     {
    ///         flame_guard!("test", "test_function_2");
    ///         test_function_2();
    ///     }
    ///     {
    ///         flame_guard!("test", "test_function_3");
    ///         test_function_3();
    ///     }
    ///     flame_dump!(html, "example.SomeStruct");
    /// }
    /// ```
    #[macro_export]
    macro_rules! flame_all_tests {
    ([$($path_seg:expr),+ $(,)?], $($test_fns:ident),+ $(,)?) => {
        #[cfg(feature = "flame_on")]
        #[cfg_attr(feature = "flame_on", test)]
        fn flame_all_tests() {
            const FG_PATH: &[&str] = &[$($path_seg,)*];
            let _fg = ::flame::start_guard(FG_PATH.join("::"));
            $({
                let _this_test = stringify!($test_fns);
                let _fgt = ::flame::start_guard(format!("test::{}", _this_test));
                $test_fns();
            })*
            let secs = std::time::UNIX_EPOCH
                        .elapsed()
                        .expect("Unable to get time since epoch")
                        .as_secs();
            ::flame::dump_html(
                std::fs::File::create(format!(
                    "flames/{}.{}.html",
                    FG_PATH.join("."),
                    secs
                ))
                .expect("Unable to create new html file"),
            )
            .expect("Unable to dump flamegraph data to html");
        }
    };
}
}
