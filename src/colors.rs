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

/// Trait used to facilitate converting various types to a color.
pub trait ToColor {
    /// Perform the conversion.
    fn to_color(&self) -> Color;
}

/// Wrapper struct around a (u8, u8, u8) tuple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Color(u8, u8, u8);

impl Color {
    /// Create a new color from the given RGB values.
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    /// Attempt to create a new color from the given hexadecimal string.
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

        if !string.is_ascii() {}

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
    pub fn as_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }

    /// Get the RGB tuple of this color.
    pub const fn rgb(&self) -> (u8, u8, u8) {
        (self.0, self.1, self.2)
    }

    /// Get the **Red** value of this color.
    pub const fn r(&self) -> u8 {
        self.0
    }

    /// Get the **Green** value of this color.
    pub const fn g(&self) -> u8 {
        self.1
    }

    /// Get the **Blue** value of this color.
    pub const fn b(&self) -> u8 {
        self.2
    }
}

impl ToColor for Color {
    fn to_color(&self) -> Color {
        *self
    }
}

/// Enum containing known named colors.
pub enum Colors {
    Maroon,
    DarkRed,
    Brown,
    Firebrick,
    Crimson,
    Red,
    Tomato,
    Coral,
    IndianRed,
    LightCoral,
    DarkSalmon,
    Salmon,
    LightSalmon,
    OrangeRed,
    DarkOrange,
    Orange,
    Gold,
    DarkGoldenRod,
    GoldenRod,
    PaleGoldenRod,
    DarkKhaki,
    Khaki,
    Olive,
    Yellow,
    YellowGreen,
    DarkOliveGreen,
    OliveDrab,
    LawnGreen,
    Chartreuse,
    GreenYellow,
    DarkGreen,
    Green,
    ForestGreen,
    Lime,
    LimeGreen,
    LightGreen,
    PaleGreen,
    DarkSeaGreen,
    MediumSpringGreen,
    SpringGreen,
    SeaGreen,
    MediumAquaMarine,
    MediumSeaGreen,
    LightSeaGreen,
    DarkSlateGray,
    Teal,
    DarkCyan,
    Aqua,
    Cyan,
    LightCyan,
    DarkTurquoise,
    Turquoise,
    MediumTurquoise,
    PaleTurquoise,
    AquaMarine,
    PowderBlue,
    CadetBlue,
    SteelBlue,
    CornFlowerBlue,
    DeepSkyBlue,
    DodgerBlue,
    LightBlue,
    SkyBlue,
    LightSkyBlue,
    MidnightBlue,
    Navy,
    DarkBlue,
    MediumBlue,
    Blue,
    RoyalBlue,
    BlueViolet,
    Indigo,
    DarkSlateBlue,
    SlateBlue,
    MediumSlateBlue,
    MediumPurple,
    DarkMagenta,
    DarkViolet,
    DarkOrchid,
    MediumOrchid,
    Purple,
    Thistle,
    Plum,
    Violet,
    Magenta,
    Fuschia,
    Orchid,
    MediumVioletRed,
    PaleVioletRed,
    DeepPink,
    HotPink,
    LightPink,
    Pink,
    AntiqueWhite,
    Beige,
    Bisque,
    BlanchedAlmond,
    Wheat,
    CornSilk,
    LemonChiffon,
    LightGoldenRodYellow,
    LightYellow,
    SaddleBrown,
    Sienna,
    Chocolate,
    Peru,
    SandyBrown,
    BurlyWood,
    Tan,
    RosyBrown,
    Moccasin,
    NavajoWhite,
    PeachPuff,
    MistyRose,
    LavenderBlush,
    Linen,
    OldLace,
    PapayaWhip,
    SeaShell,
    MintCream,
    SlateGray,
    LightSlateGray,
    LightSteelBlue,
    Lavender,
    FloralWhite,
    AliceBlue,
    GhostWhite,
    Honeydew,
    Ivory,
    Azure,
    Snow,
    Black,
    DimGray,
    DimGrey,
    Gray,
    Grey,
    DarkGray,
    DarkGrey,
    Silver,
    LightGray,
    LightGrey,
    Gainsboro,
    WhiteSmoke,
    White,
}

