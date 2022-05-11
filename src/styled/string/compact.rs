use crate::{Ansi, IntoAnsi};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactPrettyString(String, usize);

impl CompactPrettyString {
    pub fn new(text: impl std::fmt::Display, format: impl IntoAnsi) -> Self {
        let text = text.to_string();
        let style = format.into_ansi();
        let len = text.len();
        let styled = style.paint_text(text.as_str());
        Self(styled, len)
    }

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

    pub fn style(&self) -> Option<Ansi> {
        Ansi::parse_ansi_text(self.0.as_str())
    }

    pub fn modify_style<F: FnOnce(Option<Ansi>) -> Option<Ansi>>(&mut self, f: F) {
        let current = self.style();
        let style = f(current);
        let text = self.raw();
        assert_eq!(text.len(), self.1);
        self.0 = match style {
            Some(style) => style.paint_text(text),
            None => text.to_string(),
        };
    }

    pub fn value(&self) -> &str {
        self.0.as_str()
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
