use url::Url;

use super::{Plugin, Paths, Environment, Command, Error, Result};

pub struct HttpsPlugin {
    pub uri: Url,
}

impl Plugin<'_> for HttpsPlugin {
    fn run(&mut self, cmd: &Command, env: Environment, paths: Paths) -> Result {
        Result::Ok(())
    }
}
