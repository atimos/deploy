use pipeline::{Units, Unit, Commands, Command, Arguments, ExecutionMode};

pub fn run(unit: &Unit, args: &Option<Arguments>, units: &Units) {
    run_cmds(&unit.commands, units);
}

fn run_cmds(cmds: &Commands, units: &Units) {
    match cmds {
        Commands::Multiple { commands, mode, .. } => {
            match mode {
                ExecutionMode::Parallel => run_in_parallel(&commands, units),
                ExecutionMode::SequenceRunAll => run_all_in_sequence(&commands, units),
                ExecutionMode::SequenceStopOnError => run_sequence_to_error(&commands, units),
            }
        }
        Commands::Single(cmd) => run_cmd(&cmd, units)
    }
}

fn run_in_parallel(list: &[Commands], units: &Units) {
    run_all_in_sequence(list, units);
}
fn run_all_in_sequence(list: &[Commands], units: &Units) {
    for cmds in list {
        run_cmds(cmds, units);
    }
}
fn run_sequence_to_error(list: &[Commands], units: &Units) {
    for cmds in list {
        run_cmds(cmds, units);
    }
}

fn run_cmd(cmd: &Command, units: &Units) {
    match cmd {
        Command::Unit { id, args } => {
            println!("unit: {} - {:?}", id, units[id].description);
            run(&units[id], args, units);
        }
        Command::Wasm { uri, command, args } => {
            crate::wasm::run(uri, command, args);
        }
        Command::Oci { repository, image, command, raw_command, force_rebuild, args } => {
            crate::oci::run(repository, image, command, *raw_command, *force_rebuild, args);
        }
    }
}
