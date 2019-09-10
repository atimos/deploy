pub fn verbose(pipeline: &pipeline::Block) {
    println!("{:#?}", pipeline);
}

pub fn pretty(pipeline: &pipeline::Block, indentation: String) {
    let child_indentation = format!("    {}", indentation);

    match pipeline {
        pipeline::Block::List { description, list, mode, .. } => {
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
        pipeline::Block::On { description, condition, success, error, abort, .. } => {
            let child_indentation = format!("    {}", child_indentation);
            print!("{}On(", indentation);

            if let Some(description) = description {
                print!("\"{}\"", description);
            }

            println!("\n    {}condition:", indentation);
            pretty(condition, child_indentation.clone());

            if let Some(cmd) = success {
                println!("    {}on_success:", indentation);
                pretty(cmd, child_indentation.clone());
            }

            if let Some(cmd) = error {
                println!("    {}on_error:", indentation);
                pretty(cmd, child_indentation.clone());
            }

            if let Some(cmd) = abort {
                println!("    {}on_abort:", indentation);
                pretty(cmd, child_indentation);
            }

            println!("{})", indentation);
        }
        pipeline::Block::Program { description, location, commands, id, .. } => {
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
