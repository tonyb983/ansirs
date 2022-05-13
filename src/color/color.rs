// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Ansi, ColorParseError};

/// Wrapper struct around a (u8, u8, u8) tuple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color(u8, u8, u8);

impl Color {
    /// Create a new color from the given RGB values.
    #[must_use]
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    /// Attempt to create a new color from the given hexadecimal string.
    ///
    /// ## Errors
    /// - `ColorParseError` if the given input string cannot be converted to a color.
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    #[cfg_attr(feature = "flame_on", allow(clippy::items_after_statements))]
    pub fn from_hex<S: AsRef<str>>(input: S) -> Result<Self, ColorParseError> {
        fn convert(input: &str) -> Result<u8, ColorParseError> {
            u8::from_str_radix(input, 16).map_err(ColorParseError::ParseIntError)
        }

        let mut string = input.as_ref();

        if string.starts_with('#') {
            string = &string[1..];
        }

        if string.len() != 3 && string.len() != 6 {
            return Err(ColorParseError::WrongLength);
        }

        if !string.is_ascii() {
            return Err(ColorParseError::BadChars);
        }

        let is_double = string.len() == 6;

        let mut chars = string.chars();

        let mut rgb = [0u8, 0u8, 0u8];
        for idx in &mut rgb {
            *idx = if is_double {
                let f = chars.next().ok_or_else(|| {
                    ColorParseError::Unknown("Unexpected end of string!".to_string())
                })?;
                let s = chars.next().ok_or_else(|| {
                    ColorParseError::Unknown("Unexpected end of string!".to_string())
                })?;

                convert(&format!("{}{}", f, s))?
            } else {
                let c = chars.next().ok_or_else(|| {
                    ColorParseError::Unknown("Unexpected end of string!".to_string())
                })?;

                convert(&format!("{}{}", c, c))?
            };
        }

        Ok(Self(rgb[0], rgb[1], rgb[2]))
    }

