// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::Ansi;

pub struct PrettyString(/* todo!() */);

impl PrettyString {
    pub fn new(text: &str, format: Ansi) -> Self {
        todo!()
    }

    pub fn raw(&self) -> String {
        todo!()
    }
}

impl std::fmt::Display for PrettyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<PrettyString> for String {
    fn from(pretty: PrettyString) -> Self {
        todo!()
    }
}
