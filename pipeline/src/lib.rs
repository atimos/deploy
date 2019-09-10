mod error;
mod pipeline;
mod ron;

pub use error::*;
pub use pipeline::*;

pub fn from_ron(content: &[u8]) -> Result<Block, error::Error> {
    self::ron::parse(content)
}
