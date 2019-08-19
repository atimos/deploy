mod error;
mod pipeline;
mod ron;

pub use error::*;
pub use pipeline::*;

pub fn from_ron(content: &[u8]) -> Result<Pipeline, error::Error> {
    self::ron::parse(content)
}
