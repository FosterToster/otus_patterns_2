use std::fmt::Display;

#[derive(Default)]
pub struct Password(pub String);

impl Password {
    pub fn new() -> Self {
        Password(String::new())
    }
}

impl<T: AsRef<str>> From<T> for Password {
    fn from(val: T) -> Self {
        Self(val.as_ref().to_string())
    }
}

//Motivation
impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            write!(f, "<empty>")
        } else {
            write!(f, "*************")
        }
    }
}