    /// Create a hex string from this color.
    #[must_use]
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    pub fn as_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }

    /// Create a hex string from this color.
    #[must_use]
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    pub fn as_hex_lower(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }

    /// Get the RGB tuple of this color.
    #[must_use]
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    pub const fn rgb(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    /// Get the **Red** value of this color.
    #[must_use]
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    pub const fn r(&self) -> u8 {
        self.0
    }

    /// Get the **Green** value of this color.
    #[must_use]
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    pub const fn g(&self) -> u8 {
        self.1
    }

    /// Get the **Blue** value of this color.
    #[must_use]
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    pub const fn b(&self) -> u8 {
        self.2
    }

    /// Converts an ANSI-256 color number to an rgb [`Color`].
    #[allow(clippy::match_same_arms, clippy::too_many_lines)]
    #[must_use]
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    pub const fn ansi_256_to_color(input: u8) -> Self {
        let (r, g, b): (u8, u8, u8) = match input {
            //    8-bit, RGB hex
            // Primary 3-bit (8 colors). Unique representation!
            0 => (0x00, 0x00, 0x00),
            1 => (0x80, 0x00, 0x00),
            2 => (0x00, 0x80, 0x00),
            3 => (0x80, 0x80, 0x00),
            4 => (0x00, 0x00, 0x80),
            5 => (0x80, 0x00, 0x80),
            6 => (0x00, 0x80, 0x80),
            7 => (0xc0, 0xc0, 0xc0),

            // Equivalent "bright" versions of original 8 colors.
            8 => (0x80, 0x80, 0x80),
            9 => (0xff, 0x00, 0x00),
            10 => (0x00, 0xff, 0x00),
            11 => (0xff, 0xff, 0x00),
            12 => (0x00, 0x00, 0xff),
            13 => (0xff, 0x00, 0xff),
            14 => (0x00, 0xff, 0xff),
            15 => (0xff, 0xff, 0xff),

            // Strictly ascending.
            16 => (0x00, 0x00, 0x00),
            17 => (0x00, 0x00, 0x5f),
            18 => (0x00, 0x00, 0x87),
            19 => (0x00, 0x00, 0xaf),
            20 => (0x00, 0x00, 0xd7),
            21 => (0x00, 0x00, 0xff),
            22 => (0x00, 0x5f, 0x00),
            23 => (0x00, 0x5f, 0x5f),
            24 => (0x00, 0x5f, 0x87),
            25 => (0x00, 0x5f, 0xaf),
            26 => (0x00, 0x5f, 0xd7),
            27 => (0x00, 0x5f, 0xff),
            28 => (0x00, 0x87, 0x00),
            29 => (0x00, 0x87, 0x5f),
            30 => (0x00, 0x87, 0x87),
            31 => (0x00, 0x87, 0xaf),
            32 => (0x00, 0x87, 0xd7),
            33 => (0x00, 0x87, 0xff),
            34 => (0x00, 0xaf, 0x00),
            35 => (0x00, 0xaf, 0x5f),
            36 => (0x00, 0xaf, 0x87),
            37 => (0x00, 0xaf, 0xaf),
            38 => (0x00, 0xaf, 0xd7),
            39 => (0x00, 0xaf, 0xff),
            40 => (0x00, 0xd7, 0x00),
            41 => (0x00, 0xd7, 0x5f),
            42 => (0x00, 0xd7, 0x87),
            43 => (0x00, 0xd7, 0xaf),
            44 => (0x00, 0xd7, 0xd7),
            45 => (0x00, 0xd7, 0xff),
            46 => (0x00, 0xff, 0x00),
            47 => (0x00, 0xff, 0x5f),
            48 => (0x00, 0xff, 0x87),
            49 => (0x00, 0xff, 0xaf),
            50 => (0x00, 0xff, 0xd7),
            51 => (0x00, 0xff, 0xff),
            52 => (0x5f, 0x00, 0x00),
            53 => (0x5f, 0x00, 0x5f),
            54 => (0x5f, 0x00, 0x87),
            55 => (0x5f, 0x00, 0xaf),
            56 => (0x5f, 0x00, 0xd7),
            57 => (0x5f, 0x00, 0xff),
            58 => (0x5f, 0x5f, 0x00),
            59 => (0x5f, 0x5f, 0x5f),
            60 => (0x5f, 0x5f, 0x87),
            61 => (0x5f, 0x5f, 0xaf),
            62 => (0x5f, 0x5f, 0xd7),
            63 => (0x5f, 0x5f, 0xff),
            64 => (0x5f, 0x87, 0x00),
            65 => (0x5f, 0x87, 0x5f),
            66 => (0x5f, 0x87, 0x87),
            67 => (0x5f, 0x87, 0xaf),
            68 => (0x5f, 0x87, 0xd7),
            69 => (0x5f, 0x87, 0xff),
            70 => (0x5f, 0xaf, 0x00),
            71 => (0x5f, 0xaf, 0x5f),
            72 => (0x5f, 0xaf, 0x87),
            73 => (0x5f, 0xaf, 0xaf),
            74 => (0x5f, 0xaf, 0xd7),
            75 => (0x5f, 0xaf, 0xff),
            76 => (0x5f, 0xd7, 0x00),
            77 => (0x5f, 0xd7, 0x5f),
            78 => (0x5f, 0xd7, 0x87),
            79 => (0x5f, 0xd7, 0xaf),
            80 => (0x5f, 0xd7, 0xd7),
            81 => (0x5f, 0xd7, 0xff),
            82 => (0x5f, 0xff, 0x00),
            83 => (0x5f, 0xff, 0x5f),
            84 => (0x5f, 0xff, 0x87),
            85 => (0x5f, 0xff, 0xaf),
            86 => (0x5f, 0xff, 0xd7),
            87 => (0x5f, 0xff, 0xff),
            88 => (0x87, 0x00, 0x00),
            89 => (0x87, 0x00, 0x5f),
            90 => (0x87, 0x00, 0x87),
            91 => (0x87, 0x00, 0xaf),
            92 => (0x87, 0x00, 0xd7),
            93 => (0x87, 0x00, 0xff),
            94 => (0x87, 0x5f, 0x00),
            95 => (0x87, 0x5f, 0x5f),
            96 => (0x87, 0x5f, 0x87),
            97 => (0x87, 0x5f, 0xaf),
            98 => (0x87, 0x5f, 0xd7),
            99 => (0x87, 0x5f, 0xff),
            100 => (0x87, 0x87, 0x00),
            101 => (0x87, 0x87, 0x5f),
            102 => (0x87, 0x87, 0x87),
            103 => (0x87, 0x87, 0xaf),
            104 => (0x87, 0x87, 0xd7),
            105 => (0x87, 0x87, 0xff),
            106 => (0x87, 0xaf, 0x00),
            107 => (0x87, 0xaf, 0x5f),
            108 => (0x87, 0xaf, 0x87),
            109 => (0x87, 0xaf, 0xaf),
            110 => (0x87, 0xaf, 0xd7),
            111 => (0x87, 0xaf, 0xff),
            112 => (0x87, 0xd7, 0x00),
            113 => (0x87, 0xd7, 0x5f),
            114 => (0x87, 0xd7, 0x87),
            115 => (0x87, 0xd7, 0xaf),
            116 => (0x87, 0xd7, 0xd7),
            117 => (0x87, 0xd7, 0xff),
            118 => (0x87, 0xff, 0x00),
            119 => (0x87, 0xff, 0x5f),
            120 => (0x87, 0xff, 0x87),
            121 => (0x87, 0xff, 0xaf),
            122 => (0x87, 0xff, 0xd7),
            123 => (0x87, 0xff, 0xff),
            124 => (0xaf, 0x00, 0x00),
            125 => (0xaf, 0x00, 0x5f),
            126 => (0xaf, 0x00, 0x87),
            127 => (0xaf, 0x00, 0xaf),
            128 => (0xaf, 0x00, 0xd7),
            129 => (0xaf, 0x00, 0xff),
            130 => (0xaf, 0x5f, 0x00),
            131 => (0xaf, 0x5f, 0x5f),
            132 => (0xaf, 0x5f, 0x87),
            133 => (0xaf, 0x5f, 0xaf),
            134 => (0xaf, 0x5f, 0xd7),
            135 => (0xaf, 0x5f, 0xff),
            136 => (0xaf, 0x87, 0x00),
            137 => (0xaf, 0x87, 0x5f),
            138 => (0xaf, 0x87, 0x87),
            139 => (0xaf, 0x87, 0xaf),
            140 => (0xaf, 0x87, 0xd7),
            141 => (0xaf, 0x87, 0xff),
            142 => (0xaf, 0xaf, 0x00),
            143 => (0xaf, 0xaf, 0x5f),
            144 => (0xaf, 0xaf, 0x87),
            145 => (0xaf, 0xaf, 0xaf),
            146 => (0xaf, 0xaf, 0xd7),
            147 => (0xaf, 0xaf, 0xff),
            148 => (0xaf, 0xd7, 0x00),
            149 => (0xaf, 0xd7, 0x5f),
            150 => (0xaf, 0xd7, 0x87),
            151 => (0xaf, 0xd7, 0xaf),
            152 => (0xaf, 0xd7, 0xd7),
            153 => (0xaf, 0xd7, 0xff),
            154 => (0xaf, 0xff, 0x00),
            155 => (0xaf, 0xff, 0x5f),
            156 => (0xaf, 0xff, 0x87),
            157 => (0xaf, 0xff, 0xaf),
            158 => (0xaf, 0xff, 0xd7),
            159 => (0xaf, 0xff, 0xff),
            160 => (0xd7, 0x00, 0x00),
            161 => (0xd7, 0x00, 0x5f),
            162 => (0xd7, 0x00, 0x87),
            163 => (0xd7, 0x00, 0xaf),
            164 => (0xd7, 0x00, 0xd7),
            165 => (0xd7, 0x00, 0xff),
            166 => (0xd7, 0x5f, 0x00),
            167 => (0xd7, 0x5f, 0x5f),
            168 => (0xd7, 0x5f, 0x87),
            169 => (0xd7, 0x5f, 0xaf),
            170 => (0xd7, 0x5f, 0xd7),
            171 => (0xd7, 0x5f, 0xff),
            172 => (0xd7, 0x87, 0x00),
            173 => (0xd7, 0x87, 0x5f),
            174 => (0xd7, 0x87, 0x87),
            175 => (0xd7, 0x87, 0xaf),
            176 => (0xd7, 0x87, 0xd7),
            177 => (0xd7, 0x87, 0xff),
            178 => (0xd7, 0xaf, 0x00),
            179 => (0xd7, 0xaf, 0x5f),
            180 => (0xd7, 0xaf, 0x87),
            181 => (0xd7, 0xaf, 0xaf),
            182 => (0xd7, 0xaf, 0xd7),
            183 => (0xd7, 0xaf, 0xff),
            184 => (0xd7, 0xd7, 0x00),
            185 => (0xd7, 0xd7, 0x5f),
            186 => (0xd7, 0xd7, 0x87),
            187 => (0xd7, 0xd7, 0xaf),
            188 => (0xd7, 0xd7, 0xd7),
            189 => (0xd7, 0xd7, 0xff),
            190 => (0xd7, 0xff, 0x00),
            191 => (0xd7, 0xff, 0x5f),
            192 => (0xd7, 0xff, 0x87),
            193 => (0xd7, 0xff, 0xaf),
            194 => (0xd7, 0xff, 0xd7),
            195 => (0xd7, 0xff, 0xff),
            196 => (0xff, 0x00, 0x00),
            197 => (0xff, 0x00, 0x5f),
            198 => (0xff, 0x00, 0x87),
            199 => (0xff, 0x00, 0xaf),
            200 => (0xff, 0x00, 0xd7),
            201 => (0xff, 0x00, 0xff),
            202 => (0xff, 0x5f, 0x00),
            203 => (0xff, 0x5f, 0x5f),
            204 => (0xff, 0x5f, 0x87),
            205 => (0xff, 0x5f, 0xaf),
            206 => (0xff, 0x5f, 0xd7),
            207 => (0xff, 0x5f, 0xff),
            208 => (0xff, 0x87, 0x00),
            209 => (0xff, 0x87, 0x5f),
            210 => (0xff, 0x87, 0x87),
            211 => (0xff, 0x87, 0xaf),
            212 => (0xff, 0x87, 0xd7),
            213 => (0xff, 0x87, 0xff),
            214 => (0xff, 0xaf, 0x00),
            215 => (0xff, 0xaf, 0x5f),
            216 => (0xff, 0xaf, 0x87),
            217 => (0xff, 0xaf, 0xaf),
            218 => (0xff, 0xaf, 0xd7),
            219 => (0xff, 0xaf, 0xff),
            220 => (0xff, 0xd7, 0x00),
            221 => (0xff, 0xd7, 0x5f),
            222 => (0xff, 0xd7, 0x87),
            223 => (0xff, 0xd7, 0xaf),
            224 => (0xff, 0xd7, 0xd7),
            225 => (0xff, 0xd7, 0xff),
            226 => (0xff, 0xff, 0x00),
            227 => (0xff, 0xff, 0x5f),
            228 => (0xff, 0xff, 0x87),
            229 => (0xff, 0xff, 0xaf),
            230 => (0xff, 0xff, 0xd7),
            231 => (0xff, 0xff, 0xff),

            // Gray-scale range.
            232 => (0x08, 0x08, 0x08),
            233 => (0x12, 0x12, 0x12),
            234 => (0x1c, 0x1c, 0x1c),
            235 => (0x26, 0x26, 0x26),
            236 => (0x30, 0x30, 0x30),
            237 => (0x3a, 0x3a, 0x3a),
            238 => (0x44, 0x44, 0x44),
            239 => (0x4e, 0x4e, 0x4e),
            240 => (0x58, 0x58, 0x58),
            241 => (0x62, 0x62, 0x62),
            242 => (0x6c, 0x6c, 0x6c),
            243 => (0x76, 0x76, 0x76),
            244 => (0x80, 0x80, 0x80),
            245 => (0x8a, 0x8a, 0x8a),
            246 => (0x94, 0x94, 0x94),
            247 => (0x9e, 0x9e, 0x9e),
            248 => (0xa8, 0xa8, 0xa8),
            249 => (0xb2, 0xb2, 0xb2),
            250 => (0xbc, 0xbc, 0xbc),
            251 => (0xc6, 0xc6, 0xc6),
            252 => (0xd0, 0xd0, 0xd0),
            253 => (0xda, 0xda, 0xda),
            254 => (0xe4, 0xe4, 0xe4),
            255 => (0xee, 0xee, 0xee),
        };

        Self::from_rgb(r, g, b)
    }

    /// Converts this color into an [`Ansi`] instance by using it as the **foreground** color.
    #[must_use]
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    pub fn into_ansi(self) -> Ansi {
        Ansi::from_fg(self)
    }
}

