// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ansirs::{Ansi, Colors, Styled};

use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{
        format::{FmtSpan, Format, Pretty},
        SubscriberBuilder,
    },
    FmtSubscriber,
};

pub fn test_sub_builder() -> SubscriberBuilder<Pretty, Format<Pretty>> {
    FmtSubscriber::builder()
        .pretty()
        .with_span_events(FmtSpan::CLOSE)
        .with_max_level(LevelFilter::TRACE)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
    //.finish()
}

pub fn init_test_subscriber() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    test_sub_builder().try_init()
}

#[test]
// #[cfg_attr(feature = "trace", test)]
fn tracing_test() {
    init_test_subscriber().expect("unable to init test subscriber");

    let header = Ansi::from_fg(Colors::BlanchedAlmond)
        .bg(Colors::BlueViolet)
        .underline()
        .bold();
    let body = Ansi::from_fg(Colors::LightGreen);
    let italic = body.italic();
    let bold = body.bold();
    let underline = body.underline();
    let link = Ansi::from_fg(Colors::CornFlowerBlue).underline();

    tracing::info!("tracing_test starting");

    let header_text = "Here is some header text".style(header);

    let body_text = format!(
        r#"{}, including {}, {}, {} and even a {}!"#,
        "Here is some content".style(body),
        "some italic text".style(italic),
        "some bold text".style(bold),
        "some underlined text".style(underline),
        "a link".style(link)
    );

    let text = format!("{header_text}\n\n{body_text}");

    println!("{text}");

    tracing::info!("tracing_test starting");
}
