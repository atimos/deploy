pub mod data;
mod error;
mod pipeline;
mod ron;

use std::convert::TryInto;

pub use error::*;
pub use pipeline::*;

pub fn from_ron(content: &[u8]) -> Result<Node, error::Error> {
    Ok(::ron::de::from_bytes::<ron::Pipeline>(content).map_err(Error::Syntax)?.try_into()?)
}
