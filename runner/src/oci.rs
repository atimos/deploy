use crate::environment::Environment;
use pipeline::Arguments;

pub fn run(
    repo: &str,
    image: &str,
    cmd: &str,
    raw_cmd: bool,
    force_rebuild: bool,
    args: &Option<Arguments>,
    env: &mut Environment,
) {
    if raw_cmd || args.is_none() {
        println!(
            "docker run --rm --interactive --tty --volume $PWD:/app --user $(id -u):$(id -g) {}",
            cmd
        );
    } else {
        println!("docker run --rm --interactive --tty --volume $PWD:/app --user $(id -u):$(id -g) {} {:?}", cmd, args);
    }
}
