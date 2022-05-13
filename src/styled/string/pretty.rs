use crate::{Ansi, IntoAnsi};

/// A string and some styling.
///
/// Some stats:
/// - Sizeof usize or & = 8
/// - Sizeof Ansi = 9
/// - Sizeof String = 24
/// - Sizeof (String, usize) = 32
/// - Sizeof Stage 1 = 40
/// - Sizeof Stage 2 = 40
///
/// First stage was: ([`String`], [`Ansi`])
/// Sizeof = 40 bytes
/// Adjusted this so that if style is empty or default, we can save space by not storing it.
///
/// Second stage was ([`String`], [`Option<Ansi>`])
/// Sizeof = 40 bytes
/// This is taking up the same amount of space as Stage 1
///
/// Potential third stage is (String, usize)
/// Sizeof = 32 bytes
/// This would represent the text (including the formatting) and the length of the original text.
/// We can always retrieve the original text using the stored length, and the known offset of the ansi suffix ([`Ansi::SUFFIX`]).
/// This would require an Ansi function that can parse a string of ansi codes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrettyString(String, Option<Ansi>);

impl PrettyString {
    /// Create a [`PrettyString`] with no styling.
    #[must_use]
    pub fn plain(s: impl Into<String>) -> Self {
        Self(s.into(), None)
    }

    /// Takes a plain string `text` and a style `format`.
    #[must_use]
    pub fn new(text: impl AsRef<str>, format: impl IntoAnsi) -> Self {
        let text = text.as_ref().to_string();
        let style = format.into_ansi();
        Self(
            text,
            if style.is_default() {
                None
            } else {
                Some(style)
            },
        )
    }

    /// Get the "raw" (aka unstyled / original) text.
    #[must_use]
    pub fn raw(&self) -> &str {
        self.0.as_str()
    }

    /// Get the [`Ansi`] styling applied to this text.
    #[must_use]
    pub fn style(&self) -> Option<&Ansi> {
        self.1.as_ref()
    }

    /// Modify the styling applied to this text using the given closure.
    pub fn modify_style<F: FnMut(Option<&Ansi>) -> Option<Ansi>>(&mut self, mut f: F) {
        self.1 = f(self.1.as_ref());
    }

    /// Get the formatted value of this [`PrettyString`].
    #[must_use]
    pub fn value(&self) -> String {
        self.to_string()
    }

    /// Gets the length of the ***original text***, i.e. the VISIBLE length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks if the original / **visible** text is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Display for PrettyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            Some(ref style) => write!(f, "{}", style.paint_text(self.0.as_str())),
            None => write!(f, "{}", self.0),
        }
    }
}

impl From<PrettyString> for String {
    fn from(pretty: PrettyString) -> Self {
        pretty.0
    }
}

impl From<&PrettyString> for String {
    fn from(pretty: &PrettyString) -> Self {
        pretty.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ansi;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn basic_usage() {
        let style = Ansi::new().fg((25, 100, 250)).bold().underline();
        let text = "Hello, World!";
        let pretty = PrettyString::new(text, style);
        assert_eq!(pretty.raw(), text);
        assert_eq!(pretty.style(), Some(&style));
        assert_eq!(
            pretty.to_string(),
            format!("{}{}{}", style, text, Ansi::reset())
        );
    }

    #[test]
    fn plain() {
        assert_eq!(
            PrettyString::plain("Hello, World!").to_string(),
            "Hello, World!"
        );
        let style = Ansi::new().fg((25, 100, 250)).bold().underline();
        let text = "Hello, World!";
        let mut pretty = PrettyString::new(text, style);
        assert_ne!(pretty.to_string(), text);
        pretty.modify_style(|_| None);
        assert_eq!(pretty.to_string(), text);
        assert_eq!(PrettyString::plain(text), pretty);
        assert_eq!(PrettyString::new(text, Ansi::default()), pretty);
        assert_eq!(
            PrettyString::new(text, Ansi::default()),
            PrettyString::plain(text)
        );
    }

    #[test]
    fn modify_style() {
        let style1 = Ansi::new().fg((25, 100, 250)).bold().underline();
        let style2 = Ansi::new().bg((25, 100, 250)).italic().strike();
        let text = "Hello, World!";
        let mut pretty = PrettyString::new(text, style1);
        assert_eq!(pretty.raw(), text);
        assert_eq!(pretty.style(), Some(&style1));
        assert_eq!(
            pretty.to_string(),
            format!("{}{}{}", style1, text, Ansi::reset())
        );

        pretty.modify_style(|_| Some(style2));
        assert_eq!(pretty.raw(), text);
        assert_eq!(pretty.style(), Some(&style2));
        assert_eq!(
            pretty.to_string(),
            format!("{}{}{}", style2, text, Ansi::reset())
        );
    }

    #[test]
    fn conversion() {
        use std::borrow::Borrow;
        let style = Ansi::new().fg((25, 250, 75)).blink();
        let pretty = PrettyString::new("Hello", style);
        let string: String = pretty.borrow().into();
        assert_eq!(string, "Hello");
    }
}
