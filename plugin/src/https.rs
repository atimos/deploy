use pipeline::{Command, CommandId};
use tar::Builder;
use url::Url;

use super::{Environment, Paths, Plugin};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

#[derive(Debug)]
pub struct HttpsPlugin {
    pub uri: Url,
}

#[derive(Debug)]
struct RequestData {
    space: Vec<u8>,
}

#[derive(Debug)]
struct ResponseData {
    success: bool,
    output: Option<String>,
    errors: Option<Vec<String>>,
    space: Option<Vec<u8>>,
}

impl Plugin<'_> for HttpsPlugin {
    fn run(&mut self, cmd: &Command, env: Environment, paths: Paths) -> Result<(), Error> {
        let cmd_id = match &cmd.id {
            CommandId::Named(id) => id.to_owned(),
            CommandId::Uuid(id) => id.to_string(),
        };

        let mut archive = Builder::new(Vec::new());
        archive.follow_symlinks(false);

        if let Some(path) = paths.work {
            archive.append_dir_all("work", path)?;
        }

        if let Some(path) = paths.artifact {
            archive.append_dir_all(["artifact", cmd_id.as_ref()].join("/"), path)?;
        }

        if let Some(path) = paths.publish {
            archive.append_dir_all(["publish", cmd_id.as_ref()].join("/"), path)?;
        }

        let request_data = RequestData {
            space: archive.into_inner()?,
        };

        dbg!(request_data);

        Ok(())
    }
}
