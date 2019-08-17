use super::InstanceIds;
use super::Scripts;
use pipeline::{Pipeline, Unit, Units, Commands, Command};

type References<'a> = Vec<(InstanceIds<'a>, String)>;

pub fn prepare(pipeline: &Pipeline) -> Scripts {
    let mut scripts = Vec::new();

    for step in &pipeline.steps {
        prepare_unit(step, &pipeline.units, InstanceIds::new(), &mut scripts);
    }
    dbg!(scripts);
    Scripts::default()

}

pub fn prepare_unit<'a>(unit: &'a Unit, units: &'a Units, instance_ids: InstanceIds<'a>, scripts: &mut References<'a>) {
    prepare_cmds(&unit.commands, units, instance_ids, scripts);
}

fn prepare_cmds<'a>(cmds: &'a Commands, units: &'a Units, instance_ids: InstanceIds<'a>, scripts: &mut References<'a>) {
    match cmds {
        Commands::Multiple { commands, .. } => {
            for cmd in commands {
                prepare_cmds(cmd, units, instance_ids.clone(), scripts);
            }
        }
        Commands::Single(cmd) => prepare_cmd(&cmd, units, instance_ids, scripts),
    }
}

fn prepare_cmd<'a>(cmd: &'a Command, units: &'a Units, mut instance_ids: InstanceIds<'a>, scripts: &mut References<'a>) {
    match cmd {
        Command::Unit { id, instance_id, .. } => {
            instance_ids.push(instance_id);
            prepare_unit(&units[id], units, instance_ids, scripts);
        }
        Command::Wasm { uri, instance_id, .. } => {
            instance_ids.push(instance_id);
            scripts.push((instance_ids, uri.to_owned()));
        }
        _ => {}
    }
}
