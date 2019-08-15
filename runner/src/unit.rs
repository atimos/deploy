use crate::environment::Environment;
use pipeline::{Arguments, Command, Commands, ExecutionMode, Unit, Units};

pub fn run(unit: &Unit, args: &Option<Arguments>, units: &Units, env: &mut Environment) {
    println!(" - {:?}", args);
    run_cmds(&unit.commands, units, env);
}

fn run_cmds(cmds: &Commands, units: &Units, env: &mut Environment) {
    match cmds {
        Commands::Multiple { commands, mode, .. } => match mode {
            ExecutionMode::Parallel => run_in_parallel(&commands, units, env),
            ExecutionMode::SequenceRunAll => run_all_in_sequence(&commands, units, env),
            ExecutionMode::SequenceStopOnError => run_sequence_to_error(&commands, units, env),
        },
        Commands::Single(cmd) => run_cmd(&cmd, units, env),
    }
}

fn run_in_parallel(list: &[Commands], units: &Units, env: &mut Environment) {
    run_all_in_sequence(list, units, env);
}
fn run_all_in_sequence(list: &[Commands], units: &Units, env: &mut Environment) {
    for cmds in list {
        run_cmds(cmds, units, env);
    }
}
fn run_sequence_to_error(list: &[Commands], units: &Units, env: &mut Environment) {
    for cmds in list {
        run_cmds(cmds, units, env);
    }
}

fn run_cmd(cmd: &Command, units: &Units, env: &mut Environment) {
    match cmd {
        Command::Unit { id, args } => {
            println!("unit: {} - {:?}", id, units[id].description);
            run(&units[id], args, units, env);
        }
        Command::Wasm { uri, command, args } => {
            crate::wasm::run(uri, command, args, env);
        }
        Command::Oci {
            repository,
            image,
            command,
            raw_command,
            force_rebuild,
            args,
        } => {
            crate::oci::run(
                repository,
                image,
                command,
                *raw_command,
                *force_rebuild,
                args,
                env,
            );
        }
    }
}
