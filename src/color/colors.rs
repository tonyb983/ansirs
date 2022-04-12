// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::Color;

/// Enum containing known named colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Colors {
    AliceBlue,
    AntiqueWhite,
    Aqua,
    AquaMarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornFlowerBlue,
    CornSilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGoldenRod,
    DarkGray,
    DarkGreen,
    DarkGrey,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DimGrey,
    DodgerBlue,
    Firebrick,
    FloralWhite,
    ForestGreen,
    Fuschia,
    Gainsboro,
    GhostWhite,
    Gold,
    GoldenRod,
    Gray,
    Green,
    GreenYellow,
    Grey,
    Honeydew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenRodYellow,
    LightGray,
    LightGreen,
    LightGrey,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquaMarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenRod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,
}

impl Colors {
    /// Get the name of this color.
    pub fn name(&self) -> &str {
        match self {
            Self::AliceBlue => "AliceBlue",
            Self::AntiqueWhite => "AntiqueWhite",
            Self::Aqua => "Aqua",
            Self::AquaMarine => "AquaMarine",
            Self::Azure => "Azure",
            Self::Beige => "Beige",
            Self::Bisque => "Bisque",
            Self::Black => "Black",
            Self::BlanchedAlmond => "BlanchedAlmond",
            Self::Blue => "Blue",
            Self::BlueViolet => "BlueViolet",
            Self::Brown => "Brown",
            Self::BurlyWood => "BurlyWood",
            Self::CadetBlue => "CadetBlue",
            Self::Chartreuse => "Chartreuse",
            Self::Chocolate => "Chocolate",
            Self::Coral => "Coral",
            Self::CornFlowerBlue => "CornFlowerBlue",
            Self::CornSilk => "CornSilk",
            Self::Crimson => "Crimson",
            Self::Cyan => "Cyan",
            Self::DarkBlue => "DarkBlue",
            Self::DarkCyan => "DarkCyan",
            Self::DarkGoldenRod => "DarkGoldenRod",
            Self::DarkGray => "DarkGray",
            Self::DarkGreen => "DarkGreen",
            Self::DarkGrey => "DarkGrey",
            Self::DarkKhaki => "DarkKhaki",
            Self::DarkMagenta => "DarkMagenta",
            Self::DarkOliveGreen => "DarkOliveGreen",
            Self::DarkOrange => "DarkOrange",
            Self::DarkOrchid => "DarkOrchid",
            Self::DarkRed => "DarkRed",
            Self::DarkSalmon => "DarkSalmon",
            Self::DarkSeaGreen => "DarkSeaGreen",
            Self::DarkSlateBlue => "DarkSlateBlue",
            Self::DarkSlateGray => "DarkSlateGray",
            Self::DarkTurquoise => "DarkTurquoise",
            Self::DarkViolet => "DarkViolet",
            Self::DeepPink => "DeepPink",
            Self::DeepSkyBlue => "DeepSkyBlue",
            Self::DimGray => "DimGray",
            Self::DimGrey => "DimGrey",
            Self::DodgerBlue => "DodgerBlue",
            Self::Firebrick => "Firebrick",
            Self::FloralWhite => "FloralWhite",
            Self::ForestGreen => "ForestGreen",
            Self::Fuschia => "Fuschia",
            Self::Gainsboro => "Gainsboro",
            Self::GhostWhite => "GhostWhite",
            Self::Gold => "Gold",
            Self::GoldenRod => "GoldenRod",
            Self::Gray => "Gray",
            Self::Green => "Green",
            Self::GreenYellow => "GreenYellow",
            Self::Grey => "Grey",
            Self::Honeydew => "Honeydew",
            Self::HotPink => "HotPink",
            Self::IndianRed => "IndianRed",
            Self::Indigo => "Indigo",
            Self::Ivory => "Ivory",
            Self::Khaki => "Khaki",
            Self::Lavender => "Lavender",
            Self::LavenderBlush => "LavenderBlush",
            Self::LawnGreen => "LawnGreen",
            Self::LemonChiffon => "LemonChiffon",
            Self::LightBlue => "LightBlue",
            Self::LightCoral => "LightCoral",
            Self::LightCyan => "LightCyan",
            Self::LightGoldenRodYellow => "LightGoldenRodYellow",
            Self::LightGray => "LightGray",
            Self::LightGreen => "LightGreen",
            Self::LightGrey => "LightGrey",
            Self::LightPink => "LightPink",
            Self::LightSalmon => "LightSalmon",
            Self::LightSeaGreen => "LightSeaGreen",
            Self::LightSkyBlue => "LightSkyBlue",
            Self::LightSlateGray => "LightSlateGray",
            Self::LightSteelBlue => "LightSteelBlue",
            Self::LightYellow => "LightYellow",
            Self::Lime => "Lime",
            Self::LimeGreen => "LimeGreen",
            Self::Linen => "Linen",
            Self::Magenta => "Magenta",
            Self::Maroon => "Maroon",
            Self::MediumAquaMarine => "MediumAquaMarine",
            Self::MediumBlue => "MediumBlue",
            Self::MediumOrchid => "MediumOrchid",
            Self::MediumPurple => "MediumPurple",
            Self::MediumSeaGreen => "MediumSeaGreen",
            Self::MediumSlateBlue => "MediumSlateBlue",
            Self::MediumSpringGreen => "MediumSpringGreen",
            Self::MediumTurquoise => "MediumTurquoise",
            Self::MediumVioletRed => "MediumVioletRed",
            Self::MidnightBlue => "MidnightBlue",
            Self::MintCream => "MintCream",
            Self::MistyRose => "MistyRose",
            Self::Moccasin => "Moccasin",
            Self::NavajoWhite => "NavajoWhite",
            Self::Navy => "Navy",
            Self::OldLace => "OldLace",
            Self::Olive => "Olive",
            Self::OliveDrab => "OliveDrab",
            Self::Orange => "Orange",
            Self::OrangeRed => "OrangeRed",
            Self::Orchid => "Orchid",
            Self::PaleGoldenRod => "PaleGoldenRod",
            Self::PaleGreen => "PaleGreen",
            Self::PaleTurquoise => "PaleTurquoise",
            Self::PaleVioletRed => "PaleVioletRed",
            Self::PapayaWhip => "PapayaWhip",
            Self::PeachPuff => "PeachPuff",
            Self::Peru => "Peru",
            Self::Pink => "Pink",
            Self::Plum => "Plum",
            Self::PowderBlue => "PowderBlue",
            Self::Purple => "Purple",
            Self::Red => "Red",
            Self::RosyBrown => "RosyBrown",
            Self::RoyalBlue => "RoyalBlue",
            Self::SaddleBrown => "SaddleBrown",
            Self::Salmon => "Salmon",
            Self::SandyBrown => "SandyBrown",
            Self::SeaGreen => "SeaGreen",
            Self::SeaShell => "SeaShell",
            Self::Sienna => "Sienna",
            Self::Silver => "Silver",
            Self::SkyBlue => "SkyBlue",
            Self::SlateBlue => "SlateBlue",
            Self::SlateGray => "SlateGray",
            Self::Snow => "Snow",
            Self::SpringGreen => "SpringGreen",
            Self::SteelBlue => "SteelBlue",
            Self::Tan => "Tan",
            Self::Teal => "Teal",
            Self::Thistle => "Thistle",
            Self::Tomato => "Tomato",
            Self::Turquoise => "Turquoise",
            Self::Violet => "Violet",
            Self::Wheat => "Wheat",
            Self::White => "White",
            Self::WhiteSmoke => "WhiteSmoke",
            Self::Yellow => "Yellow",
            Self::YellowGreen => "YellowGreen",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::AliceBlue => Self::AntiqueWhite,
            Self::AntiqueWhite => Self::Aqua,
            Self::Aqua => Self::AquaMarine,
            Self::AquaMarine => Self::Azure,
            Self::Azure => Self::Beige,
            Self::Beige => Self::Bisque,
            Self::Bisque => Self::Black,
            Self::Black => Self::BlanchedAlmond,
            Self::BlanchedAlmond => Self::Blue,
            Self::Blue => Self::BlueViolet,
            Self::BlueViolet => Self::Brown,
            Self::Brown => Self::BurlyWood,
            Self::BurlyWood => Self::CadetBlue,
            Self::CadetBlue => Self::Chartreuse,
            Self::Chartreuse => Self::Chocolate,
            Self::Chocolate => Self::Coral,
            Self::Coral => Self::CornFlowerBlue,
            Self::CornFlowerBlue => Self::CornSilk,
            Self::CornSilk => Self::Crimson,
            Self::Crimson => Self::Cyan,
            Self::Cyan => Self::DarkBlue,
            Self::DarkBlue => Self::DarkCyan,
            Self::DarkCyan => Self::DarkGoldenRod,
            Self::DarkGoldenRod => Self::DarkGray,
            Self::DarkGray => Self::DarkGreen,
            Self::DarkGreen => Self::DarkGrey,
            Self::DarkGrey => Self::DarkKhaki,
            Self::DarkKhaki => Self::DarkMagenta,
            Self::DarkMagenta => Self::DarkOliveGreen,
            Self::DarkOliveGreen => Self::DarkOrange,
            Self::DarkOrange => Self::DarkOrchid,
            Self::DarkOrchid => Self::DarkRed,
            Self::DarkRed => Self::DarkSalmon,
            Self::DarkSalmon => Self::DarkSeaGreen,
            Self::DarkSeaGreen => Self::DarkSlateBlue,
            Self::DarkSlateBlue => Self::DarkSlateGray,
            Self::DarkSlateGray => Self::DarkTurquoise,
            Self::DarkTurquoise => Self::DarkViolet,
            Self::DarkViolet => Self::DeepPink,
            Self::DeepPink => Self::DeepSkyBlue,
            Self::DeepSkyBlue => Self::DimGray,
            Self::DimGray => Self::DimGrey,
            Self::DimGrey => Self::DodgerBlue,
            Self::DodgerBlue => Self::Firebrick,
            Self::Firebrick => Self::FloralWhite,
            Self::FloralWhite => Self::ForestGreen,
            Self::ForestGreen => Self::Fuschia,
            Self::Fuschia => Self::Gainsboro,
            Self::Gainsboro => Self::GhostWhite,
            Self::GhostWhite => Self::Gold,
            Self::Gold => Self::GoldenRod,
            Self::GoldenRod => Self::Gray,
            Self::Gray => Self::Green,
            Self::Green => Self::GreenYellow,
            Self::GreenYellow => Self::Grey,
            Self::Grey => Self::Honeydew,
            Self::Honeydew => Self::HotPink,
            Self::HotPink => Self::IndianRed,
            Self::IndianRed => Self::Indigo,
            Self::Indigo => Self::Ivory,
            Self::Ivory => Self::Khaki,
            Self::Khaki => Self::Lavender,
            Self::Lavender => Self::LavenderBlush,
            Self::LavenderBlush => Self::LawnGreen,
            Self::LawnGreen => Self::LemonChiffon,
            Self::LemonChiffon => Self::LightBlue,
            Self::LightBlue => Self::LightCoral,
            Self::LightCoral => Self::LightCyan,
            Self::LightCyan => Self::LightGoldenRodYellow,
            Self::LightGoldenRodYellow => Self::LightGray,
            Self::LightGray => Self::LightGreen,
            Self::LightGreen => Self::LightGrey,
            Self::LightGrey => Self::LightPink,
            Self::LightPink => Self::LightSalmon,
            Self::LightSalmon => Self::LightSeaGreen,
            Self::LightSeaGreen => Self::LightSkyBlue,
            Self::LightSkyBlue => Self::LightSlateGray,
            Self::LightSlateGray => Self::LightSteelBlue,
            Self::LightSteelBlue => Self::LightYellow,
            Self::LightYellow => Self::Lime,
            Self::Lime => Self::LimeGreen,
            Self::LimeGreen => Self::Linen,
            Self::Linen => Self::Magenta,
            Self::Magenta => Self::Maroon,
            Self::Maroon => Self::MediumAquaMarine,
            Self::MediumAquaMarine => Self::MediumBlue,
            Self::MediumBlue => Self::MediumOrchid,
            Self::MediumOrchid => Self::MediumPurple,
            Self::MediumPurple => Self::MediumSeaGreen,
            Self::MediumSeaGreen => Self::MediumSlateBlue,
            Self::MediumSlateBlue => Self::MediumSpringGreen,
            Self::MediumSpringGreen => Self::MediumTurquoise,
            Self::MediumTurquoise => Self::MediumVioletRed,
            Self::MediumVioletRed => Self::MidnightBlue,
            Self::MidnightBlue => Self::MintCream,
            Self::MintCream => Self::MistyRose,
            Self::MistyRose => Self::Moccasin,
            Self::Moccasin => Self::NavajoWhite,
            Self::NavajoWhite => Self::Navy,
            Self::Navy => Self::OldLace,
            Self::OldLace => Self::Olive,
            Self::Olive => Self::OliveDrab,
            Self::OliveDrab => Self::Orange,
            Self::Orange => Self::OrangeRed,
            Self::OrangeRed => Self::Orchid,
            Self::Orchid => Self::PaleGoldenRod,
            Self::PaleGoldenRod => Self::PaleGreen,
            Self::PaleGreen => Self::PaleTurquoise,
            Self::PaleTurquoise => Self::PaleVioletRed,
            Self::PaleVioletRed => Self::PapayaWhip,
            Self::PapayaWhip => Self::PeachPuff,
            Self::PeachPuff => Self::Peru,
            Self::Peru => Self::Pink,
            Self::Pink => Self::Plum,
            Self::Plum => Self::PowderBlue,
            Self::PowderBlue => Self::Purple,
            Self::Purple => Self::Red,
            Self::Red => Self::RosyBrown,
            Self::RosyBrown => Self::RoyalBlue,
            Self::RoyalBlue => Self::SaddleBrown,
            Self::SaddleBrown => Self::Salmon,
            Self::Salmon => Self::SandyBrown,
            Self::SandyBrown => Self::SeaGreen,
            Self::SeaGreen => Self::SeaShell,
            Self::SeaShell => Self::Sienna,
            Self::Sienna => Self::Silver,
            Self::Silver => Self::SkyBlue,
            Self::SkyBlue => Self::SlateBlue,
            Self::SlateBlue => Self::SlateGray,
            Self::SlateGray => Self::Snow,
            Self::Snow => Self::SpringGreen,
            Self::SpringGreen => Self::SteelBlue,
            Self::SteelBlue => Self::Tan,
            Self::Tan => Self::Teal,
            Self::Teal => Self::Thistle,
            Self::Thistle => Self::Tomato,
            Self::Tomato => Self::Turquoise,
            Self::Turquoise => Self::Violet,
            Self::Violet => Self::Wheat,
            Self::Wheat => Self::White,
            Self::White => Self::WhiteSmoke,
            Self::WhiteSmoke => Self::Yellow,
            Self::Yellow => Self::YellowGreen,
            Self::YellowGreen => Self::AliceBlue,
        }
    }