impl Colors {
    /// Get the name of this color.
    pub fn name(&self) -> &str {
        match self {
            Colors::Maroon => "Maroon",
            Colors::DarkRed => "DarkRed",
            Colors::Brown => "Brown",
            Colors::Firebrick => "Firebrick",
            Colors::Crimson => "Crimson",
            Colors::Red => "Red",
            Colors::Tomato => "Tomato",
            Colors::Coral => "Coral",
            Colors::IndianRed => "IndianRed",
            Colors::LightCoral => "LightCoral",
            Colors::DarkSalmon => "DarkSalmon",
            Colors::Salmon => "Salmon",
            Colors::LightSalmon => "LightSalmon",
            Colors::OrangeRed => "OrangeRed",
            Colors::DarkOrange => "DarkOrange",
            Colors::Orange => "Orange",
            Colors::Gold => "Gold",
            Colors::DarkGoldenRod => "DarkGoldenRod",
            Colors::GoldenRod => "GoldenRod",
            Colors::PaleGoldenRod => "PaleGoldenRod",
            Colors::DarkKhaki => "DarkKhaki",
            Colors::Khaki => "Khaki",
            Colors::Olive => "Olive",
            Colors::Yellow => "Yellow",
            Colors::YellowGreen => "YellowGreen",
            Colors::DarkOliveGreen => "DarkOliveGreen",
            Colors::OliveDrab => "OliveDrab",
            Colors::LawnGreen => "LawnGreen",
            Colors::Chartreuse => "Chartreuse",
            Colors::GreenYellow => "GreenYellow",
            Colors::DarkGreen => "DarkGreen",
            Colors::Green => "Green",
            Colors::ForestGreen => "ForestGreen",
            Colors::Lime => "Lime",
            Colors::LimeGreen => "LimeGreen",
            Colors::LightGreen => "LightGreen",
            Colors::PaleGreen => "PaleGreen",
            Colors::DarkSeaGreen => "DarkSeaGreen",
            Colors::MediumSpringGreen => "MediumSpringGreen",
            Colors::SpringGreen => "SpringGreen",
            Colors::SeaGreen => "SeaGreen",
            Colors::MediumAquaMarine => "MediumAquaMarine",
            Colors::MediumSeaGreen => "MediumSeaGreen",
            Colors::LightSeaGreen => "LightSeaGreen",
            Colors::DarkSlateGray => "DarkSlateGray",
            Colors::Teal => "Teal",
            Colors::DarkCyan => "DarkCyan",
            Colors::Aqua => "Aqua",
            Colors::Cyan => "Cyan",
            Colors::LightCyan => "LightCyan",
            Colors::DarkTurquoise => "DarkTurquoise",
            Colors::Turquoise => "Turquoise",
            Colors::MediumTurquoise => "MediumTurquoise",
            Colors::PaleTurquoise => "PaleTurquoise",
            Colors::AquaMarine => "AquaMarine",
            Colors::PowderBlue => "PowderBlue",
            Colors::CadetBlue => "CadetBlue",
            Colors::SteelBlue => "SteelBlue",
            Colors::CornFlowerBlue => "CornFlowerBlue",
            Colors::DeepSkyBlue => "DeepSkyBlue",
            Colors::DodgerBlue => "DodgerBlue",
            Colors::LightBlue => "LightBlue",
            Colors::SkyBlue => "SkyBlue",
            Colors::LightSkyBlue => "LightSkyBlue",
            Colors::MidnightBlue => "MidnightBlue",
            Colors::Navy => "Navy",
            Colors::DarkBlue => "DarkBlue",
            Colors::MediumBlue => "MediumBlue",
            Colors::Blue => "Blue",
            Colors::RoyalBlue => "RoyalBlue",
            Colors::BlueViolet => "BlueViolet",
            Colors::Indigo => "Indigo",
            Colors::DarkSlateBlue => "DarkSlateBlue",
            Colors::SlateBlue => "SlateBlue",
            Colors::MediumSlateBlue => "MediumSlateBlue",
            Colors::MediumPurple => "MediumPurple",
            Colors::DarkMagenta => "DarkMagenta",
            Colors::DarkViolet => "DarkViolet",
            Colors::DarkOrchid => "DarkOrchid",
            Colors::MediumOrchid => "MediumOrchid",
            Colors::Purple => "Purple",
            Colors::Thistle => "Thistle",
            Colors::Plum => "Plum",
            Colors::Violet => "Violet",
            Colors::Magenta => "Magenta",
            Colors::Fuschia => "Fuschia",
            Colors::Orchid => "Orchid",
            Colors::MediumVioletRed => "MediumVioletRed",
            Colors::PaleVioletRed => "PaleVioletRed",
            Colors::DeepPink => "DeepPink",
            Colors::HotPink => "HotPink",
            Colors::LightPink => "LightPink",
            Colors::Pink => "Pink",
            Colors::AntiqueWhite => "AntiqueWhite",
            Colors::Beige => "Beige",
            Colors::Bisque => "Bisque",
            Colors::BlanchedAlmond => "BlanchedAlmond",
            Colors::Wheat => "Wheat",
            Colors::CornSilk => "CornSilk",
            Colors::LemonChiffon => "LemonChiffon",
            Colors::LightGoldenRodYellow => "LightGoldenRodYellow",
            Colors::LightYellow => "LightYellow",
            Colors::SaddleBrown => "SaddleBrown",
            Colors::Sienna => "Sienna",
            Colors::Chocolate => "Chocolate",
            Colors::Peru => "Peru",
            Colors::SandyBrown => "SandyBrown",
            Colors::BurlyWood => "BurlyWood",
            Colors::Tan => "Tan",
            Colors::RosyBrown => "RosyBrown",
            Colors::Moccasin => "Moccasin",
            Colors::NavajoWhite => "NavajoWhite",
            Colors::PeachPuff => "PeachPuff",
            Colors::MistyRose => "MistyRose",
            Colors::LavenderBlush => "LavenderBlush",
            Colors::Linen => "Linen",
            Colors::OldLace => "OldLace",
            Colors::PapayaWhip => "PapayaWhip",
            Colors::SeaShell => "SeaShell",
            Colors::MintCream => "MintCream",
            Colors::SlateGray => "SlateGray",
            Colors::LightSlateGray => "LightSlateGray",
            Colors::LightSteelBlue => "LightSteelBlue",
            Colors::Lavender => "Lavender",
            Colors::FloralWhite => "FloralWhite",
            Colors::AliceBlue => "AliceBlue",
            Colors::GhostWhite => "GhostWhite",
            Colors::Honeydew => "Honeydew",
            Colors::Ivory => "Ivory",
            Colors::Azure => "Azure",
            Colors::Snow => "Snow",
            Colors::Black => "Black",
            Colors::DimGray => "DimGray",
            Colors::DimGrey => "DimGrey",
            Colors::Gray => "Gray",
            Colors::Grey => "Grey",
            Colors::DarkGray => "DarkGray",
            Colors::DarkGrey => "DarkGrey",
            Colors::Silver => "Silver",
            Colors::LightGray => "LightGray",
            Colors::LightGrey => "LightGrey",
            Colors::Gainsboro => "Gainsboro",
            Colors::WhiteSmoke => "WhiteSmoke",
            Colors::White => "White",
        }
    }

