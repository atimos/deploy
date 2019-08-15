use pipeline::Arguments;

pub fn run(repo: &str, image: &str, cmd: &str, raw_cmd: bool, force_rebuild: bool, args: &Option<Arguments>) {
    println!("oci: {}/{}: {}", repo, image, cmd);
}
