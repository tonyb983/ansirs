// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ansirs::{style_text, Ansi, Color};

fn main() {
    for code in u8::MIN..=u8::MAX {
        let color = Color::ansi_256_to_color(code);
        println!(
            "Code {} = {}",
            code,
            style_text(color, Ansi::from_fg(color))
        )
    }
}
