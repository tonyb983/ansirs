// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ansirs::{style_text, Ansi, Colors};

fn main() {
    let first = Colors::AliceBlue;
    for color in first {
        println!("{}", style_text(color.name(), Ansi::from_fg(color)))
    }
}
