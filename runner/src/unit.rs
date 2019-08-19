use super::InstanceIds;
use std::collections::HashMap;
use crate::environment::Environment;
use pipeline::{Command, Commands, ExecutionMode, Unit, Units, Expression};

pub fn run(unit: &Unit, args: &Option<HashMap<String, String>>, units: &Units, env: &mut Environment, instance_ids: InstanceIds) {
    run_cmds(&unit.commands, units, env, instance_ids);
}

fn run_cmds(cmds: &Commands, units: &Units, env: &mut Environment, instance_ids: InstanceIds) {
    match cmds {
        Commands::Multiple { commands, mode, .. } => match mode {
            ExecutionMode::Parallel => run_in_parallel(&commands, units, env, instance_ids),
            ExecutionMode::SequenceRunAll => run_all_in_sequence(&commands, units, env, instance_ids),
            ExecutionMode::SequenceStopOnError => run_sequence_to_error(&commands, units, env, instance_ids),
        },
        Commands::Expression(expr) => run_expr(&expr, units, env, instance_ids),
        Commands::Command(cmd) => run_cmd(&cmd, units, env, instance_ids),
    }
}

fn run_in_parallel(list: &[Commands], units: &Units, env: &mut Environment, instance_ids: InstanceIds) {
    run_all_in_sequence(list, units, env, instance_ids);
}
fn run_all_in_sequence(list: &[Commands], units: &Units, env: &mut Environment, instance_ids: InstanceIds) {
    for cmds in list {
        run_cmds(cmds, units, env, instance_ids.clone());
    }
}
fn run_sequence_to_error(list: &[Commands], units: &Units, env: &mut Environment, instance_ids: InstanceIds) {
    for cmds in list {
        run_cmds(cmds, units, env, instance_ids.clone());
    }
}

fn run_expr<'a>(expr: &'a Expression, units: &Units, env: &mut Environment, instance_ids: InstanceIds<'a>) {
    match expr {
        Expression::If { condition, success, error } => {
            println!("IF");
            run_cmds(condition, units, env, instance_ids.clone());
            if let Some(success) = success {
                run_cmds(success, units, env, instance_ids.clone());
            }
            if let Some(error) = error {
                run_cmds(error, units, env, instance_ids.clone());
            }
        }
    }
}

fn run_cmd<'a>(cmd: &'a Command, units: &Units, env: &mut Environment, mut instance_ids: InstanceIds<'a>) {
    match cmd {
        Command::Unit { id, args, instance_id } => {
            instance_ids.push(instance_id);
            println!("UNIT: {} - {:?}", id, units[id].description);
            run(&units[id], args, units, env, instance_ids);
        }
        Command::Wasm { uri, commands, instance_id } => {
            instance_ids.push(instance_id);
            crate::wasm::run(uri, commands, env, instance_ids);
        }
        Command::Oci { repository, image, commands, instance_id } => {
            instance_ids.push(instance_id);
            crate::oci::run(repository, image, commands, env, instance_ids);
        }
    }
}
