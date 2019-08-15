use pipeline::Arguments;
pub fn run(uri: &str, cmd: &str, args: &Option<Arguments>) {
    println!("wasm: {}: {}", uri, cmd);
}
