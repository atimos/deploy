use pipeline::ExecutionMode;

pub fn pretty(pipeline: &pipeline::Node, indentation: String) {
    let child_indentation = format!("    {}", indentation);

    match pipeline {
        pipeline::Node::List { description, list, mode, .. } => {
            print!("{}{}(", indentation, print_mode(mode));
            if let Some(description) = description {
                print!("\"{}\" ", description);
            }
            println!("[");
            for cmd in list {
                pretty(cmd, child_indentation.clone());
            }
            println!("{}])", indentation);
        }
        pipeline::Node::On { description, condition, success, error, abort, .. } => {
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

fn print_mode(mode: &ExecutionMode) -> &'static str {
    match mode {
        ExecutionMode::SequenceStopOnError => "stop-on-error",
        ExecutionMode::SequenceRunAll => "run-all",
        ExecutionMode::Parallel => "parallel",
    }
}
