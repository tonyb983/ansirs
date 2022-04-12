// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::Color;
use std::fmt;

#[macro_export]

macro_rules! make_colors {
        ($(($color:ident,($r:expr,$g:expr,$b:expr))),*,) => {
                /// Enum containing known named colors.
                #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub enum Colors {
                        $($color,)*
                }

                impl Colors {

                    /// Get the name of this color.
                    pub const fn name(&self) -> &'static str {
                        match self {
                            $(Colors::$color => stringify!($color),)*
                        }
                    }

                    /// Get the RGB values of this color.
                    pub const fn rgb(&self) -> (u8, u8, u8) {
                        match self {
                            $(Colors::$color => ($r,$g,$b),)*
                        }
                    }

                }

        };
}

make_colors! {

    (AliceBlue, (240, 248, 255)),
    (AntiqueWhite,(250, 235, 215)),
    (Aqua , (0, 255, 255)),
    (AquaMarine , (127, 255, 212)),
    (Azure , (240, 255, 255)),
    (Beige , (245, 245, 220)),
    (Bisque , (255, 228, 196)),
    (Black , (0, 0, 0)),
    (BlanchedAlmond , (255, 235, 205)),
    (Blue , (0, 0, 255)),
    (BlueViolet , (138, 43, 226)),
    (Brown , (165, 42, 42)),
    (BurlyWood , (222, 184, 135)),
    (CadetBlue , (95, 158, 160)),
    (Chartreuse , (127, 255, 0)),
    (Chocolate , (210, 105, 30)),
    (Coral , (255, 127, 80)),
    (CornFlowerBlue , (100, 149, 237)),
    (CornSilk , (255, 248, 220)),
    (Crimson , (220, 20, 60)),
    (Cyan , (0, 255, 255)),
    (DarkBlue , (0, 0, 139)),
    (DarkCyan , (0, 139, 139)),
    (DarkGoldenRod , (184, 134, 11)),
    (DarkGray , (169, 169, 169)),
    (DarkGreen , (0, 100, 0)),
    (DarkGrey , (169, 169, 169)),
    (DarkKhaki , (189, 183, 107)),
    (DarkMagenta , (139, 0, 139)),
    (DarkOliveGreen , (85, 107, 47)),
    (DarkOrange , (255, 140, 0)),
    (DarkOrchid , (153, 50, 204)),
    (DarkRed , (139, 0, 0)),
    (DarkSalmon , (233, 150, 122)),
    (DarkSeaGreen , (143, 188, 143)),
    (DarkSlateBlue , (72, 61, 139)),
    (DarkSlateGray , (47, 79, 79)),
    (DarkTurquoise , (0, 206, 209)),
    (DarkViolet , (148, 0, 211)),
    (DeepPink , (255, 20, 147)),
    (DeepSkyBlue , (0, 191, 255)),
    (DimGray , (105, 105, 105)),
    (DimGrey , (105, 105, 105)),
    (DodgerBlue , (30, 144, 255)),
    (Firebrick , (178, 34, 34)),
    (FloralWhite , (255, 250, 240)),
    (ForestGreen , (34, 139, 34)),
    (Fuchsia , (255, 0, 255)),
    (Gainsboro , (220, 220, 220)),
    (GhostWhite , (248, 248, 255)),
    (Gold , (255, 215, 0)),
    (GoldenRod , (218, 165, 32)),
    (Gray , (128, 128, 128)),
    (Green , (0, 128, 0)),
    (GreenYellow , (173, 255, 47)),
    (Grey , (128, 128, 128)),
    (Honeydew , (240, 255, 240)),
    (HotPink , (255, 105, 180)),
    (IndianRed , (205, 92, 92)),
    (Indigo , (75, 0, 130)),
    (Ivory , (255, 255, 240)),
    (Khaki , (240, 230, 140)),
    (Lavender , (230, 230, 250)),
    (LavenderBlush , (255, 240, 245)),
    (LawnGreen , (124, 252, 0)),
    (LemonChiffon , (255, 250, 205)),
    (LightBlue , (173, 216, 230)),
    (LightCoral , (240, 128, 128)),
    (LightCyan , (224, 255, 255)),
    (LightGoldenRodYellow , (250, 250, 210)),
    (LightGray , (211, 211, 211)),
    (LightGreen , (144, 238, 144)),
    (LightGrey , (211, 211, 211)),
    (LightPink , (255, 182, 193)),
    (LightSalmon , (255, 160, 122)),
    (LightSeaGreen , (32, 178, 170)),
    (LightSkyBlue , (135, 206, 250)),
    (LightSlateGray , (119, 136, 153)),
    (LightSteelBlue , (176, 196, 222)),
    (LightYellow , (255, 255, 224)),
    (Lime , (0, 255, 0)),
    (LimeGreen , (50, 205, 50)),
    (Linen , (250, 240, 230)),
    (Magenta , (255, 0, 255)),
    (Maroon , (128, 0, 0)),
    (MediumAquaMarine , (102, 205, 170)),
    (MediumBlue , (0, 0, 205)),
    (MediumOrchid , (186, 85, 211)),
    (MediumPurple , (147, 112, 219)),
    (MediumSeaGreen , (60, 179, 113)),
    (MediumSlateBlue , (123, 104, 238)),
    (MediumSpringGreen , (0, 250, 154)),
    (MediumTurquoise , (72, 209, 204)),
    (MediumVioletRed , (199, 21, 133)),
    (MidnightBlue , (25, 25, 112)),
    (MintCream , (245, 255, 250)),
    (MistyRose , (255, 228, 225)),
    (Moccasin , (255, 228, 181)),
    (NavajoWhite , (255, 222, 173)),
    (Navy , (0, 0, 128)),
    (OldLace , (253, 245, 230)),
    (Olive , (128, 128, 0)),
    (OliveDrab , (107, 142, 35)),
    (Orange , (255, 165, 0)),
    (OrangeRed , (255, 69, 0)),
    (Orchid , (218, 112, 214)),
    (PaleGoldenRod , (238, 232, 170)),
    (PaleGreen , (152, 251, 152)),
    (PaleTurquoise , (175, 238, 238)),
    (PaleVioletRed , (219, 112, 147)),
    (PapayaWhip , (255, 239, 213)),
    (PeachPuff , (255, 218, 185)),
    (Peru , (205, 133, 63)),
    (Pink , (255, 192, 203)),
    (Plum , (221, 160, 221)),
    (PowderBlue , (176, 224, 230)),
    (Purple , (128, 0, 128)),
    (Red , (255, 0, 0)),
    (RosyBrown , (188, 143, 143)),
    (RoyalBlue , (65, 105, 225)),
    (SaddleBrown , (139, 69, 19)),
    (Salmon , (250, 128, 114)),
    (SandyBrown , (244, 164, 96)),
    (SeaGreen , (46, 139, 87)),
    (SeaShell , (255, 245, 238)),
    (Sienna , (160, 82, 45)),
    (Silver , (192, 192, 192)),
    (SkyBlue , (135, 206, 235)),
    (SlateBlue , (106, 90, 205)),
    (SlateGray , (112, 128, 144)),
    (Snow , (255, 250, 250)),
    (SpringGreen , (0, 255, 127)),
    (SteelBlue , (70, 130, 180)),
    (Tan , (210, 180, 140)),
    (Teal , (0, 128, 128)),
    (Thistle , (216, 191, 216)),
    (Tomato , (255, 99, 71)),
    (Turquoise , (64, 224, 208)),
    (Violet , (238, 130, 238)),
    (Wheat , (245, 222, 179)),
    (White , (255, 255, 255)),
    (WhiteSmoke , (245, 245, 245)),
    (Yellow , (255, 255, 0)),
    (YellowGreen , (154, 205, 50)),

}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Colors {
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
    pub fn get_closest_color(input: (u8, u8, u8)) -> Self {
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
        let (closest, amount) = Self::all()
            .map(|c| {
                let (cr, cg, cb) = c.rgb();
                let cr = cr as usize;
                let cg = cg as usize;
                let cb = cb as usize;
                (c, abs_diff(r, cr) + abs_diff(g, cg) + abs_diff(b, cb))
            })
            .min_by_key(|(_, d)| *d)
            .unwrap();

        println!(
            "Closest color to {:?} is {:?} with distance {:?}",
            input, closest, amount
        );
        closest
    }
}

