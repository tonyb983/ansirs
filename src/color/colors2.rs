use crate::Color;

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
                            $(Self::$color => stringify!($color),)*
                        }
                    }

                    /// Get the RGB values of this color.
                    pub const fn rgb(&self) -> (u8, u8, u8) {
                        match self {
                            $(Self::$color => ($r,$g,$b),)*
                        }
                    }

                    pub fn from_name(input: &str) -> Option<Self> {
                        match input {
                            $(stringify!($color) => Some(Self::$color),)*
                            _ => None,
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

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

impl AsRef<str> for Colors {
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::str::FromStr for Colors {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_name(s).ok_or(())
    }
}
