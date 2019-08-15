mod check;
mod error;
mod pipeline;
mod toml;

use self::toml::Pipeline as TomlPipeline;

pub use error::*;
pub use pipeline::*;

pub type Url = url::Url;

pub fn from_toml(content: &[u8]) -> Result<Pipeline> {
    check::check(TomlPipeline::into(::toml::from_slice::<TomlPipeline>(
        content,
    )?))
}
