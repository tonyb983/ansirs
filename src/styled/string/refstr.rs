use crate::{Ansi, IntoAnsi};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PrettyStr<'original>(Option<Ansi>, &'original str);

impl<'original> PrettyStr<'original> {
    pub fn new(text: &'original str, format: impl IntoAnsi) -> Self {
        let style = format.into_ansi();
        let style = if style.is_default() {
            None
        } else {
            Some(style)
        };

        Self(style, text)
    }

    pub fn raw(&self) -> &str {
        self.1
    }

    pub fn style(&self) -> Option<&Ansi> {
        self.0.as_ref()
    }

    pub fn value(&self) -> String {
        self.0.unwrap_or_default().paint_text(self.1)
    }

    pub fn len(&self) -> usize {
        self.1.len()
    }

    pub fn modify_style<F: FnMut(Option<&Ansi>) -> Option<Ansi>>(&mut self, mut f: F) {
        let new = f(self.0.as_ref());
        self.0 = new;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn sizeof() {
        println!("Size of PrettyStr: {}", std::mem::size_of::<PrettyStr>());
    }
}
