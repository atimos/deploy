use pipeline::Command;

pub fn load(repository: &str, image: &str) -> Result<String, ()> {
    Ok(String::new())
}

pub fn run(container_id: &str, cmds: Option<Command>) -> Result<(), ()> {
    println!("OCI: docker run --rm --interactive --tty --volume $PWD:/app --user $(id -u):$(id -g) {:?}", cmds);
    Ok(())
}
