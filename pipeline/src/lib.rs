mod check;
mod error;
mod pipeline;
mod toml;

use std::convert::TryInto;
use self::toml::Pipeline as TomlPipeline;

pub use error::*;
pub use pipeline::*;

pub fn from_toml(content: &[u8]) -> Result<Pipeline> {
    let toml_pipeline = ::toml::from_slice::<TomlPipeline>(content)?;
    check::check(toml_pipeline.try_into()?)
}
