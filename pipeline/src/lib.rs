mod check;
mod error;
mod pipeline;

pub use check::Pipeline;
pub use error::*;
pub use pipeline::*;

pub fn from_toml(content: &[u8]) -> Result<Pipeline> {
    check::check(toml::from_slice::<pipeline::Pipeline>(content)?)
}