/// TODO: Should this be changed?
impl std::fmt::Display for Color {
    #[cfg_attr(feature = "flame_on", flamer::flame("color::Color"))]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (r, g, b) = self.rgb();
        write!(f, "Color({},{},{})", r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn hex() {
        let color = Color::from_rgb(25, 100, 250);
        assert_eq!(color.as_hex(), "#1964FA");
        assert_eq!(color.as_hex_lower(), "#1964fa");
    }

    #[test]
    fn color_from_non_ascii() {
        assert!(Color::from_hex("üßü").is_err());
    }

    #[test]
    fn components() {
        let color = Color::from_rgb(25, 100, 250);
        assert_eq!(color.r(), 25);
        assert_eq!(color.g(), 100);
        assert_eq!(color.b(), 250);
    }

    #[test]
    fn ansi_256_to_color() {
        // Test that all u8s parse parsable without error or panic
        for u in u8::MIN..=u8::MAX {
            let _ = Color::ansi_256_to_color(u);
        }
    }

    #[test]
    fn display() {
        let color = Color::from_rgb(25, 100, 250);
        assert_eq!(color.to_string(), "Color(25,100,250)");
    }

    crate::flame_all_tests!(
        ["color", "tests"],
        hex,
        color_from_non_ascii,
        components,
        ansi_256_to_color,
        display
    );
}