    /// Get the RGB values of this color.
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Colors::AliceBlue => (240, 248, 255),
            Colors::AntiqueWhite => (250, 235, 215),
            Colors::Aqua => (0, 255, 255),
            Colors::AquaMarine => (127, 255, 212),
            Colors::Azure => (240, 255, 255),
            Colors::Beige => (245, 245, 220),
            Colors::Bisque => (255, 228, 196),
            Colors::Black => (0, 0, 0),
            Colors::BlanchedAlmond => (255, 235, 205),
            Colors::Blue => (0, 0, 255),
            Colors::BlueViolet => (138, 43, 226),
            Colors::Brown => (165, 42, 42),
            Colors::BurlyWood => (222, 184, 135),
            Colors::CadetBlue => (95, 158, 160),
            Colors::Chartreuse => (127, 255, 0),
            Colors::Chocolate => (210, 105, 30),
            Colors::Coral => (255, 127, 80),
            Colors::CornFlowerBlue => (100, 149, 237),
            Colors::CornSilk => (255, 248, 220),
            Colors::Crimson => (220, 20, 60),
            Colors::Cyan => (0, 255, 255),
            Colors::DarkBlue => (0, 0, 139),
            Colors::DarkCyan => (0, 139, 139),
            Colors::DarkGoldenRod => (184, 134, 11),
            Colors::DarkGray => (169, 169, 169),
            Colors::DarkGreen => (0, 100, 0),
            Colors::DarkGrey => (169, 169, 169),
            Colors::DarkKhaki => (189, 183, 107),
            Colors::DarkMagenta => (139, 0, 139),
            Colors::DarkOliveGreen => (85, 107, 47),
            Colors::DarkOrange => (255, 140, 0),
            Colors::DarkOrchid => (153, 50, 204),
            Colors::DarkRed => (139, 0, 0),
            Colors::DarkSalmon => (233, 150, 122),
            Colors::DarkSeaGreen => (143, 188, 143),
            Colors::DarkSlateBlue => (72, 61, 139),
            Colors::DarkSlateGray => (47, 79, 79),
            Colors::DarkTurquoise => (0, 206, 209),
            Colors::DarkViolet => (148, 0, 211),
            Colors::DeepPink => (255, 20, 147),
            Colors::DeepSkyBlue => (0, 191, 255),
            Colors::DimGray => (105, 105, 105),
            Colors::DimGrey => (105, 105, 105),
            Colors::DodgerBlue => (30, 144, 255),
            Colors::Firebrick => (178, 34, 34),
            Colors::FloralWhite => (255, 250, 240),
            Colors::ForestGreen => (34, 139, 34),
            Colors::Fuschia => (255, 0, 255),
            Colors::Gainsboro => (220, 220, 220),
            Colors::GhostWhite => (248, 248, 255),
            Colors::Gold => (255, 215, 0),
            Colors::GoldenRod => (218, 165, 32),
            Colors::Gray => (128, 128, 128),
            Colors::Green => (0, 128, 0),
            Colors::GreenYellow => (173, 255, 47),
            Colors::Grey => (128, 128, 128),
            Colors::Honeydew => (240, 255, 240),
            Colors::HotPink => (255, 105, 180),
            Colors::IndianRed => (205, 92, 92),
            Colors::Indigo => (75, 0, 130),
            Colors::Ivory => (255, 255, 240),
            Colors::Khaki => (240, 230, 140),
            Colors::Lavender => (230, 230, 250),
            Colors::LavenderBlush => (255, 240, 245),
            Colors::LawnGreen => (124, 252, 0),
            Colors::LemonChiffon => (255, 250, 205),
            Colors::LightBlue => (173, 216, 230),
            Colors::LightCoral => (240, 128, 128),
            Colors::LightCyan => (224, 255, 255),
            Colors::LightGoldenRodYellow => (250, 250, 210),
            Colors::LightGray => (211, 211, 211),
            Colors::LightGreen => (144, 238, 144),
            Colors::LightGrey => (211, 211, 211),
            Colors::LightPink => (255, 182, 193),
            Colors::LightSalmon => (255, 160, 122),
            Colors::LightSeaGreen => (32, 178, 170),
            Colors::LightSkyBlue => (135, 206, 250),
            Colors::LightSlateGray => (119, 136, 153),
            Colors::LightSteelBlue => (176, 196, 222),
            Colors::LightYellow => (255, 255, 224),
            Colors::Lime => (0, 255, 0),
            Colors::LimeGreen => (50, 205, 50),
            Colors::Linen => (250, 240, 230),
            Colors::Magenta => (255, 0, 255),
            Colors::Maroon => (128, 0, 0),
            Colors::MediumAquaMarine => (102, 205, 170),
            Colors::MediumBlue => (0, 0, 205),
            Colors::MediumOrchid => (186, 85, 211),
            Colors::MediumPurple => (147, 112, 219),
            Colors::MediumSeaGreen => (60, 179, 113),
            Colors::MediumSlateBlue => (123, 104, 238),
            Colors::MediumSpringGreen => (0, 250, 154),
            Colors::MediumTurquoise => (72, 209, 204),
            Colors::MediumVioletRed => (199, 21, 133),
            Colors::MidnightBlue => (25, 25, 112),
            Colors::MintCream => (245, 255, 250),
            Colors::MistyRose => (255, 228, 225),
            Colors::Moccasin => (255, 228, 181),
            Colors::NavajoWhite => (255, 222, 173),
            Colors::Navy => (0, 0, 128),
            Colors::OldLace => (253, 245, 230),
            Colors::Olive => (128, 128, 0),
            Colors::OliveDrab => (107, 142, 35),
            Colors::Orange => (255, 165, 0),
            Colors::OrangeRed => (255, 69, 0),
            Colors::Orchid => (218, 112, 214),
            Colors::PaleGoldenRod => (238, 232, 170),
            Colors::PaleGreen => (152, 251, 152),
            Colors::PaleTurquoise => (175, 238, 238),
            Colors::PaleVioletRed => (219, 112, 147),
            Colors::PapayaWhip => (255, 239, 213),
            Colors::PeachPuff => (255, 218, 185),
            Colors::Peru => (205, 133, 63),
            Colors::Pink => (255, 192, 203),
            Colors::Plum => (221, 160, 221),
            Colors::PowderBlue => (176, 224, 230),
            Colors::Purple => (128, 0, 128),
            Colors::Red => (255, 0, 0),
            Colors::RosyBrown => (188, 143, 143),
            Colors::RoyalBlue => (65, 105, 225),
            Colors::SaddleBrown => (139, 69, 19),
            Colors::Salmon => (250, 128, 114),
            Colors::SandyBrown => (244, 164, 96),
            Colors::SeaGreen => (46, 139, 87),
            Colors::SeaShell => (255, 245, 238),
            Colors::Sienna => (160, 82, 45),
            Colors::Silver => (192, 192, 192),
            Colors::SkyBlue => (135, 206, 235),
            Colors::SlateBlue => (106, 90, 205),
            Colors::SlateGray => (112, 128, 144),
            Colors::Snow => (255, 250, 250),
            Colors::SpringGreen => (0, 255, 127),
            Colors::SteelBlue => (70, 130, 180),
            Colors::Tan => (210, 180, 140),
            Colors::Teal => (0, 128, 128),
            Colors::Thistle => (216, 191, 216),
            Colors::Tomato => (255, 99, 71),
            Colors::Turquoise => (64, 224, 208),
            Colors::Violet => (238, 130, 238),
            Colors::Wheat => (245, 222, 179),
            Colors::White => (255, 255, 255),
            Colors::WhiteSmoke => (245, 245, 245),
            Colors::Yellow => (255, 255, 0),
            Colors::YellowGreen => (154, 205, 50),
        }
    }

    pub fn into_color(self) -> Color {
        let (r, g, b) = self.rgb();
        Color::from_rgb(r, g, b)
    }

    pub fn all() -> impl Iterator<Item = Self> {
        Self::AliceBlue.into_iter()
    }
}

impl IntoIterator for Colors {
    type Item = Self;

    type IntoIter = crate::iter::ColorsIter;

    fn into_iter(self) -> Self::IntoIter {
        crate::iter::ColorsIter::starting_with(self)
    }
}
