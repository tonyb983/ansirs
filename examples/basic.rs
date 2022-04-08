// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ansirs::{style_text, Ansi, Colors};

enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn label(&self) -> &'static str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

struct Logger {
    info_style: Ansi,
    warn_style: Ansi,
    error_style: Ansi,
}

impl Logger {
    fn new() -> Self {
        Self {
            info_style: Ansi::from_fg(Colors::LawnGreen),
            warn_style: Ansi::from_fg(Colors::Yellow).italic(),
            error_style: Ansi::from_fg(Colors::Red).bold(),
        }
    }

    fn log(&self, level: LogLevel, text: impl std::fmt::Display) {
        let ansi = match level {
            LogLevel::Info => &self.info_style,
            LogLevel::Warn => &self.warn_style,
            LogLevel::Error => &self.error_style,
        };

        println!(
            "{}",
            style_text(format!("[{}]: {}", level.label(), text), *ansi)
        );
    }

    fn info(&self, text: impl std::fmt::Display) {
        self.log(LogLevel::Info, text)
    }

    fn warn(&self, text: impl std::fmt::Display) {
        self.log(LogLevel::Warn, text)
    }

    fn error(&self, text: impl std::fmt::Display) {
        self.log(LogLevel::Error, text)
    }
}

fn main() {
    let logger = Logger::new();
    logger.info("Here some info message");
    logger.warn("Here is a warning");
    logger.error("Uh oh, an error has occurred");
}