impl IntoIterator for Colors {
    type Item = Self;

    type IntoIter = crate::iter::ColorsIter;

    fn into_iter(self) -> Self::IntoIter {
        crate::iter::ColorsIter::starting_with(self)
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
    fn dumb_test4() {
        assert_eq!(Colors::AliceBlue.to_string(), "AliceBlue");
        assert_eq!(Colors::AntiqueWhite.to_string(), "AntiqueWhite");
        assert_eq!(Colors::Aqua.to_string(), "Aqua");
        assert_eq!(Colors::AquaMarine.to_string(), "AquaMarine");
        assert_eq!(Colors::Azure.to_string(), "Azure");
        assert_eq!(Colors::Beige.to_string(), "Beige");
        assert_eq!(Colors::Bisque.to_string(), "Bisque");
        assert_eq!(Colors::Black.to_string(), "Black");
        assert_eq!(Colors::BlanchedAlmond.to_string(), "BlanchedAlmond");
        assert_eq!(Colors::Blue.to_string(), "Blue");
        assert_eq!(Colors::BlueViolet.to_string(), "BlueViolet");
        assert_eq!(Colors::Brown.to_string(), "Brown");
        assert_eq!(Colors::BurlyWood.to_string(), "BurlyWood");
        assert_eq!(Colors::CadetBlue.to_string(), "CadetBlue");
        assert_eq!(Colors::Chartreuse.to_string(), "Chartreuse");
        assert_eq!(Colors::Chocolate.to_string(), "Chocolate");
        assert_eq!(Colors::Coral.to_string(), "Coral");
        assert_eq!(Colors::CornFlowerBlue.to_string(), "CornFlowerBlue");
        assert_eq!(Colors::CornSilk.to_string(), "CornSilk");
        assert_eq!(Colors::Crimson.to_string(), "Crimson");
        assert_eq!(Colors::Cyan.to_string(), "Cyan");
        assert_eq!(Colors::DarkBlue.to_string(), "DarkBlue");
        assert_eq!(Colors::DarkCyan.to_string(), "DarkCyan");
        assert_eq!(Colors::DarkGoldenRod.to_string(), "DarkGoldenRod");
        assert_eq!(Colors::DarkGray.to_string(), "DarkGray");
        assert_eq!(Colors::DarkGreen.to_string(), "DarkGreen");
        assert_eq!(Colors::DarkGrey.to_string(), "DarkGrey");
        assert_eq!(Colors::DarkKhaki.to_string(), "DarkKhaki");
        assert_eq!(Colors::DarkMagenta.to_string(), "DarkMagenta");
        assert_eq!(Colors::DarkOliveGreen.to_string(), "DarkOliveGreen");
        assert_eq!(Colors::DarkOrange.to_string(), "DarkOrange");
        assert_eq!(Colors::DarkOrchid.to_string(), "DarkOrchid");
        assert_eq!(Colors::DarkRed.to_string(), "DarkRed");
        assert_eq!(Colors::DarkSalmon.to_string(), "DarkSalmon");
        assert_eq!(Colors::DarkSeaGreen.to_string(), "DarkSeaGreen");
        assert_eq!(Colors::DarkSlateBlue.to_string(), "DarkSlateBlue");
        assert_eq!(Colors::DarkSlateGray.to_string(), "DarkSlateGray");
        assert_eq!(Colors::DarkTurquoise.to_string(), "DarkTurquoise");
        assert_eq!(Colors::DarkViolet.to_string(), "DarkViolet");
        assert_eq!(Colors::DeepPink.to_string(), "DeepPink");
        assert_eq!(Colors::DeepSkyBlue.to_string(), "DeepSkyBlue");
        assert_eq!(Colors::DimGray.to_string(), "DimGray");
        assert_eq!(Colors::DimGrey.to_string(), "DimGrey");
        assert_eq!(Colors::DodgerBlue.to_string(), "DodgerBlue");
        assert_eq!(Colors::Firebrick.to_string(), "Firebrick");
        assert_eq!(Colors::FloralWhite.to_string(), "FloralWhite");
        assert_eq!(Colors::ForestGreen.to_string(), "ForestGreen");
        assert_eq!(Colors::Fuchsia.to_string(), "Fuchsia");
        assert_eq!(Colors::Gainsboro.to_string(), "Gainsboro");
        assert_eq!(Colors::GhostWhite.to_string(), "GhostWhite");
        assert_eq!(Colors::Gold.to_string(), "Gold");
        assert_eq!(Colors::GoldenRod.to_string(), "GoldenRod");
        assert_eq!(Colors::Gray.to_string(), "Gray");
        assert_eq!(Colors::Green.to_string(), "Green");
        assert_eq!(Colors::GreenYellow.to_string(), "GreenYellow");
        assert_eq!(Colors::Grey.to_string(), "Grey");
        assert_eq!(Colors::Honeydew.to_string(), "Honeydew");
        assert_eq!(Colors::HotPink.to_string(), "HotPink");
        assert_eq!(Colors::IndianRed.to_string(), "IndianRed");
        assert_eq!(Colors::Indigo.to_string(), "Indigo");
        assert_eq!(Colors::Ivory.to_string(), "Ivory");
        assert_eq!(Colors::Khaki.to_string(), "Khaki");
        assert_eq!(Colors::Lavender.to_string(), "Lavender");
        assert_eq!(Colors::LavenderBlush.to_string(), "LavenderBlush");
        assert_eq!(Colors::LawnGreen.to_string(), "LawnGreen");
        assert_eq!(Colors::LemonChiffon.to_string(), "LemonChiffon");
        assert_eq!(Colors::LightBlue.to_string(), "LightBlue");
        assert_eq!(Colors::LightCoral.to_string(), "LightCoral");
        assert_eq!(Colors::LightCyan.to_string(), "LightCyan");
        assert_eq!(
            Colors::LightGoldenRodYellow.to_string(),
            "LightGoldenRodYellow"
        );
        assert_eq!(Colors::LightGray.to_string(), "LightGray");
        assert_eq!(Colors::LightGreen.to_string(), "LightGreen");
        assert_eq!(Colors::LightGrey.to_string(), "LightGrey");
        assert_eq!(Colors::LightPink.to_string(), "LightPink");
        assert_eq!(Colors::LightSalmon.to_string(), "LightSalmon");
        assert_eq!(Colors::LightSeaGreen.to_string(), "LightSeaGreen");
        assert_eq!(Colors::LightSkyBlue.to_string(), "LightSkyBlue");
        assert_eq!(Colors::LightSlateGray.to_string(), "LightSlateGray");
        assert_eq!(Colors::LightSteelBlue.to_string(), "LightSteelBlue");
        assert_eq!(Colors::LightYellow.to_string(), "LightYellow");
        assert_eq!(Colors::Lime.to_string(), "Lime");
        assert_eq!(Colors::LimeGreen.to_string(), "LimeGreen");
        assert_eq!(Colors::Linen.to_string(), "Linen");
        assert_eq!(Colors::Magenta.to_string(), "Magenta");
        assert_eq!(Colors::Maroon.to_string(), "Maroon");
        assert_eq!(Colors::MediumAquaMarine.to_string(), "MediumAquaMarine");
        assert_eq!(Colors::MediumBlue.to_string(), "MediumBlue");
        assert_eq!(Colors::MediumOrchid.to_string(), "MediumOrchid");
        assert_eq!(Colors::MediumPurple.to_string(), "MediumPurple");
        assert_eq!(Colors::MediumSeaGreen.to_string(), "MediumSeaGreen");
        assert_eq!(Colors::MediumSlateBlue.to_string(), "MediumSlateBlue");
        assert_eq!(Colors::MediumSpringGreen.to_string(), "MediumSpringGreen");
        assert_eq!(Colors::MediumTurquoise.to_string(), "MediumTurquoise");
        assert_eq!(Colors::MediumVioletRed.to_string(), "MediumVioletRed");
        assert_eq!(Colors::MidnightBlue.to_string(), "MidnightBlue");
        assert_eq!(Colors::MintCream.to_string(), "MintCream");
        assert_eq!(Colors::MistyRose.to_string(), "MistyRose");
        assert_eq!(Colors::Moccasin.to_string(), "Moccasin");
        assert_eq!(Colors::NavajoWhite.to_string(), "NavajoWhite");
        assert_eq!(Colors::Navy.to_string(), "Navy");
        assert_eq!(Colors::OldLace.to_string(), "OldLace");
        assert_eq!(Colors::Olive.to_string(), "Olive");
        assert_eq!(Colors::OliveDrab.to_string(), "OliveDrab");
        assert_eq!(Colors::Orange.to_string(), "Orange");
        assert_eq!(Colors::OrangeRed.to_string(), "OrangeRed");
        assert_eq!(Colors::Orchid.to_string(), "Orchid");
        assert_eq!(Colors::PaleGoldenRod.to_string(), "PaleGoldenRod");
        assert_eq!(Colors::PaleGreen.to_string(), "PaleGreen");
        assert_eq!(Colors::PaleTurquoise.to_string(), "PaleTurquoise");
        assert_eq!(Colors::PaleVioletRed.to_string(), "PaleVioletRed");
        assert_eq!(Colors::PapayaWhip.to_string(), "PapayaWhip");
        assert_eq!(Colors::PeachPuff.to_string(), "PeachPuff");
        assert_eq!(Colors::Peru.to_string(), "Peru");
        assert_eq!(Colors::Pink.to_string(), "Pink");
        assert_eq!(Colors::Plum.to_string(), "Plum");
        assert_eq!(Colors::PowderBlue.to_string(), "PowderBlue");
        assert_eq!(Colors::Purple.to_string(), "Purple");
        assert_eq!(Colors::Red.to_string(), "Red");
        assert_eq!(Colors::RosyBrown.to_string(), "RosyBrown");
        assert_eq!(Colors::RoyalBlue.to_string(), "RoyalBlue");
        assert_eq!(Colors::SaddleBrown.to_string(), "SaddleBrown");
        assert_eq!(Colors::Salmon.to_string(), "Salmon");
        assert_eq!(Colors::SandyBrown.to_string(), "SandyBrown");
        assert_eq!(Colors::SeaGreen.to_string(), "SeaGreen");
        assert_eq!(Colors::SeaShell.to_string(), "SeaShell");
        assert_eq!(Colors::Sienna.to_string(), "Sienna");
        assert_eq!(Colors::Silver.to_string(), "Silver");
        assert_eq!(Colors::SkyBlue.to_string(), "SkyBlue");
        assert_eq!(Colors::SlateBlue.to_string(), "SlateBlue");
        assert_eq!(Colors::SlateGray.to_string(), "SlateGray");
        assert_eq!(Colors::Snow.to_string(), "Snow");
        assert_eq!(Colors::SpringGreen.to_string(), "SpringGreen");
        assert_eq!(Colors::SteelBlue.to_string(), "SteelBlue");
        assert_eq!(Colors::Tan.to_string(), "Tan");
        assert_eq!(Colors::Teal.to_string(), "Teal");
        assert_eq!(Colors::Thistle.to_string(), "Thistle");
        assert_eq!(Colors::Tomato.to_string(), "Tomato");
        assert_eq!(Colors::Turquoise.to_string(), "Turquoise");
        assert_eq!(Colors::Violet.to_string(), "Violet");
        assert_eq!(Colors::Wheat.to_string(), "Wheat");
        assert_eq!(Colors::White.to_string(), "White");
        assert_eq!(Colors::WhiteSmoke.to_string(), "WhiteSmoke");
        assert_eq!(Colors::Yellow.to_string(), "Yellow");
        assert_eq!(Colors::YellowGreen.to_string(), "YellowGreen");
    }

    #[test]
    fn closest() {
        assert_eq!(
            Colors::get_closest_color((240, 248, 255)),
            Colors::AliceBlue
        );
        assert_eq!(
            Colors::get_closest_color((240, 248, 254)),
            Colors::AliceBlue
        );
        assert_eq!(
            Colors::get_closest_color((240, 247, 254)),
            Colors::AliceBlue
        );
        assert_eq!(
            Colors::get_closest_color((239, 247, 254)),
            Colors::AliceBlue
        );
        assert_eq!(Colors::get_closest_color((0, 0, 0)), Colors::Black);
        assert_eq!(Colors::get_closest_color((255, 255, 255)), Colors::White);
        assert_eq!(Colors::get_closest_color((200, 200, 200)), Colors::Silver);
        assert_eq!(
            Colors::get_closest_color((25, 125, 250)),
            Colors::DodgerBlue
        );
    }
}
