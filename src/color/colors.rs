// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

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
    Fuchsia,
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
    pub const fn name(&self) -> &'static str {
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
            Self::Fuchsia => "Fuchsia",
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

    /// Try to get a named color from the given string.
    /// #### Non-const function.
    pub fn from_name(input: &str) -> Option<Self> {
        match input {
            "AliceBlue" | "Alice Blue" => Some(Self::AliceBlue),
            "AntiqueWhite" | "Antique White" => Some(Self::AntiqueWhite),
            "Aqua" => Some(Self::Aqua),
            "AquaMarine" | "Aqua Marine" => Some(Self::AquaMarine),
            "Azure" => Some(Self::Azure),
            "Beige" => Some(Self::Beige),
            "Bisque" => Some(Self::Bisque),
            "Black" => Some(Self::Black),
            "BlanchedAlmond" | "Blanched Almond" => Some(Self::BlanchedAlmond),
            "Blue" => Some(Self::Blue),
            "BlueViolet" | "Blue Violet" => Some(Self::BlueViolet),
            "Brown" => Some(Self::Brown),
            "BurlyWood" | "Burly Wood" => Some(Self::BurlyWood),
            "CadetBlue" | "Cadet Blue" => Some(Self::CadetBlue),
            "Chartreuse" => Some(Self::Chartreuse),
            "Chocolate" => Some(Self::Chocolate),
            "Coral" => Some(Self::Coral),
            "CornFlowerBlue" | "Corn Flower Blue" => Some(Self::CornFlowerBlue),
            "CornSilk" | "Corn Silk" => Some(Self::CornSilk),
            "Crimson" => Some(Self::Crimson),
            "Cyan" => Some(Self::Cyan),
            "DarkBlue" | "Dark Blue" => Some(Self::DarkBlue),
            "DarkCyan" | "Dark Cyan" => Some(Self::DarkCyan),
            "DarkGoldenRod" | "Dark Golden Rod" => Some(Self::DarkGoldenRod),
            "DarkGray" | "Dark Gray" => Some(Self::DarkGray),
            "DarkGreen" | "Dark Green" => Some(Self::DarkGreen),
            "DarkGrey" | "Dark Grey" => Some(Self::DarkGrey),
            "DarkKhaki" | "Dark Khaki" => Some(Self::DarkKhaki),
            "DarkMagenta" | "Dark Magenta" => Some(Self::DarkMagenta),
            "DarkOliveGreen" | "Dark Olive Green" => Some(Self::DarkOliveGreen),
            "DarkOrange" | "Dark Orange" => Some(Self::DarkOrange),
            "DarkOrchid" | "Dark Orchid" => Some(Self::DarkOrchid),
            "DarkRed" | "Dark Red" => Some(Self::DarkRed),
            "DarkSalmon" | "Dark Salmon" => Some(Self::DarkSalmon),
            "DarkSeaGreen" | "Dark Sea Green" => Some(Self::DarkSeaGreen),
            "DarkSlateBlue" | "Dark Slate Blue" => Some(Self::DarkSlateBlue),
            "DarkSlateGray" | "Dark Slate Gray" => Some(Self::DarkSlateGray),
            "DarkTurquoise" | "Dark Turquoise" => Some(Self::DarkTurquoise),
            "DarkViolet" | "Dark Violet" => Some(Self::DarkViolet),
            "DeepPink" | "Deep Pink" => Some(Self::DeepPink),
            "DeepSkyBlue" | "Deep Sky Blue" => Some(Self::DeepSkyBlue),
            "DimGray" | "Dim Gray" => Some(Self::DimGray),
            "DimGrey" | "Dim Grey" => Some(Self::DimGrey),
            "DodgerBlue" | "Dodger Blue" => Some(Self::DodgerBlue),
            "Firebrick" => Some(Self::Firebrick),
            "FloralWhite" | "Floral White" => Some(Self::FloralWhite),
            "ForestGreen" | "Forest Green" => Some(Self::ForestGreen),
            "Fuchsia" => Some(Self::Fuchsia),
            "Gainsboro" => Some(Self::Gainsboro),
            "GhostWhite" | "Ghost White" => Some(Self::GhostWhite),
            "Gold" => Some(Self::Gold),
            "GoldenRod" | "Golden Rod" => Some(Self::GoldenRod),
            "Gray" => Some(Self::Gray),
            "Green" => Some(Self::Green),
            "GreenYellow" | "Green Yellow" => Some(Self::GreenYellow),
            "Grey" => Some(Self::Grey),
            "Honeydew" => Some(Self::Honeydew),
            "HotPink" | "Hot Pink" => Some(Self::HotPink),
            "IndianRed" | "Indian Red" => Some(Self::IndianRed),
            "Indigo" => Some(Self::Indigo),
            "Ivory" => Some(Self::Ivory),
            "Khaki" => Some(Self::Khaki),
            "Lavender" => Some(Self::Lavender),
            "LavenderBlush" | "Lavender Blush" => Some(Self::LavenderBlush),
            "LawnGreen" | "Lawn Green" => Some(Self::LawnGreen),
            "LemonChiffon" | "Lemon Chiffon" => Some(Self::LemonChiffon),
            "LightBlue" | "Light Blue" => Some(Self::LightBlue),
            "LightCoral" | "Light Coral" => Some(Self::LightCoral),
            "LightCyan" | "Light Cyan" => Some(Self::LightCyan),
            "LightGoldenRodYellow" | "Light Golden Rod Yellow" => Some(Self::LightGoldenRodYellow),
            "LightGray" | "Light Gray" => Some(Self::LightGray),
            "LightGreen" | "Light Green" => Some(Self::LightGreen),
            "LightGrey" | "Light Grey" => Some(Self::LightGrey),
            "LightPink" | "Light Pink" => Some(Self::LightPink),
            "LightSalmon" | "Light Salmon" => Some(Self::LightSalmon),
            "LightSeaGreen" | "Light Sea Green" => Some(Self::LightSeaGreen),
            "LightSkyBlue" | "Light Sky Blue" => Some(Self::LightSkyBlue),
            "LightSlateGray" | "Light Slate Gray" => Some(Self::LightSlateGray),
            "LightSteelBlue" | "Light Steel Blue" => Some(Self::LightSteelBlue),
            "LightYellow" | "Light Yellow" => Some(Self::LightYellow),
            "Lime" => Some(Self::Lime),
            "LimeGreen" | "Lime Green" => Some(Self::LimeGreen),
            "Linen" => Some(Self::Linen),
            "Magenta" => Some(Self::Magenta),
            "Maroon" => Some(Self::Maroon),
            "MediumAquaMarine" | "Medium Aqua Marine" => Some(Self::MediumAquaMarine),
            "MediumBlue" | "Medium Blue" => Some(Self::MediumBlue),
            "MediumOrchid" | "Medium Orchid" => Some(Self::MediumOrchid),
            "MediumPurple" | "Medium Purple" => Some(Self::MediumPurple),
            "MediumSeaGreen" | "Medium Sea Green" => Some(Self::MediumSeaGreen),
            "MediumSlateBlue" | "Medium Slate Blue" => Some(Self::MediumSlateBlue),
            "MediumSpringGreen" | "Medium Spring Green" => Some(Self::MediumSpringGreen),
            "MediumTurquoise" | "Medium Turquoise" => Some(Self::MediumTurquoise),
            "MediumVioletRed" | "Medium Violet Red" => Some(Self::MediumVioletRed),
            "MidnightBlue" | "Midnight Blue" => Some(Self::MidnightBlue),
            "MintCream" | "Mint Cream" => Some(Self::MintCream),
            "MistyRose" | "Misty Rose" => Some(Self::MistyRose),
            "Moccasin" => Some(Self::Moccasin),
            "NavajoWhite" | "Navajo White" => Some(Self::NavajoWhite),
            "Navy" => Some(Self::Navy),
            "OldLace" | "Old Lace" => Some(Self::OldLace),
            "Olive" => Some(Self::Olive),
            "OliveDrab" | "Olive Drab" => Some(Self::OliveDrab),
            "Orange" => Some(Self::Orange),
            "OrangeRed" | "Orange Red" => Some(Self::OrangeRed),
            "Orchid" => Some(Self::Orchid),
            "PaleGoldenRod" | "Pale Golden Rod" => Some(Self::PaleGoldenRod),
            "PaleGreen" | "Pale Green" => Some(Self::PaleGreen),
            "PaleTurquoise" | "Pale Turquoise" => Some(Self::PaleTurquoise),
            "PaleVioletRed" | "Pale Violet Red" => Some(Self::PaleVioletRed),
            "PapayaWhip" | "Papaya Whip" => Some(Self::PapayaWhip),
            "PeachPuff" | "Peach Puff" => Some(Self::PeachPuff),
            "Peru" => Some(Self::Peru),
            "Pink" => Some(Self::Pink),
            "Plum" => Some(Self::Plum),
            "PowderBlue" | "Powder Blue" => Some(Self::PowderBlue),
            "Purple" => Some(Self::Purple),
            "Red" => Some(Self::Red),
            "RosyBrown" | "Rosy Brown" => Some(Self::RosyBrown),
            "RoyalBlue" | "Royal Blue" => Some(Self::RoyalBlue),
            "SaddleBrown" | "Saddle Brown" => Some(Self::SaddleBrown),
            "Salmon" => Some(Self::Salmon),
            "SandyBrown" | "Sandy Brown" => Some(Self::SandyBrown),
            "SeaGreen" | "Sea Green" => Some(Self::SeaGreen),
            "SeaShell" | "Sea Shell" => Some(Self::SeaShell),
            "Sienna" => Some(Self::Sienna),
            "Silver" => Some(Self::Silver),
            "SkyBlue" | "Sky Blue" => Some(Self::SkyBlue),
            "SlateBlue" | "Slate Blue" => Some(Self::SlateBlue),
            "SlateGray" | "Slate Gray" => Some(Self::SlateGray),
            "Snow" => Some(Self::Snow),
            "SpringGreen" | "Spring Green" => Some(Self::SpringGreen),
            "SteelBlue" | "Steel Blue" => Some(Self::SteelBlue),
            "Tan" => Some(Self::Tan),
            "Teal" => Some(Self::Teal),
            "Thistle" => Some(Self::Thistle),
            "Tomato" => Some(Self::Tomato),
            "Turquoise" => Some(Self::Turquoise),
            "Violet" => Some(Self::Violet),
            "Wheat" => Some(Self::Wheat),
            "White" => Some(Self::White),
            "WhiteSmoke" | "White Smoke" => Some(Self::WhiteSmoke),
            "Yellow" => Some(Self::Yellow),
            "YellowGreen" | "Yellow Green" => Some(Self::YellowGreen),
            _ => None,
        }
    }

    /// Try to get a named color from the given name (compared as lowercase strings).
    /// #### Non-const function.
    pub fn from_name_ignore_case(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "aliceblue" | "alice blue" => Some(Self::AliceBlue),
            "antiquewhite" | "antique white" => Some(Self::AntiqueWhite),
            "aqua" => Some(Self::Aqua),
            "aquamarine" | "aqua marine" => Some(Self::AquaMarine),
            "azure" => Some(Self::Azure),
            "beige" => Some(Self::Beige),
            "bisque" => Some(Self::Bisque),
            "black" => Some(Self::Black),
            "blanchedalmond" | "blanched almond" => Some(Self::BlanchedAlmond),
            "blue" => Some(Self::Blue),
            "blueviolet" | "blue violet" => Some(Self::BlueViolet),
            "brown" => Some(Self::Brown),
            "burlywood" | "burly wood" => Some(Self::BurlyWood),
            "cadetblue" | "cadet blue" => Some(Self::CadetBlue),
            "chartreuse" => Some(Self::Chartreuse),
            "chocolate" => Some(Self::Chocolate),
            "coral" => Some(Self::Coral),
            "cornflowerblue" | "corn flower blue" => Some(Self::CornFlowerBlue),
            "cornsilk" | "corn silk" => Some(Self::CornSilk),
            "crimson" => Some(Self::Crimson),
            "cyan" => Some(Self::Cyan),
            "darkblue" | "dark blue" => Some(Self::DarkBlue),
            "darkcyan" | "dark cyan" => Some(Self::DarkCyan),
            "darkgoldenrod" | "dark golden rod" => Some(Self::DarkGoldenRod),
            "darkgray" | "dark gray" => Some(Self::DarkGray),
            "darkgreen" | "dark green" => Some(Self::DarkGreen),
            "darkgrey" | "dark grey" => Some(Self::DarkGrey),
            "darkkhaki" | "dark khaki" => Some(Self::DarkKhaki),
            "darkmagenta" | "dark magenta" => Some(Self::DarkMagenta),
            "darkolivegreen" | "dark olive green" => Some(Self::DarkOliveGreen),
            "darkorange" | "dark orange" => Some(Self::DarkOrange),
            "darkorchid" | "dark orchid" => Some(Self::DarkOrchid),
            "darkred" | "dark red" => Some(Self::DarkRed),
            "darksalmon" | "dark salmon" => Some(Self::DarkSalmon),
            "darkseagreen" | "dark sea green" => Some(Self::DarkSeaGreen),
            "darkslateblue" | "dark slate blue" => Some(Self::DarkSlateBlue),
            "darkslategray" | "dark slate gray" => Some(Self::DarkSlateGray),
            "darkturquoise" | "dark turquoise" => Some(Self::DarkTurquoise),
            "darkviolet" | "dark violet" => Some(Self::DarkViolet),
            "deeppink" | "deep pink" => Some(Self::DeepPink),
            "deepskyblue" | "deep sky blue" => Some(Self::DeepSkyBlue),
            "dimgray" | "dim gray" => Some(Self::DimGray),
            "dimgrey" | "dim grey" => Some(Self::DimGrey),
            "dodgerblue" | "dodger blue" => Some(Self::DodgerBlue),
            "firebrick" => Some(Self::Firebrick),
            "floralwhite" | "floral white" => Some(Self::FloralWhite),
            "forestgreen" | "forest green" => Some(Self::ForestGreen),
            "fuchsia" => Some(Self::Fuchsia),
            "gainsboro" => Some(Self::Gainsboro),
            "ghostwhite" | "ghost white" => Some(Self::GhostWhite),
            "gold" => Some(Self::Gold),
            "goldenrod" | "golden rod" => Some(Self::GoldenRod),
            "gray" => Some(Self::Gray),
            "green" => Some(Self::Green),
            "greenyellow" | "green yellow" => Some(Self::GreenYellow),
            "grey" => Some(Self::Grey),
            "honeydew" => Some(Self::Honeydew),
            "hotpink" | "hot pink" => Some(Self::HotPink),
            "indianred" | "indian red" => Some(Self::IndianRed),
            "indigo" => Some(Self::Indigo),
            "ivory" => Some(Self::Ivory),
            "khaki" => Some(Self::Khaki),
            "lavender" => Some(Self::Lavender),
            "lavenderblush" | "lavender blush" => Some(Self::LavenderBlush),
            "lawngreen" | "lawn green" => Some(Self::LawnGreen),
            "lemonchiffon" | "lemon chiffon" => Some(Self::LemonChiffon),
            "lightblue" | "light blue" => Some(Self::LightBlue),
            "lightcoral" | "light coral" => Some(Self::LightCoral),
            "lightcyan" | "light cyan" => Some(Self::LightCyan),
            "lightgoldenrodyellow" | "light golden rod yellow" => Some(Self::LightGoldenRodYellow),
            "lightgray" | "light gray" => Some(Self::LightGray),
            "lightgreen" | "light green" => Some(Self::LightGreen),
            "lightgrey" | "light grey" => Some(Self::LightGrey),
            "lightpink" | "light pink" => Some(Self::LightPink),
            "lightsalmon" | "light salmon" => Some(Self::LightSalmon),
            "lightseagreen" | "light sea green" => Some(Self::LightSeaGreen),
            "lightskyblue" | "light sky blue" => Some(Self::LightSkyBlue),
            "lightslategray" | "light slate gray" => Some(Self::LightSlateGray),
            "lightsteelblue" | "light steel blue" => Some(Self::LightSteelBlue),
            "lightyellow" | "light yellow" => Some(Self::LightYellow),
            "lime" => Some(Self::Lime),
            "limegreen" | "lime green" => Some(Self::LimeGreen),
            "linen" => Some(Self::Linen),
            "magenta" => Some(Self::Magenta),
            "maroon" => Some(Self::Maroon),
            "mediumaquamarine" | "medium aqua marine" => Some(Self::MediumAquaMarine),
            "mediumblue" | "medium blue" => Some(Self::MediumBlue),
            "mediumorchid" | "medium orchid" => Some(Self::MediumOrchid),
            "mediumpurple" | "medium purple" => Some(Self::MediumPurple),
            "mediumseagreen" | "medium sea green" => Some(Self::MediumSeaGreen),
            "mediumslateblue" | "medium slate blue" => Some(Self::MediumSlateBlue),
            "mediumspringgreen" | "medium spring green" => Some(Self::MediumSpringGreen),
            "mediumturquoise" | "medium turquoise" => Some(Self::MediumTurquoise),
            "mediumvioletred" | "medium violet red" => Some(Self::MediumVioletRed),
            "midnightblue" | "midnight blue" => Some(Self::MidnightBlue),
            "mintcream" | "mint cream" => Some(Self::MintCream),
            "mistyrose" | "misty rose" => Some(Self::MistyRose),
            "moccasin" => Some(Self::Moccasin),
            "navajowhite" | "navajo white" => Some(Self::NavajoWhite),
            "navy" => Some(Self::Navy),
            "oldlace" | "old lace" => Some(Self::OldLace),
            "olive" => Some(Self::Olive),
            "olivedrab" | "olive drab" => Some(Self::OliveDrab),
            "orange" => Some(Self::Orange),
            "orangered" | "orange red" => Some(Self::OrangeRed),
            "orchid" => Some(Self::Orchid),
            "palegoldenrod" | "pale golden rod" => Some(Self::PaleGoldenRod),
            "palegreen" | "pale green" => Some(Self::PaleGreen),
            "paleturquoise" | "pale turquoise" => Some(Self::PaleTurquoise),
            "palevioletred" | "pale violet red" => Some(Self::PaleVioletRed),
            "papayawhip" | "papaya whip" => Some(Self::PapayaWhip),
            "peachpuff" | "peach puff" => Some(Self::PeachPuff),
            "peru" => Some(Self::Peru),
            "pink" => Some(Self::Pink),
            "plum" => Some(Self::Plum),
            "powderblue" | "powder blue" => Some(Self::PowderBlue),
            "purple" => Some(Self::Purple),
            "red" => Some(Self::Red),
            "rosybrown" | "rosy brown" => Some(Self::RosyBrown),
            "royalblue" | "royal blue" => Some(Self::RoyalBlue),
            "saddlebrown" | "saddle brown" => Some(Self::SaddleBrown),
            "salmon" => Some(Self::Salmon),
            "sandybrown" | "sandy brown" => Some(Self::SandyBrown),
            "seagreen" | "sea green" => Some(Self::SeaGreen),
            "seashell" | "sea shell" => Some(Self::SeaShell),
            "sienna" => Some(Self::Sienna),
            "silver" => Some(Self::Silver),
            "skyblue" | "sky blue" => Some(Self::SkyBlue),
            "slateblue" | "slate blue" => Some(Self::SlateBlue),
            "slategray" | "slate gray" => Some(Self::SlateGray),
            "snow" => Some(Self::Snow),
            "springgreen" | "spring green" => Some(Self::SpringGreen),
            "steelblue" | "steel blue" => Some(Self::SteelBlue),
            "tan" => Some(Self::Tan),
            "teal" => Some(Self::Teal),
            "thistle" => Some(Self::Thistle),
            "tomato" => Some(Self::Tomato),
            "turquoise" => Some(Self::Turquoise),
            "violet" => Some(Self::Violet),
            "wheat" => Some(Self::Wheat),
            "white" => Some(Self::White),
            "whitesmoke" | "white smoke" => Some(Self::WhiteSmoke),
            "yellow" => Some(Self::Yellow),
            "yellowgreen" | "yellow green" => Some(Self::YellowGreen),
            _ => None,
        }
    }

    /// Get the next named color. Pretty much only useful for iteration.
    ///
    /// TODO: This should probably be private?
    pub const fn next(self) -> Self {
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
            Self::ForestGreen => Self::Fuchsia,
            Self::Fuchsia => Self::Gainsboro,
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
    pub const fn rgb(&self) -> (u8, u8, u8) {
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
            Colors::Fuchsia => (255, 0, 255),
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

    /// Creates a [`Color`] from this named [`Colors`].
    pub const fn into_color(self) -> Color {
        let (r, g, b) = self.rgb();
        Color::from_rgb(r, g, b)
    }

    /// Creates an iterator over all named colors, in alphabetical order, starting
    /// with [`Colors::AliceBlue`] and ending with [`Colors::YellowGreen`].
    pub fn all() -> impl Iterator<Item = Self> {
        Self::AliceBlue.into_iter()
    }

    /// Gets the closest named color to the rgb values given using simple absolute difference calculations
    ///
    /// TODO: This can probably be better right?
    pub fn get_closest_color(input: (u8, u8, u8)) -> Option<(Self, usize)> {
        fn abs_diff(a: usize, b: usize) -> usize {
            if a > b {
                a - b
            } else {
                b - a
            }
        }
        let (r, g, b) = input;
        let r = r as usize;
        let g = g as usize;
        let b = b as usize;
        Self::all()
            .map(|c| {
                let (cr, cg, cb) = c.rgb();
                let cr = cr as usize;
                let cg = cg as usize;
                let cb = cb as usize;
                (c, abs_diff(r, cr) + abs_diff(g, cg) + abs_diff(b, cb))
            })
            .min_by_key(|(_, d)| *d)
    }
}

impl IntoIterator for Colors {
    type Item = Self;

    type IntoIter = crate::iter::ColorsIter;

    fn into_iter(self) -> Self::IntoIter {
        crate::iter::ColorsIter::starting_with(self)
    }
}

impl AsRef<str> for Colors {
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FromStr for Colors {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s).ok_or(())
    }
}

