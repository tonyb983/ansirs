//! # Ansirs
//!
//! Simple library for working with ANSI escape codes to add pretty colors to your shitty console text.
//!
//! Copyright (c) 2022 Tony Barbitta
//!
//! This Source Code Form is subject to the terms of the Mozilla Public
//! License, v. 2.0. If a copy of the MPL was not distributed with this
//! file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(dead_code)]

mod ansi;
mod color;
mod styled;

pub mod iter {
    pub use crate::color::iter::*;
}

pub use ansi::*;
pub use color::*;
pub use styled::*;
