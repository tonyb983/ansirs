// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Error type used when parsing a color.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorParseError {
    /// Bad characters were found in the color string.
    BadChars,
    /// The color string had too few or too many characters.
    WrongLength,
    /// The color string segment could not be parsed into a valid decimal number.
    ParseIntError(std::num::ParseIntError),
    /// Other errors (with message).
    Unknown(String),
}

impl std::fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorParseError::BadChars => write!(f, "Bad characters found in color string"),
            ColorParseError::WrongLength => {
                write!(f, "Color string had too few or too many characters")
            }
            ColorParseError::ParseIntError(inner) => {
                write!(f, "Could not parse color string into a number: {}", inner)
            }
            ColorParseError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for ColorParseError {}
