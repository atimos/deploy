use pipeline::Command;

pub fn load(uri: &str) -> Result<Vec<u8>, ()> {
    Ok(Vec::new())
}
pub fn run(bin: &[u8], cmds: Option<Command>) -> Result<(), ()> {
    println!("WASM: {:?}", cmds);
    Ok(())
}
