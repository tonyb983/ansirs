// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// A set of flags to determine text style with ANSI color codes.
#[derive(Default, Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct AnsiFlags {
    bits: u8,
}

impl AnsiFlags {
    /// Bold text.
    pub const BOLD: Self = Self { bits: (1 << 0) };
    /// Underlined text.
    pub const UNDERLINE: Self = Self { bits: (1 << 1) };
    /// Italicized text.
    pub const ITALIC: Self = Self { bits: (1 << 2) };
    /// Blinking text.
    pub const BLINK: Self = Self { bits: (1 << 3) };
    /// Reversed / Inverted text.
    pub const REVERSE: Self = Self { bits: (1 << 4) };
    /// Striken text.
    pub const STRIKE: Self = Self { bits: (1 << 5) };

    /// Returns an empty set of flags.
    #[inline]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Returns the set containing all flags.
    #[inline]
    pub const fn all() -> Self {
        Self {
            bits: <Self as BitFlags>::BOLD
                | <Self as BitFlags>::UNDERLINE
                | <Self as BitFlags>::ITALIC
                | <Self as BitFlags>::BLINK
                | <Self as BitFlags>::REVERSE
                | <Self as BitFlags>::STRIKE,
        }
    }

    /// Returns the raw value of the flags currently stored.
    #[inline]
    pub const fn bits(&self) -> u8 {
        self.bits
    }

    /// Convert from underlying bit representation, unless that
    /// representation contains bits that do not correspond to a flag.
    #[inline]
    pub const fn from_bits(bits: u8) -> std::option::Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            std::option::Option::Some(Self { bits })
        } else {
            std::option::Option::None
        }
    }

    /// Convert from underlying bit representation, dropping any bits
    /// that do not correspond to flags.
    #[inline]
    pub const fn from_bits_truncate(bits: u8) -> Self {
        Self {
            bits: bits & Self::all().bits,
        }
    }

    /// Convert from underlying bit representation, preserving all
    /// bits (even those not corresponding to a defined flag).
    ///
    /// # Safety
    ///
    /// The caller of the `bitflags!` macro can chose to allow or
    /// disallow extra bits for their bitflags type.
    ///
    /// The caller of `from_bits_unchecked()` has to ensure that
    /// all bits correspond to a defined flag or that extra bits
    /// are valid for this bitflags type.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        Self { bits }
    }

    /// Returns `true` if no flags are currently stored.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }

    /// Returns `true` if all flags are currently set.
    #[inline]
    pub const fn is_all(&self) -> bool {
        Self::all().bits | self.bits == self.bits
    }

    /// Returns `true` if there are flags common to both `self` and `other`.
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !(Self {
            bits: self.bits & other.bits,
        })
        .is_empty()
    }

    /// Returns `true` if all of the flags in `other` are contained within `self`.
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    /// Returns the intersection between the flags in `self` and
    /// `other`.
    ///
    /// Specifically, the returned set contains only the flags which are
    /// present in *both* `self` *and* `other`.
    ///
    /// This is equivalent to using the `&` operator (e.g.
    /// [`ops::BitAnd`]), as in `flags & other`.
    ///
    /// [`ops::BitAnd`]: https://doc.rust-lang.org/std/ops/trait.BitAnd.html
    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }

    /// Returns the union of between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags which are
    /// present in *either* `self` *or* `other`, including any which are
    /// present in both (see [`Self::symmetric_difference`] if that
    /// is undesirable).
    ///
    /// This is equivalent to using the `|` operator (e.g.
    /// [`ops::BitOr`]), as in `flags | other`.
    ///
    /// [`ops::BitOr`]: https://doc.rust-lang.org/std/ops/trait.BitOr.html
    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    /// Returns the difference between the flags in `self` and `other`.
    ///
    /// Specifically, the returned set contains all flags present in
    /// `self`, except for the ones present in `other`.
    ///
    /// It is also conceptually equivalent to the \"bit-clear\" operation:
    /// `flags & !other` (and this syntax is also supported).
    ///
    /// This is equivalent to using the `-` operator (e.g.
    /// [`ops::Sub`]), as in `flags - other`.
    ///
    /// [`ops::Sub`]: https://doc.rust-lang.org/std/ops/trait.Sub.html
    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
    /// Returns the [symmetric difference][sym-diff] between the flags
    /// in `self` and `other`.
    ///
    /// Specifically, the returned set contains the flags present which
    /// are present in `self` or `other`, but that are not present in
    /// both. Equivalently, it contains the flags present in *exactly
    /// one* of the sets `self` and `other`.
    ///
    /// This is equivalent to using the `^` operator (e.g.
    /// [`ops::BitXor`]), as in `flags ^ other`.
    ///
    /// [sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    /// [`ops::BitXor`]: https://doc.rust-lang.org/std/ops/trait.BitXor.html
    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self {
            bits: self.bits ^ other.bits,
        }
    }

    /// Returns the complement of this set of flags.
    ///
    /// Specifically, the returned set contains all the flags which are
    /// not set in `self`, but which are allowed for this type.
    ///
    /// Alternatively, it can be thought of as the set difference
    /// between [`Self::all()`] and `self` (e.g. `Self::all() - self`)
    ///
    /// This is equivalent to using the `!` operator (e.g.
    /// [`ops::Not`]), as in `!flags`.
    ///
    /// [`Self::all()`]: Self::all
    /// [`ops::Not`]: https://doc.rust-lang.org/std/ops/trait.Not.html
    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits)
    }

    /// Inserts the specified flags in-place.
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.bits |= other.bits;
    }

    /// Inserts the specified flags and returns the result.
    #[inline]
    #[must_use]
    pub const fn insert_to(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    /// Removes the specified flags in-place.
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.bits &= !other.bits;
    }

    /// Removes the specified flags and returns the result.
    #[inline]
    #[must_use]
    pub const fn remove_to(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }

    /// Toggles the specified flags in-place.
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.bits ^= other.bits;
    }

    /// Toggles the specified flags and returns the result.
    #[inline]
    #[must_use]
    pub const fn toggle_to(self, other: Self) -> Self {
        Self {
            bits: self.bits ^ other.bits,
        }
    }

    /// Inserts or removes the specified flags depending on the passed value.
    #[inline]
    pub fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
}

