mod data;
mod error;
mod map;
mod pipeline;

use data::Pipeline as PipelineData;
use std::convert::TryFrom;

pub use error::*;
pub use pipeline::*;

pub type Url = url::Url;

pub fn from_toml(content: &[u8]) -> Result<Pipeline> {
    Ok(Pipeline::try_from(toml::from_slice::<PipelineData>(
        content,
    )?)?)
}
