use std::{fmt, error::Error};

#[derive(Debug)]
pub struct ComponentNotFoundError {
    pub name: String,
}

impl ComponentNotFoundError {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

impl fmt::Display for ComponentNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "component `{}` not found in TextBox", self.name)
    }
}

impl Error for ComponentNotFoundError {}

#[derive(Debug)]
pub enum TextBoxRenderError {
    ComponentTypeMismatch {
        name: String,
        expected: &'static str
    },
}

impl fmt::Display for TextBoxRenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextBoxRenderError::ComponentTypeMismatch {
                name,
                expected
            } => {
                write!(
                    f,
                    "component `{name}` expects {expected} but was not provided a value of that type"
                )
            }
        }
    }
}

impl Error for TextBoxRenderError {}