    /// Get the RGB values of this color.
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Colors::Maroon => (128, 0, 0),
            Colors::DarkRed => (139, 0, 0),
            Colors::Brown => (165, 42, 42),
            Colors::Firebrick => (178, 34, 34),
            Colors::Crimson => (220, 20, 60),
            Colors::Red => (255, 0, 0),
            Colors::Tomato => (255, 99, 71),
            Colors::Coral => (255, 127, 80),
            Colors::IndianRed => (205, 92, 92),
            Colors::LightCoral => (240, 128, 128),
            Colors::DarkSalmon => (233, 150, 122),
            Colors::Salmon => (250, 128, 114),
            Colors::LightSalmon => (255, 160, 122),
            Colors::OrangeRed => (255, 69, 0),
            Colors::DarkOrange => (255, 140, 0),
            Colors::Orange => (255, 165, 0),
            Colors::Gold => (255, 215, 0),
            Colors::DarkGoldenRod => (184, 134, 11),
            Colors::GoldenRod => (218, 165, 32),
            Colors::PaleGoldenRod => (238, 232, 170),
            Colors::DarkKhaki => (189, 183, 107),
            Colors::Khaki => (240, 230, 140),
            Colors::Olive => (128, 128, 0),
            Colors::Yellow => (255, 255, 0),
            Colors::YellowGreen => (154, 205, 50),
            Colors::DarkOliveGreen => (85, 107, 47),
            Colors::OliveDrab => (107, 142, 35),
            Colors::LawnGreen => (124, 252, 0),
            Colors::Chartreuse => (127, 255, 0),
            Colors::GreenYellow => (173, 255, 47),
            Colors::DarkGreen => (0, 100, 0),
            Colors::Green => (0, 128, 0),
            Colors::ForestGreen => (34, 139, 34),
            Colors::Lime => (0, 255, 0),
            Colors::LimeGreen => (50, 205, 50),
            Colors::LightGreen => (144, 238, 144),
            Colors::PaleGreen => (152, 251, 152),
            Colors::DarkSeaGreen => (143, 188, 143),
            Colors::MediumSpringGreen => (0, 250, 154),
            Colors::SpringGreen => (0, 255, 127),
            Colors::SeaGreen => (46, 139, 87),
            Colors::MediumAquaMarine => (102, 205, 170),
            Colors::MediumSeaGreen => (60, 179, 113),
            Colors::LightSeaGreen => (32, 178, 170),
            Colors::DarkSlateGray => (47, 79, 79),
            Colors::Teal => (0, 128, 128),
            Colors::DarkCyan => (0, 139, 139),
            Colors::Aqua => (0, 255, 255),
            Colors::Cyan => (0, 255, 255),
            Colors::LightCyan => (224, 255, 255),
            Colors::DarkTurquoise => (0, 206, 209),
            Colors::Turquoise => (64, 224, 208),
            Colors::MediumTurquoise => (72, 209, 204),
            Colors::PaleTurquoise => (175, 238, 238),
            Colors::AquaMarine => (127, 255, 212),
            Colors::PowderBlue => (176, 224, 230),
            Colors::CadetBlue => (95, 158, 160),
            Colors::SteelBlue => (70, 130, 180),
            Colors::CornFlowerBlue => (100, 149, 237),
            Colors::DeepSkyBlue => (0, 191, 255),
            Colors::DodgerBlue => (30, 144, 255),
            Colors::LightBlue => (173, 216, 230),
            Colors::SkyBlue => (135, 206, 235),
            Colors::LightSkyBlue => (135, 206, 250),
            Colors::MidnightBlue => (25, 25, 112),
            Colors::Navy => (0, 0, 128),
            Colors::DarkBlue => (0, 0, 139),
            Colors::MediumBlue => (0, 0, 205),
            Colors::Blue => (0, 0, 255),
            Colors::RoyalBlue => (65, 105, 225),
            Colors::BlueViolet => (138, 43, 226),
            Colors::Indigo => (75, 0, 130),
            Colors::DarkSlateBlue => (72, 61, 139),
            Colors::SlateBlue => (106, 90, 205),
            Colors::MediumSlateBlue => (123, 104, 238),
            Colors::MediumPurple => (147, 112, 219),
            Colors::DarkMagenta => (139, 0, 139),
            Colors::DarkViolet => (148, 0, 211),
            Colors::DarkOrchid => (153, 50, 204),
            Colors::MediumOrchid => (186, 85, 211),
            Colors::Purple => (128, 0, 128),
            Colors::Thistle => (216, 191, 216),
            Colors::Plum => (221, 160, 221),
            Colors::Violet => (238, 130, 238),
            Colors::Magenta => (255, 0, 255),
            Colors::Fuschia => (255, 0, 255),
            Colors::Orchid => (218, 112, 214),
            Colors::MediumVioletRed => (199, 21, 133),
            Colors::PaleVioletRed => (219, 112, 147),
            Colors::DeepPink => (255, 20, 147),
            Colors::HotPink => (255, 105, 180),
            Colors::LightPink => (255, 182, 193),
            Colors::Pink => (255, 192, 203),
            Colors::AntiqueWhite => (250, 235, 215),
            Colors::Beige => (245, 245, 220),
            Colors::Bisque => (255, 228, 196),
            Colors::BlanchedAlmond => (255, 235, 205),
            Colors::Wheat => (245, 222, 179),
            Colors::CornSilk => (255, 248, 220),
            Colors::LemonChiffon => (255, 250, 205),
            Colors::LightGoldenRodYellow => (250, 250, 210),
            Colors::LightYellow => (255, 255, 224),
            Colors::SaddleBrown => (139, 69, 19),
            Colors::Sienna => (160, 82, 45),
            Colors::Chocolate => (210, 105, 30),
            Colors::Peru => (205, 133, 63),
            Colors::SandyBrown => (244, 164, 96),
            Colors::BurlyWood => (222, 184, 135),
            Colors::Tan => (210, 180, 140),
            Colors::RosyBrown => (188, 143, 143),
            Colors::Moccasin => (255, 228, 181),
            Colors::NavajoWhite => (255, 222, 173),
            Colors::PeachPuff => (255, 218, 185),
            Colors::MistyRose => (255, 228, 225),
            Colors::LavenderBlush => (255, 240, 245),
            Colors::Linen => (250, 240, 230),
            Colors::OldLace => (253, 245, 230),
            Colors::PapayaWhip => (255, 239, 213),
            Colors::SeaShell => (255, 245, 238),
            Colors::MintCream => (245, 255, 250),
            Colors::SlateGray => (112, 128, 144),
            Colors::LightSlateGray => (119, 136, 153),
            Colors::LightSteelBlue => (176, 196, 222),
            Colors::Lavender => (230, 230, 250),
            Colors::FloralWhite => (255, 250, 240),
            Colors::AliceBlue => (240, 248, 255),
            Colors::GhostWhite => (248, 248, 255),
            Colors::Honeydew => (240, 255, 240),
            Colors::Ivory => (255, 255, 240),
            Colors::Azure => (240, 255, 255),
            Colors::Snow => (255, 250, 250),
            Colors::Black => (0, 0, 0),
            Colors::DimGray => (105, 105, 105),
            Colors::DimGrey => (105, 105, 105),
            Colors::Gray => (128, 128, 128),
            Colors::Grey => (128, 128, 128),
            Colors::DarkGray => (169, 169, 169),
            Colors::DarkGrey => (169, 169, 169),
            Colors::Silver => (192, 192, 192),
            Colors::LightGray => (211, 211, 211),
            Colors::LightGrey => (211, 211, 211),
            Colors::Gainsboro => (220, 220, 220),
            Colors::WhiteSmoke => (245, 245, 245),
            Colors::White => (255, 255, 255),
        }
    }
}

