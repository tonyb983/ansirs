use crate::{Ansi, IntoAnsi};

/// This implementation of a styled / formatted string uses a reference to the original text so as to not
/// allocate a new [`String`] for each instance. This *does* mean that the `value` call must be calculated
/// each time it is called.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrettyStr<'original>(Option<Ansi>, &'original str);

impl<'original> PrettyStr<'original> {
    /// Takes a plain string `text` and a style `format`.
    pub fn new(text: &'original str, format: impl IntoAnsi) -> Self {
        let style = format.into_ansi();
        let style = if style.is_default() {
            None
        } else {
            Some(style)
        };

        Self(style, text)
    }

    /// Get the "raw" (aka unstyled / original) text.
    #[must_use]
    pub fn raw(&self) -> &str {
        self.1
    }

    /// Get the [`Ansi`] styling applied to this text.
    #[must_use]
    pub fn style(&self) -> Option<&Ansi> {
        self.0.as_ref()
    }

    /// Get the formatted text of this [`PrettyStr`].
    #[must_use]
    pub fn value(&self) -> String {
        self.0.unwrap_or_default().paint_text(self.1)
    }

    /// Gets the length of the ***original text***, i.e. the VISIBLE length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.1.len()
    }

    /// Checks if the original / **visible** text is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.1.is_empty()
    }

    /// Modify the styling applied to this text using the given closure.
    pub fn modify_style<F: FnMut(Option<&Ansi>) -> Option<Ansi>>(&mut self, mut f: F) {
        let new = f(self.0.as_ref());
        self.0 = new;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizeof() {
        println!(
            "Size of PrettyStr: {}",
            std::mem::size_of::<PrettyStr<'_>>()
        );
    }
}