impl std::ops::BitOr for AnsiFlags {
    type Output = Self;
    /// Returns the union of the two sets of flags.
    #[inline]
    fn bitor(self, other: AnsiFlags) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }
}
impl std::ops::BitOrAssign for AnsiFlags {
    /// Adds the set of flags.
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        self.bits |= other.bits;
    }
}
impl std::ops::BitXor for AnsiFlags {
    type Output = Self;
    /// Returns the left flags, but with all the right flags toggled.
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        Self {
            bits: self.bits ^ other.bits,
        }
    }
}
impl std::ops::BitXorAssign for AnsiFlags {
    /// Toggles the set of flags.
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        self.bits ^= other.bits;
    }
}
impl std::ops::BitAnd for AnsiFlags {
    type Output = Self;
    /// Returns the intersection between the two sets of flags.
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }
}
impl std::ops::BitAndAssign for AnsiFlags {
    /// Disables all flags disabled in the set.
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        self.bits &= other.bits;
    }
}
impl std::ops::Sub for AnsiFlags {
    type Output = Self;
    /// Returns the set difference of the two sets of flags.
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            bits: self.bits & !other.bits,
        }
    }
}
impl std::ops::SubAssign for AnsiFlags {
    /// Disables all flags enabled in the set.
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.bits &= !other.bits;
    }
}
impl std::ops::Not for AnsiFlags {
    type Output = Self;
    /// Returns the complement of this set of flags.
    #[inline]
    fn not(self) -> Self {
        Self { bits: !self.bits } & Self::all()
    }
}

impl std::iter::Extend<AnsiFlags> for AnsiFlags {
    fn extend<T: std::iter::IntoIterator<Item = Self>>(&mut self, iterator: T) {
        for item in iterator {
            self.insert(item)
        }
    }
}
impl std::iter::FromIterator<AnsiFlags> for AnsiFlags {
    fn from_iter<T: std::iter::IntoIterator<Item = Self>>(iterator: T) -> Self {
        let mut result = Self::empty();
        result.extend(iterator);
        result
    }
}