impl ToColor for Colors {
    fn to_color(&self) -> Color {
        self.rgb().into()
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Self(rgb.0, rgb.1, rgb.2)
    }
}

impl ToColor for (u8,u8,u8) {
    fn to_color(&self) -> Color {
        Color(self.0, self.1, self.2)
    }
}

impl TryFrom<&str> for Color {
    type Error = ColorParseError;

    /// Attempts to parse the given string as a hex string into a [`Color`].
    fn try_from(input: &str) -> Result<Self, ColorParseError> {
        Color::from_hex(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_convert_1() {
        let color1 = Color::from_hex("#FF0000").unwrap();
        let color2 = Color::from_hex("FF0000").unwrap();
        let color3 = Color::from_hex("#f00").unwrap();
        assert_eq!(color1.rgb(), (255, 0, 0));
        assert_eq!(color2.rgb(), (255, 0, 0));
        assert_eq!(color3.rgb(), (255, 0, 0));
    }

    #[test]
    #[should_panic]
    fn hex_convert_too_small_panics() {
        let _ = Color::from_hex("#FF00").unwrap();
    }

    #[test]
    #[should_panic]
    fn hex_convert_too_big_panics() {
        let _ = Color::from_hex("#FF00000").unwrap();
    }

    #[test]
    #[should_panic]
    fn hex_convert_bad_char_panics() {
        let _ = Color::from_hex("#FF000G").unwrap();
    }
}
