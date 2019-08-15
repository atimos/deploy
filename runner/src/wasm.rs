use crate::environment::Environment;
use pipeline::Arguments;

pub fn run(uri: &str, cmd: &str, args: &Option<Arguments>, env: &mut Environment) {
    println!("GET: {}#{}?{:?} ", uri, cmd, args);
}
