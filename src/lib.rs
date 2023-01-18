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