impl std::fmt::Debug for AnsiFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        if <Self as BoolFlags>::BOLD(self) {
            if !first {
                f.write_str(" | ")?;
            }
            first = false;
            f.write_str("BOLD")?;
        }
        if <Self as BoolFlags>::UNDERLINE(self) {
            if !first {
                f.write_str(" | ")?;
            }
            first = false;
            f.write_str("UNDERLINE")?;
        }
        if <Self as BoolFlags>::ITALIC(self) {
            if !first {
                f.write_str(" | ")?;
            }
            first = false;
            f.write_str("ITALIC")?;
        }
        if <Self as BoolFlags>::BLINK(self) {
            if !first {
                f.write_str(" | ")?;
            }
            first = false;
            f.write_str("BLINK")?;
        }
        if <Self as BoolFlags>::REVERSE(self) {
            if !first {
                f.write_str(" | ")?;
            }
            first = false;
            f.write_str("REVERSE")?;
        }
        if <Self as BoolFlags>::STRIKE(self) {
            if !first {
                f.write_str(" | ")?;
            }
            first = false;
            f.write_str("STRIKE")?;
        }
        let extra_bits = self.bits & !Self::all().bits();
        if extra_bits != 0 {
            if !first {
                f.write_str(" | ")?;
            }
            first = false;
            f.write_str("0x")?;
            std::fmt::LowerHex::fmt(&extra_bits, f)?;
        }
        if first {
            f.write_str("(empty)")?;
        }
        Ok(())
    }
}
impl std::fmt::Binary for AnsiFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.bits, f)
    }
}
impl std::fmt::Octal for AnsiFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.bits, f)
    }
}
impl std::fmt::LowerHex for AnsiFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.bits, f)
    }
}
impl std::fmt::UpperHex for AnsiFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.bits, f)
    }
}

trait BitFlags {
    const BOLD: u8 = 0;
    const UNDERLINE: u8 = 0;
    const ITALIC: u8 = 0;
    const BLINK: u8 = 0;
    const REVERSE: u8 = 0;
    const STRIKE: u8 = 0;
}
impl BitFlags for AnsiFlags {
    const BOLD: u8 = Self::BOLD.bits;
    const UNDERLINE: u8 = Self::UNDERLINE.bits;
    const ITALIC: u8 = Self::ITALIC.bits;
    const BLINK: u8 = Self::BLINK.bits;
    const REVERSE: u8 = Self::REVERSE.bits;
    const STRIKE: u8 = Self::STRIKE.bits;
}

#[allow(non_snake_case)]
trait BoolFlags {
    #[inline]
    fn BOLD(&self) -> bool {
        false
    }
    #[inline]
    fn UNDERLINE(&self) -> bool {
        false
    }
    #[inline]
    fn ITALIC(&self) -> bool {
        false
    }
    #[inline]
    fn BLINK(&self) -> bool {
        false
    }
    #[inline]
    fn REVERSE(&self) -> bool {
        false
    }
    #[inline]
    fn STRIKE(&self) -> bool {
        false
    }
}
#[allow(non_snake_case)]
impl BoolFlags for AnsiFlags {
    #[allow(deprecated)]
    #[inline]
    fn BOLD(&self) -> bool {
        if Self::BOLD.bits == 0 && self.bits != 0 {
            false
        } else {
            self.bits & Self::BOLD.bits == Self::BOLD.bits
        }
    }
    #[allow(deprecated)]
    #[inline]
    fn UNDERLINE(&self) -> bool {
        if Self::UNDERLINE.bits == 0 && self.bits != 0 {
            false
        } else {
            self.bits & Self::UNDERLINE.bits == Self::UNDERLINE.bits
        }
    }
    #[allow(deprecated)]
    #[inline]
    fn ITALIC(&self) -> bool {
        if Self::ITALIC.bits == 0 && self.bits != 0 {
            false
        } else {
            self.bits & Self::ITALIC.bits == Self::ITALIC.bits
        }
    }
    #[allow(deprecated)]
    #[inline]
    fn BLINK(&self) -> bool {
        if Self::BLINK.bits == 0 && self.bits != 0 {
            false
        } else {
            self.bits & Self::BLINK.bits == Self::BLINK.bits
        }
    }
    #[allow(deprecated)]
    #[inline]
    fn REVERSE(&self) -> bool {
        if Self::REVERSE.bits == 0 && self.bits != 0 {
            false
        } else {
            self.bits & Self::REVERSE.bits == Self::REVERSE.bits
        }
    }
    #[allow(deprecated)]
    #[inline]
    fn STRIKE(&self) -> bool {
        if Self::STRIKE.bits == 0 && self.bits != 0 {
            false
        } else {
            self.bits & Self::STRIKE.bits == Self::STRIKE.bits
        }
    }
}
