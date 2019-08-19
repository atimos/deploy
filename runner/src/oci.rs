use crate::environment::Environment;
use pipeline::ExternalCommand;

#[derive(Debug)]
pub enum Container {
    Id(String),
    Image(String, String),
}

impl Container {
    pub fn run(cmds: &[ExternalCommand], env: &mut Environment) {
        println!("OCI: docker run --rm --interactive --tty --volume $PWD:/app --user $(id -u):$(id -g) {:?}", cmds);
    }
}
