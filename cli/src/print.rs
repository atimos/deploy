pub fn verbose(pipeline: &pipeline::Node) {
    dbg!(pipeline);
}

pub fn pretty(pipeline: &pipeline::Node, indentation: String) {
    let child_indentation = format!("    {}", indentation);

    match pipeline {
        pipeline::Node::List { description, list, mode, .. } => {
            print!("{}{:?}(", indentation, mode);
            if let Some(description) = description {
                print!("\"{}\" ", description);
            }
            println!("[");
            for cmd in list {
                pretty(cmd, child_indentation.clone());
            }
            println!("{}])", indentation);
        }
        pipeline::Node::Program { description, location, commands, id, .. } => {
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
                commands
                    .iter()
                    .map(|cmd| cmd.name.to_owned())
                    .collect::<Vec<String>>()
                    .join("\", \"")
            );

            println!(" id: \"{:?}\")", id);
        }
    }
}