pub mod iter {
    use super::Colors;

    pub struct ColorsIter {
        current: Option<Colors>,
    }

    impl Default for ColorsIter {
        fn default() -> Self {
            Self {
                current: Some(Self::FIRST),
            }
        }
    }

    impl ColorsIter {
        const FIRST: Colors = Colors::AliceBlue;
        const LAST: Colors = Colors::YellowGreen;

        pub fn new() -> Self {
            Default::default()
        }

        pub fn starting_with(color: Colors) -> Self {
            Self {
                current: Some(color),
            }
        }
    }

    impl Iterator for ColorsIter {
        type Item = Colors;

        fn next(&mut self) -> Option<Self::Item> {
            let current = self.current.take();
            self.current = current.and_then(|c| {
                let next = c.next();
                if next == Self::FIRST {
                    None
                } else {
                    Some(next)
                }
            });

            current
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn default() {
            let mut iter: ColorsIter = Default::default();
            assert_eq!(iter.next(), Some(Colors::AliceBlue));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn into_iter() {
        let mut colors_iter = Colors::all();
        let first = colors_iter.next().unwrap();
        assert_eq!(first, Colors::AliceBlue);
        assert_eq!(first.name(), "AliceBlue");
        assert_eq!(first.rgb(), (240, 248, 255));
        assert_eq!(first.next(), Colors::AntiqueWhite);
    }

    #[test]
    fn dumb_test() {
        assert_eq!(Colors::AliceBlue.rgb(), (240, 248, 255));
        assert_eq!(Colors::AntiqueWhite.rgb(), (250, 235, 215));
        assert_eq!(Colors::Aqua.rgb(), (0, 255, 255));
        assert_eq!(Colors::AquaMarine.rgb(), (127, 255, 212));
        assert_eq!(Colors::Azure.rgb(), (240, 255, 255));
        assert_eq!(Colors::Beige.rgb(), (245, 245, 220));
        assert_eq!(Colors::Bisque.rgb(), (255, 228, 196));
        assert_eq!(Colors::Black.rgb(), (0, 0, 0));
        assert_eq!(Colors::BlanchedAlmond.rgb(), (255, 235, 205));
        assert_eq!(Colors::Blue.rgb(), (0, 0, 255));
        assert_eq!(Colors::BlueViolet.rgb(), (138, 43, 226));
        assert_eq!(Colors::Brown.rgb(), (165, 42, 42));
        assert_eq!(Colors::BurlyWood.rgb(), (222, 184, 135));
        assert_eq!(Colors::CadetBlue.rgb(), (95, 158, 160));
        assert_eq!(Colors::Chartreuse.rgb(), (127, 255, 0));
        assert_eq!(Colors::Chocolate.rgb(), (210, 105, 30));
        assert_eq!(Colors::Coral.rgb(), (255, 127, 80));
        assert_eq!(Colors::CornFlowerBlue.rgb(), (100, 149, 237));
        assert_eq!(Colors::CornSilk.rgb(), (255, 248, 220));
        assert_eq!(Colors::Crimson.rgb(), (220, 20, 60));
        assert_eq!(Colors::Cyan.rgb(), (0, 255, 255));
        assert_eq!(Colors::DarkBlue.rgb(), (0, 0, 139));
        assert_eq!(Colors::DarkCyan.rgb(), (0, 139, 139));
        assert_eq!(Colors::DarkGoldenRod.rgb(), (184, 134, 11));
        assert_eq!(Colors::DarkGray.rgb(), (169, 169, 169));
        assert_eq!(Colors::DarkGreen.rgb(), (0, 100, 0));
        assert_eq!(Colors::DarkGrey.rgb(), (169, 169, 169));
        assert_eq!(Colors::DarkKhaki.rgb(), (189, 183, 107));
        assert_eq!(Colors::DarkMagenta.rgb(), (139, 0, 139));
        assert_eq!(Colors::DarkOliveGreen.rgb(), (85, 107, 47));
        assert_eq!(Colors::DarkOrange.rgb(), (255, 140, 0));
        assert_eq!(Colors::DarkOrchid.rgb(), (153, 50, 204));
        assert_eq!(Colors::DarkRed.rgb(), (139, 0, 0));
        assert_eq!(Colors::DarkSalmon.rgb(), (233, 150, 122));
        assert_eq!(Colors::DarkSeaGreen.rgb(), (143, 188, 143));
        assert_eq!(Colors::DarkSlateBlue.rgb(), (72, 61, 139));
        assert_eq!(Colors::DarkSlateGray.rgb(), (47, 79, 79));
        assert_eq!(Colors::DarkTurquoise.rgb(), (0, 206, 209));
        assert_eq!(Colors::DarkViolet.rgb(), (148, 0, 211));
        assert_eq!(Colors::DeepPink.rgb(), (255, 20, 147));
        assert_eq!(Colors::DeepSkyBlue.rgb(), (0, 191, 255));
        assert_eq!(Colors::DimGray.rgb(), (105, 105, 105));
        assert_eq!(Colors::DimGrey.rgb(), (105, 105, 105));
        assert_eq!(Colors::DodgerBlue.rgb(), (30, 144, 255));
        assert_eq!(Colors::Firebrick.rgb(), (178, 34, 34));
        assert_eq!(Colors::FloralWhite.rgb(), (255, 250, 240));
        assert_eq!(Colors::ForestGreen.rgb(), (34, 139, 34));
        assert_eq!(Colors::Fuchsia.rgb(), (255, 0, 255));
        assert_eq!(Colors::Gainsboro.rgb(), (220, 220, 220));
        assert_eq!(Colors::GhostWhite.rgb(), (248, 248, 255));
        assert_eq!(Colors::Gold.rgb(), (255, 215, 0));
        assert_eq!(Colors::GoldenRod.rgb(), (218, 165, 32));
        assert_eq!(Colors::Gray.rgb(), (128, 128, 128));
        assert_eq!(Colors::Green.rgb(), (0, 128, 0));
        assert_eq!(Colors::GreenYellow.rgb(), (173, 255, 47));
        assert_eq!(Colors::Grey.rgb(), (128, 128, 128));
        assert_eq!(Colors::Honeydew.rgb(), (240, 255, 240));
        assert_eq!(Colors::HotPink.rgb(), (255, 105, 180));
        assert_eq!(Colors::IndianRed.rgb(), (205, 92, 92));
        assert_eq!(Colors::Indigo.rgb(), (75, 0, 130));
        assert_eq!(Colors::Ivory.rgb(), (255, 255, 240));
        assert_eq!(Colors::Khaki.rgb(), (240, 230, 140));
        assert_eq!(Colors::Lavender.rgb(), (230, 230, 250));
        assert_eq!(Colors::LavenderBlush.rgb(), (255, 240, 245));
        assert_eq!(Colors::LawnGreen.rgb(), (124, 252, 0));
        assert_eq!(Colors::LemonChiffon.rgb(), (255, 250, 205));
        assert_eq!(Colors::LightBlue.rgb(), (173, 216, 230));
        assert_eq!(Colors::LightCoral.rgb(), (240, 128, 128));
        assert_eq!(Colors::LightCyan.rgb(), (224, 255, 255));
        assert_eq!(Colors::LightGoldenRodYellow.rgb(), (250, 250, 210));
        assert_eq!(Colors::LightGray.rgb(), (211, 211, 211));
        assert_eq!(Colors::LightGreen.rgb(), (144, 238, 144));
        assert_eq!(Colors::LightGrey.rgb(), (211, 211, 211));
        assert_eq!(Colors::LightPink.rgb(), (255, 182, 193));
        assert_eq!(Colors::LightSalmon.rgb(), (255, 160, 122));
        assert_eq!(Colors::LightSeaGreen.rgb(), (32, 178, 170));
        assert_eq!(Colors::LightSkyBlue.rgb(), (135, 206, 250));
        assert_eq!(Colors::LightSlateGray.rgb(), (119, 136, 153));
        assert_eq!(Colors::LightSteelBlue.rgb(), (176, 196, 222));
        assert_eq!(Colors::LightYellow.rgb(), (255, 255, 224));
        assert_eq!(Colors::Lime.rgb(), (0, 255, 0));
        assert_eq!(Colors::LimeGreen.rgb(), (50, 205, 50));
        assert_eq!(Colors::Linen.rgb(), (250, 240, 230));
        assert_eq!(Colors::Magenta.rgb(), (255, 0, 255));
        assert_eq!(Colors::Maroon.rgb(), (128, 0, 0));
        assert_eq!(Colors::MediumAquaMarine.rgb(), (102, 205, 170));
        assert_eq!(Colors::MediumBlue.rgb(), (0, 0, 205));
        assert_eq!(Colors::MediumOrchid.rgb(), (186, 85, 211));
        assert_eq!(Colors::MediumPurple.rgb(), (147, 112, 219));
        assert_eq!(Colors::MediumSeaGreen.rgb(), (60, 179, 113));
        assert_eq!(Colors::MediumSlateBlue.rgb(), (123, 104, 238));
        assert_eq!(Colors::MediumSpringGreen.rgb(), (0, 250, 154));
        assert_eq!(Colors::MediumTurquoise.rgb(), (72, 209, 204));
        assert_eq!(Colors::MediumVioletRed.rgb(), (199, 21, 133));
        assert_eq!(Colors::MidnightBlue.rgb(), (25, 25, 112));
        assert_eq!(Colors::MintCream.rgb(), (245, 255, 250));
        assert_eq!(Colors::MistyRose.rgb(), (255, 228, 225));
        assert_eq!(Colors::Moccasin.rgb(), (255, 228, 181));
        assert_eq!(Colors::NavajoWhite.rgb(), (255, 222, 173));
        assert_eq!(Colors::Navy.rgb(), (0, 0, 128));
        assert_eq!(Colors::OldLace.rgb(), (253, 245, 230));
        assert_eq!(Colors::Olive.rgb(), (128, 128, 0));
        assert_eq!(Colors::OliveDrab.rgb(), (107, 142, 35));
        assert_eq!(Colors::Orange.rgb(), (255, 165, 0));
        assert_eq!(Colors::OrangeRed.rgb(), (255, 69, 0));
        assert_eq!(Colors::Orchid.rgb(), (218, 112, 214));
        assert_eq!(Colors::PaleGoldenRod.rgb(), (238, 232, 170));
        assert_eq!(Colors::PaleGreen.rgb(), (152, 251, 152));
        assert_eq!(Colors::PaleTurquoise.rgb(), (175, 238, 238));
        assert_eq!(Colors::PaleVioletRed.rgb(), (219, 112, 147));
        assert_eq!(Colors::PapayaWhip.rgb(), (255, 239, 213));
        assert_eq!(Colors::PeachPuff.rgb(), (255, 218, 185));
        assert_eq!(Colors::Peru.rgb(), (205, 133, 63));
        assert_eq!(Colors::Pink.rgb(), (255, 192, 203));
        assert_eq!(Colors::Plum.rgb(), (221, 160, 221));
        assert_eq!(Colors::PowderBlue.rgb(), (176, 224, 230));
        assert_eq!(Colors::Purple.rgb(), (128, 0, 128));
        assert_eq!(Colors::Red.rgb(), (255, 0, 0));
        assert_eq!(Colors::RosyBrown.rgb(), (188, 143, 143));
        assert_eq!(Colors::RoyalBlue.rgb(), (65, 105, 225));
        assert_eq!(Colors::SaddleBrown.rgb(), (139, 69, 19));
        assert_eq!(Colors::Salmon.rgb(), (250, 128, 114));
        assert_eq!(Colors::SandyBrown.rgb(), (244, 164, 96));
        assert_eq!(Colors::SeaGreen.rgb(), (46, 139, 87));
        assert_eq!(Colors::SeaShell.rgb(), (255, 245, 238));
        assert_eq!(Colors::Sienna.rgb(), (160, 82, 45));
        assert_eq!(Colors::Silver.rgb(), (192, 192, 192));
        assert_eq!(Colors::SkyBlue.rgb(), (135, 206, 235));
        assert_eq!(Colors::SlateBlue.rgb(), (106, 90, 205));
        assert_eq!(Colors::SlateGray.rgb(), (112, 128, 144));
        assert_eq!(Colors::Snow.rgb(), (255, 250, 250));
        assert_eq!(Colors::SpringGreen.rgb(), (0, 255, 127));
        assert_eq!(Colors::SteelBlue.rgb(), (70, 130, 180));
        assert_eq!(Colors::Tan.rgb(), (210, 180, 140));
        assert_eq!(Colors::Teal.rgb(), (0, 128, 128));
        assert_eq!(Colors::Thistle.rgb(), (216, 191, 216));
        assert_eq!(Colors::Tomato.rgb(), (255, 99, 71));
        assert_eq!(Colors::Turquoise.rgb(), (64, 224, 208));
        assert_eq!(Colors::Violet.rgb(), (238, 130, 238));
        assert_eq!(Colors::Wheat.rgb(), (245, 222, 179));
        assert_eq!(Colors::White.rgb(), (255, 255, 255));
        assert_eq!(Colors::WhiteSmoke.rgb(), (245, 245, 245));
        assert_eq!(Colors::Yellow.rgb(), (255, 255, 0));
        assert_eq!(Colors::YellowGreen.rgb(), (154, 205, 50));
    }

    #[test]
    fn dumb_test2() {
        assert_eq!(Colors::AliceBlue.name(), "AliceBlue");
        assert_eq!(Colors::AntiqueWhite.name(), "AntiqueWhite");
        assert_eq!(Colors::Aqua.name(), "Aqua");
        assert_eq!(Colors::AquaMarine.name(), "AquaMarine");
        assert_eq!(Colors::Azure.name(), "Azure");
        assert_eq!(Colors::Beige.name(), "Beige");
        assert_eq!(Colors::Bisque.name(), "Bisque");
        assert_eq!(Colors::Black.name(), "Black");
        assert_eq!(Colors::BlanchedAlmond.name(), "BlanchedAlmond");
        assert_eq!(Colors::Blue.name(), "Blue");
        assert_eq!(Colors::BlueViolet.name(), "BlueViolet");
        assert_eq!(Colors::Brown.name(), "Brown");
        assert_eq!(Colors::BurlyWood.name(), "BurlyWood");
        assert_eq!(Colors::CadetBlue.name(), "CadetBlue");
        assert_eq!(Colors::Chartreuse.name(), "Chartreuse");
        assert_eq!(Colors::Chocolate.name(), "Chocolate");
        assert_eq!(Colors::Coral.name(), "Coral");
        assert_eq!(Colors::CornFlowerBlue.name(), "CornFlowerBlue");
        assert_eq!(Colors::CornSilk.name(), "CornSilk");
        assert_eq!(Colors::Crimson.name(), "Crimson");
        assert_eq!(Colors::Cyan.name(), "Cyan");
        assert_eq!(Colors::DarkBlue.name(), "DarkBlue");
        assert_eq!(Colors::DarkCyan.name(), "DarkCyan");
        assert_eq!(Colors::DarkGoldenRod.name(), "DarkGoldenRod");
        assert_eq!(Colors::DarkGray.name(), "DarkGray");
        assert_eq!(Colors::DarkGreen.name(), "DarkGreen");
        assert_eq!(Colors::DarkGrey.name(), "DarkGrey");
        assert_eq!(Colors::DarkKhaki.name(), "DarkKhaki");
        assert_eq!(Colors::DarkMagenta.name(), "DarkMagenta");
        assert_eq!(Colors::DarkOliveGreen.name(), "DarkOliveGreen");
        assert_eq!(Colors::DarkOrange.name(), "DarkOrange");
        assert_eq!(Colors::DarkOrchid.name(), "DarkOrchid");
        assert_eq!(Colors::DarkRed.name(), "DarkRed");
        assert_eq!(Colors::DarkSalmon.name(), "DarkSalmon");
        assert_eq!(Colors::DarkSeaGreen.name(), "DarkSeaGreen");
        assert_eq!(Colors::DarkSlateBlue.name(), "DarkSlateBlue");
        assert_eq!(Colors::DarkSlateGray.name(), "DarkSlateGray");
        assert_eq!(Colors::DarkTurquoise.name(), "DarkTurquoise");
        assert_eq!(Colors::DarkViolet.name(), "DarkViolet");
        assert_eq!(Colors::DeepPink.name(), "DeepPink");
        assert_eq!(Colors::DeepSkyBlue.name(), "DeepSkyBlue");
        assert_eq!(Colors::DimGray.name(), "DimGray");
        assert_eq!(Colors::DimGrey.name(), "DimGrey");
        assert_eq!(Colors::DodgerBlue.name(), "DodgerBlue");
        assert_eq!(Colors::Firebrick.name(), "Firebrick");
        assert_eq!(Colors::FloralWhite.name(), "FloralWhite");
        assert_eq!(Colors::ForestGreen.name(), "ForestGreen");
        assert_eq!(Colors::Fuchsia.name(), "Fuchsia");
        assert_eq!(Colors::Gainsboro.name(), "Gainsboro");
        assert_eq!(Colors::GhostWhite.name(), "GhostWhite");
        assert_eq!(Colors::Gold.name(), "Gold");
        assert_eq!(Colors::GoldenRod.name(), "GoldenRod");
        assert_eq!(Colors::Gray.name(), "Gray");
        assert_eq!(Colors::Green.name(), "Green");
        assert_eq!(Colors::GreenYellow.name(), "GreenYellow");
        assert_eq!(Colors::Grey.name(), "Grey");
        assert_eq!(Colors::Honeydew.name(), "Honeydew");
        assert_eq!(Colors::HotPink.name(), "HotPink");
        assert_eq!(Colors::IndianRed.name(), "IndianRed");
        assert_eq!(Colors::Indigo.name(), "Indigo");
        assert_eq!(Colors::Ivory.name(), "Ivory");
        assert_eq!(Colors::Khaki.name(), "Khaki");
        assert_eq!(Colors::Lavender.name(), "Lavender");
        assert_eq!(Colors::LavenderBlush.name(), "LavenderBlush");
        assert_eq!(Colors::LawnGreen.name(), "LawnGreen");
        assert_eq!(Colors::LemonChiffon.name(), "LemonChiffon");
        assert_eq!(Colors::LightBlue.name(), "LightBlue");
        assert_eq!(Colors::LightCoral.name(), "LightCoral");
        assert_eq!(Colors::LightCyan.name(), "LightCyan");
        assert_eq!(Colors::LightGoldenRodYellow.name(), "LightGoldenRodYellow");
        assert_eq!(Colors::LightGray.name(), "LightGray");
        assert_eq!(Colors::LightGreen.name(), "LightGreen");
        assert_eq!(Colors::LightGrey.name(), "LightGrey");
        assert_eq!(Colors::LightPink.name(), "LightPink");
        assert_eq!(Colors::LightSalmon.name(), "LightSalmon");
        assert_eq!(Colors::LightSeaGreen.name(), "LightSeaGreen");
        assert_eq!(Colors::LightSkyBlue.name(), "LightSkyBlue");
        assert_eq!(Colors::LightSlateGray.name(), "LightSlateGray");
        assert_eq!(Colors::LightSteelBlue.name(), "LightSteelBlue");
        assert_eq!(Colors::LightYellow.name(), "LightYellow");
        assert_eq!(Colors::Lime.name(), "Lime");
        assert_eq!(Colors::LimeGreen.name(), "LimeGreen");
        assert_eq!(Colors::Linen.name(), "Linen");
        assert_eq!(Colors::Magenta.name(), "Magenta");
        assert_eq!(Colors::Maroon.name(), "Maroon");
        assert_eq!(Colors::MediumAquaMarine.name(), "MediumAquaMarine");
        assert_eq!(Colors::MediumBlue.name(), "MediumBlue");
        assert_eq!(Colors::MediumOrchid.name(), "MediumOrchid");
        assert_eq!(Colors::MediumPurple.name(), "MediumPurple");
        assert_eq!(Colors::MediumSeaGreen.name(), "MediumSeaGreen");
        assert_eq!(Colors::MediumSlateBlue.name(), "MediumSlateBlue");
        assert_eq!(Colors::MediumSpringGreen.name(), "MediumSpringGreen");
        assert_eq!(Colors::MediumTurquoise.name(), "MediumTurquoise");
        assert_eq!(Colors::MediumVioletRed.name(), "MediumVioletRed");
        assert_eq!(Colors::MidnightBlue.name(), "MidnightBlue");
        assert_eq!(Colors::MintCream.name(), "MintCream");
        assert_eq!(Colors::MistyRose.name(), "MistyRose");
        assert_eq!(Colors::Moccasin.name(), "Moccasin");
        assert_eq!(Colors::NavajoWhite.name(), "NavajoWhite");
        assert_eq!(Colors::Navy.name(), "Navy");
        assert_eq!(Colors::OldLace.name(), "OldLace");
        assert_eq!(Colors::Olive.name(), "Olive");
        assert_eq!(Colors::OliveDrab.name(), "OliveDrab");
        assert_eq!(Colors::Orange.name(), "Orange");
        assert_eq!(Colors::OrangeRed.name(), "OrangeRed");
        assert_eq!(Colors::Orchid.name(), "Orchid");
        assert_eq!(Colors::PaleGoldenRod.name(), "PaleGoldenRod");
        assert_eq!(Colors::PaleGreen.name(), "PaleGreen");
        assert_eq!(Colors::PaleTurquoise.name(), "PaleTurquoise");
        assert_eq!(Colors::PaleVioletRed.name(), "PaleVioletRed");
        assert_eq!(Colors::PapayaWhip.name(), "PapayaWhip");
        assert_eq!(Colors::PeachPuff.name(), "PeachPuff");
        assert_eq!(Colors::Peru.name(), "Peru");
        assert_eq!(Colors::Pink.name(), "Pink");
        assert_eq!(Colors::Plum.name(), "Plum");
        assert_eq!(Colors::PowderBlue.name(), "PowderBlue");
        assert_eq!(Colors::Purple.name(), "Purple");
        assert_eq!(Colors::Red.name(), "Red");
        assert_eq!(Colors::RosyBrown.name(), "RosyBrown");
        assert_eq!(Colors::RoyalBlue.name(), "RoyalBlue");
        assert_eq!(Colors::SaddleBrown.name(), "SaddleBrown");
        assert_eq!(Colors::Salmon.name(), "Salmon");
        assert_eq!(Colors::SandyBrown.name(), "SandyBrown");
        assert_eq!(Colors::SeaGreen.name(), "SeaGreen");
        assert_eq!(Colors::SeaShell.name(), "SeaShell");
        assert_eq!(Colors::Sienna.name(), "Sienna");
        assert_eq!(Colors::Silver.name(), "Silver");
        assert_eq!(Colors::SkyBlue.name(), "SkyBlue");
        assert_eq!(Colors::SlateBlue.name(), "SlateBlue");
        assert_eq!(Colors::SlateGray.name(), "SlateGray");
        assert_eq!(Colors::Snow.name(), "Snow");
        assert_eq!(Colors::SpringGreen.name(), "SpringGreen");
        assert_eq!(Colors::SteelBlue.name(), "SteelBlue");
        assert_eq!(Colors::Tan.name(), "Tan");
        assert_eq!(Colors::Teal.name(), "Teal");
        assert_eq!(Colors::Thistle.name(), "Thistle");
        assert_eq!(Colors::Tomato.name(), "Tomato");
        assert_eq!(Colors::Turquoise.name(), "Turquoise");
        assert_eq!(Colors::Violet.name(), "Violet");
        assert_eq!(Colors::Wheat.name(), "Wheat");
        assert_eq!(Colors::White.name(), "White");
        assert_eq!(Colors::WhiteSmoke.name(), "WhiteSmoke");
        assert_eq!(Colors::Yellow.name(), "Yellow");
        assert_eq!(Colors::YellowGreen.name(), "YellowGreen");
    }

    #[test]
    fn dumb_test3() {
        assert_eq!(Colors::AliceBlue.next(), Colors::AntiqueWhite);
        assert_eq!(Colors::AntiqueWhite.next(), Colors::Aqua);
        assert_eq!(Colors::Aqua.next(), Colors::AquaMarine);
        assert_eq!(Colors::AquaMarine.next(), Colors::Azure);
        assert_eq!(Colors::Azure.next(), Colors::Beige);
        assert_eq!(Colors::Beige.next(), Colors::Bisque);
        assert_eq!(Colors::Bisque.next(), Colors::Black);
        assert_eq!(Colors::Black.next(), Colors::BlanchedAlmond);
        assert_eq!(Colors::BlanchedAlmond.next(), Colors::Blue);
        assert_eq!(Colors::Blue.next(), Colors::BlueViolet);
        assert_eq!(Colors::BlueViolet.next(), Colors::Brown);
        assert_eq!(Colors::Brown.next(), Colors::BurlyWood);
        assert_eq!(Colors::BurlyWood.next(), Colors::CadetBlue);
        assert_eq!(Colors::CadetBlue.next(), Colors::Chartreuse);
        assert_eq!(Colors::Chartreuse.next(), Colors::Chocolate);
        assert_eq!(Colors::Chocolate.next(), Colors::Coral);
        assert_eq!(Colors::Coral.next(), Colors::CornFlowerBlue);
        assert_eq!(Colors::CornFlowerBlue.next(), Colors::CornSilk);
        assert_eq!(Colors::CornSilk.next(), Colors::Crimson);
        assert_eq!(Colors::Crimson.next(), Colors::Cyan);
        assert_eq!(Colors::Cyan.next(), Colors::DarkBlue);
        assert_eq!(Colors::DarkBlue.next(), Colors::DarkCyan);
        assert_eq!(Colors::DarkCyan.next(), Colors::DarkGoldenRod);
        assert_eq!(Colors::DarkGoldenRod.next(), Colors::DarkGray);
        assert_eq!(Colors::DarkGray.next(), Colors::DarkGreen);
        assert_eq!(Colors::DarkGreen.next(), Colors::DarkGrey);
        assert_eq!(Colors::DarkGrey.next(), Colors::DarkKhaki);
        assert_eq!(Colors::DarkKhaki.next(), Colors::DarkMagenta);
        assert_eq!(Colors::DarkMagenta.next(), Colors::DarkOliveGreen);
        assert_eq!(Colors::DarkOliveGreen.next(), Colors::DarkOrange);
        assert_eq!(Colors::DarkOrange.next(), Colors::DarkOrchid);
        assert_eq!(Colors::DarkOrchid.next(), Colors::DarkRed);
        assert_eq!(Colors::DarkRed.next(), Colors::DarkSalmon);
        assert_eq!(Colors::DarkSalmon.next(), Colors::DarkSeaGreen);
        assert_eq!(Colors::DarkSeaGreen.next(), Colors::DarkSlateBlue);
        assert_eq!(Colors::DarkSlateBlue.next(), Colors::DarkSlateGray);
        assert_eq!(Colors::DarkSlateGray.next(), Colors::DarkTurquoise);
        assert_eq!(Colors::DarkTurquoise.next(), Colors::DarkViolet);
        assert_eq!(Colors::DarkViolet.next(), Colors::DeepPink);
        assert_eq!(Colors::DeepPink.next(), Colors::DeepSkyBlue);
        assert_eq!(Colors::DeepSkyBlue.next(), Colors::DimGray);
        assert_eq!(Colors::DimGray.next(), Colors::DimGrey);
        assert_eq!(Colors::DimGrey.next(), Colors::DodgerBlue);
        assert_eq!(Colors::DodgerBlue.next(), Colors::Firebrick);
        assert_eq!(Colors::Firebrick.next(), Colors::FloralWhite);
        assert_eq!(Colors::FloralWhite.next(), Colors::ForestGreen);
        assert_eq!(Colors::ForestGreen.next(), Colors::Fuchsia);
        assert_eq!(Colors::Fuchsia.next(), Colors::Gainsboro);
        assert_eq!(Colors::Gainsboro.next(), Colors::GhostWhite);
        assert_eq!(Colors::GhostWhite.next(), Colors::Gold);
        assert_eq!(Colors::Gold.next(), Colors::GoldenRod);
        assert_eq!(Colors::GoldenRod.next(), Colors::Gray);
        assert_eq!(Colors::Gray.next(), Colors::Green);
        assert_eq!(Colors::Green.next(), Colors::GreenYellow);
        assert_eq!(Colors::GreenYellow.next(), Colors::Grey);
        assert_eq!(Colors::Grey.next(), Colors::Honeydew);
        assert_eq!(Colors::Honeydew.next(), Colors::HotPink);
        assert_eq!(Colors::HotPink.next(), Colors::IndianRed);
        assert_eq!(Colors::IndianRed.next(), Colors::Indigo);
        assert_eq!(Colors::Indigo.next(), Colors::Ivory);
        assert_eq!(Colors::Ivory.next(), Colors::Khaki);
        assert_eq!(Colors::Khaki.next(), Colors::Lavender);
        assert_eq!(Colors::Lavender.next(), Colors::LavenderBlush);
        assert_eq!(Colors::LavenderBlush.next(), Colors::LawnGreen);
        assert_eq!(Colors::LawnGreen.next(), Colors::LemonChiffon);
        assert_eq!(Colors::LemonChiffon.next(), Colors::LightBlue);
        assert_eq!(Colors::LightBlue.next(), Colors::LightCoral);
        assert_eq!(Colors::LightCoral.next(), Colors::LightCyan);
        assert_eq!(Colors::LightCyan.next(), Colors::LightGoldenRodYellow);
        assert_eq!(Colors::LightGoldenRodYellow.next(), Colors::LightGray);
        assert_eq!(Colors::LightGray.next(), Colors::LightGreen);
        assert_eq!(Colors::LightGreen.next(), Colors::LightGrey);
        assert_eq!(Colors::LightGrey.next(), Colors::LightPink);
        assert_eq!(Colors::LightPink.next(), Colors::LightSalmon);
        assert_eq!(Colors::LightSalmon.next(), Colors::LightSeaGreen);
        assert_eq!(Colors::LightSeaGreen.next(), Colors::LightSkyBlue);
        assert_eq!(Colors::LightSkyBlue.next(), Colors::LightSlateGray);
        assert_eq!(Colors::LightSlateGray.next(), Colors::LightSteelBlue);
        assert_eq!(Colors::LightSteelBlue.next(), Colors::LightYellow);
        assert_eq!(Colors::LightYellow.next(), Colors::Lime);
        assert_eq!(Colors::Lime.next(), Colors::LimeGreen);
        assert_eq!(Colors::LimeGreen.next(), Colors::Linen);
        assert_eq!(Colors::Linen.next(), Colors::Magenta);
        assert_eq!(Colors::Magenta.next(), Colors::Maroon);
        assert_eq!(Colors::Maroon.next(), Colors::MediumAquaMarine);
        assert_eq!(Colors::MediumAquaMarine.next(), Colors::MediumBlue);
        assert_eq!(Colors::MediumBlue.next(), Colors::MediumOrchid);
        assert_eq!(Colors::MediumOrchid.next(), Colors::MediumPurple);
        assert_eq!(Colors::MediumPurple.next(), Colors::MediumSeaGreen);
        assert_eq!(Colors::MediumSeaGreen.next(), Colors::MediumSlateBlue);
        assert_eq!(Colors::MediumSlateBlue.next(), Colors::MediumSpringGreen);
        assert_eq!(Colors::MediumSpringGreen.next(), Colors::MediumTurquoise);
        assert_eq!(Colors::MediumTurquoise.next(), Colors::MediumVioletRed);
        assert_eq!(Colors::MediumVioletRed.next(), Colors::MidnightBlue);
        assert_eq!(Colors::MidnightBlue.next(), Colors::MintCream);
        assert_eq!(Colors::MintCream.next(), Colors::MistyRose);
        assert_eq!(Colors::MistyRose.next(), Colors::Moccasin);
        assert_eq!(Colors::Moccasin.next(), Colors::NavajoWhite);
        assert_eq!(Colors::NavajoWhite.next(), Colors::Navy);
        assert_eq!(Colors::Navy.next(), Colors::OldLace);
        assert_eq!(Colors::OldLace.next(), Colors::Olive);
        assert_eq!(Colors::Olive.next(), Colors::OliveDrab);
        assert_eq!(Colors::OliveDrab.next(), Colors::Orange);
        assert_eq!(Colors::Orange.next(), Colors::OrangeRed);
        assert_eq!(Colors::OrangeRed.next(), Colors::Orchid);
        assert_eq!(Colors::Orchid.next(), Colors::PaleGoldenRod);
        assert_eq!(Colors::PaleGoldenRod.next(), Colors::PaleGreen);
        assert_eq!(Colors::PaleGreen.next(), Colors::PaleTurquoise);
        assert_eq!(Colors::PaleTurquoise.next(), Colors::PaleVioletRed);
        assert_eq!(Colors::PaleVioletRed.next(), Colors::PapayaWhip);
        assert_eq!(Colors::PapayaWhip.next(), Colors::PeachPuff);
        assert_eq!(Colors::PeachPuff.next(), Colors::Peru);
        assert_eq!(Colors::Peru.next(), Colors::Pink);
        assert_eq!(Colors::Pink.next(), Colors::Plum);
        assert_eq!(Colors::Plum.next(), Colors::PowderBlue);
        assert_eq!(Colors::PowderBlue.next(), Colors::Purple);
        assert_eq!(Colors::Purple.next(), Colors::Red);
        assert_eq!(Colors::Red.next(), Colors::RosyBrown);
        assert_eq!(Colors::RosyBrown.next(), Colors::RoyalBlue);
        assert_eq!(Colors::RoyalBlue.next(), Colors::SaddleBrown);
        assert_eq!(Colors::SaddleBrown.next(), Colors::Salmon);
        assert_eq!(Colors::Salmon.next(), Colors::SandyBrown);
        assert_eq!(Colors::SandyBrown.next(), Colors::SeaGreen);
        assert_eq!(Colors::SeaGreen.next(), Colors::SeaShell);
        assert_eq!(Colors::SeaShell.next(), Colors::Sienna);
        assert_eq!(Colors::Sienna.next(), Colors::Silver);
        assert_eq!(Colors::Silver.next(), Colors::SkyBlue);
        assert_eq!(Colors::SkyBlue.next(), Colors::SlateBlue);
        assert_eq!(Colors::SlateBlue.next(), Colors::SlateGray);
        assert_eq!(Colors::SlateGray.next(), Colors::Snow);
        assert_eq!(Colors::Snow.next(), Colors::SpringGreen);
        assert_eq!(Colors::SpringGreen.next(), Colors::SteelBlue);
        assert_eq!(Colors::SteelBlue.next(), Colors::Tan);
        assert_eq!(Colors::Tan.next(), Colors::Teal);
        assert_eq!(Colors::Teal.next(), Colors::Thistle);
        assert_eq!(Colors::Thistle.next(), Colors::Tomato);
        assert_eq!(Colors::Tomato.next(), Colors::Turquoise);
        assert_eq!(Colors::Turquoise.next(), Colors::Violet);
        assert_eq!(Colors::Violet.next(), Colors::Wheat);
        assert_eq!(Colors::Wheat.next(), Colors::White);
        assert_eq!(Colors::White.next(), Colors::WhiteSmoke);
        assert_eq!(Colors::WhiteSmoke.next(), Colors::Yellow);
        assert_eq!(Colors::Yellow.next(), Colors::YellowGreen);
        assert_eq!(Colors::YellowGreen.next(), Colors::AliceBlue);
    }

    #[test]
    fn closest() {
        assert!(matches!(
            Colors::get_closest_color((240, 248, 255)),
            Some((Colors::AliceBlue, _))
        ));
        assert!(matches!(
            Colors::get_closest_color((239, 247, 254)),
            Some((Colors::AliceBlue, _))
        ));
        assert!(matches!(
            Colors::get_closest_color((0, 0, 0)),
            Some((Colors::Black, _))
        ));
        assert!(matches!(
            Colors::get_closest_color((255, 255, 255)),
            Some((Colors::White, _))
        ));
        assert!(matches!(
            Colors::get_closest_color((200, 200, 200)),
            Some((Colors::Silver, _))
        ));
        assert!(matches!(
            Colors::get_closest_color((25, 125, 250)),
            Some((Colors::DodgerBlue, _))
        ));
    }

    #[test]
    fn from_name() {
        let names = Colors::all().map(|c| (c, c.name())).collect::<Vec<_>>();
        let names_upper = Colors::all()
            .map(|c| (c, c.name().to_uppercase()))
            .collect::<Vec<_>>();

        for (color, name) in names {
            assert_eq!(
                Colors::from_name(name),
                Some(color),
                "from_name failed on color {} with name {}",
                color,
                name
            );
            assert_eq!(
                Colors::from_str(name),
                Ok(color),
                "from_str failed on color {} with name {}",
                color,
                name
            );
            assert_eq!(
                color.to_string(),
                name,
                "to_string failed on color {} with name {}",
                color,
                name
            );
            assert_eq!(
                color.as_ref(),
                name,
                "as_ref failed on color {} with name {}",
                color,
                name
            );
        }

        for (color, name) in names_upper {
            assert_eq!(
                Colors::from_name_ignore_case(name.as_str()),
                Some(color),
                "from_name_ignore_case failed on color {} with name {}",
                color,
                name
            );
        }

        assert!(Colors::from_name("").is_none());
        assert!(Colors::from_str(" ").is_err());
        assert!(Colors::from_name_ignore_case("                 ").is_none());
    }

    #[test]
    fn sizeof() {
        assert_eq!(
            std::mem::size_of::<Colors>(),
            1,
            "sizeof Colors should be 1"
        );
    }
}
