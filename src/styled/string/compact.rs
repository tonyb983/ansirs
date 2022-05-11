use crate::{Ansi, IntoAnsi};

/// A "compact" version of pretty string that is meant to improve performance for `value` calls (which I figured
/// would be the most common use case for this type) by immediately calculating it and storing it. This means that
/// anytime `raw` or `style` are called they must be extracted / calculated / parsed from the formatted text,
/// making them slower than the standard [`PrettyString`] type. It also turns out to be the slowest of the three
/// iterations to construct (by quite a bit).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompactPrettyString(String, usize);

impl CompactPrettyString {
    /// Takes a plain string `text` and a style `format`.
    pub fn new(text: impl std::fmt::Display, format: impl IntoAnsi) -> Self {
        let text = text.to_string();
        let style = format.into_ansi();
        let len = text.len();
        let styled = style.paint_text(text.as_str());
        Self(styled, len)
    }

    /// Get the "raw" (aka unstyled / original) text.
    #[must_use]
    pub fn raw(&self) -> &str {
        let total_size = self.0.len();
        if self.0.is_empty() || total_size == self.1 {
            return self.0.as_str();
        }

        let total_size = self.0.len();
        let reset_size = Ansi::reset().len();
        let string_end = total_size - reset_size;
        let string_start = total_size - reset_size - self.1;
        &self.0[string_start..string_end]
    }

    /// Get the [`Ansi`] styling applied to this text.
    #[must_use]
    pub fn style(&self) -> Option<Ansi> {
        Ansi::parse_ansi_text(self.0.as_str())
    }

    /// Modify the styling applied to this text using the given closure.
    pub fn modify_style<F: FnOnce(Option<Ansi>) -> Option<Ansi>>(&mut self, f: F) {
        let current = self.style();
        let style = f(current);
        let text = self.raw();
        debug_assert!(text.len() == self.1);
        self.0 = match style {
            Some(style) => style.paint_text(text),
            None => text.to_string(),
        };
    }

    /// Get the formatted text of this [`CompactPrettyString`].
    #[must_use]
    pub fn value(&self) -> &str {
        self.0.as_str()
    }

    /// Gets the length of the ***original text***, i.e. the VISIBLE length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.1
    }

    /// Checks if the original / **visible** text is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.1 == 0
    }
}

impl From<CompactPrettyString> for String {
    fn from(pretty: CompactPrettyString) -> Self {
        pretty.raw().to_string()
    }
}

impl From<&CompactPrettyString> for String {
    fn from(pretty: &CompactPrettyString) -> Self {
        pretty.raw().to_string()
    }
}

impl std::fmt::Display for CompactPrettyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<String> for CompactPrettyString {
    fn eq(&self, other: &String) -> bool {
        self.raw() == other.as_str()
    }
}
impl PartialEq<&String> for CompactPrettyString {
    fn eq(&self, other: &&String) -> bool {
        self.raw() == other.as_str()
    }
}
impl PartialEq<String> for &CompactPrettyString {
    fn eq(&self, other: &String) -> bool {
        self.raw() == other.as_str()
    }
}
