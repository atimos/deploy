mod check;
mod error;
mod pipeline;
mod toml;

use self::toml::{parse as parse_toml};

pub use error::*;
pub use pipeline::*;

pub fn from_toml(content: &[u8]) -> Result<Pipeline> {
    check::check(parse_toml(content)?)
}
