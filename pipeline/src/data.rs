use handlebars::Handlebars;
use serde::Deserialize;
use std::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

#[derive(Clone, Deserialize)]
#[serde(try_from = "String")]
pub struct Template(String);

impl TryFrom<String> for Template {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Handlebars::new().render_template(&value, &None::<Option<bool>>) {
            Ok(_) => Ok(Self(value)),
            Err(_) => Err("invalid template"),
        }
    }
}

impl Template {
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl Default for Template {
    fn default() -> Self {
        Self(String::new())
    }
}

impl Into<String> for Template {
    fn into(self) -> String {
        self.0
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl Debug for Template {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}
