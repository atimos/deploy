use pipeline::{Command, ExecutionMode, InstanceId, Location, Node};
pub fn verbose(pipeline: &pipeline::Node) {
    dbg!(pipeline);
}

pub fn pretty(pipeline: &Node, indentation: String) {
    match pipeline {
        pipeline::Node::Nodes { description, nodes, mode, .. } => {
            print_node(description, nodes, mode, indentation)
        }
        pipeline::Node::Commands { description, location, commands, id, .. } => {
            print_commands(description, location, commands, id, indentation)
        }
    }
}

fn print_node(
    description: &Option<String>,
    nodes: &[Node],
    mode: &ExecutionMode,
    indentation: String,
) {
    print!("{}{:?}(", indentation, mode);
    if let Some(description) = description {
        print!("\"{}\" ", description);
    }
    println!("[");
    for node in nodes {
        pretty(node, format!("    {}", indentation));
    }
    println!("{}])", indentation);
}

fn print_commands(
    description: &Option<String>,
    location: &Location,
    cmds: &[Command],
    id: &InstanceId,
    indentation: String,
) {
    print!("{}", indentation);
    match location {
        pipeline::Location::Oci { repository, image } => {
            print!("Oci(");
            if let Some(description) = description {
                print!("\"{}\" ", description);
            }
            print!("image: \"{}/{}\"", repository, image);
        }
        pipeline::Location::Wasm { uri } => {
            print!("Wasm(");
            if let Some(description) = description {
                print!("\"{}\" ", description);
            }
            print!("uri: \"{}\"", uri);
        }
    }
    print!(
        " commands: [\"{}\"]",
        cmds.iter().map(|cmd| cmd.name.to_owned()).collect::<Vec<String>>().join("\", \"")
    );

    println!(" id: \"{:?}\")", id);
}
