mod check;
mod data;
mod error;
mod map;
mod pipeline;

use data::Pipeline as PipelineData;

pub use error::*;
pub use pipeline::*;

pub type Url = url::Url;

pub fn from_toml(content: &[u8]) -> Result<Pipeline> {
    let pipeline = check::check(toml::from_slice::<PipelineData>(content)?)?;
    Ok(Pipeline::from(pipeline))
}
