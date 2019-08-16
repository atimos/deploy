use super::InstanceIds;
use super::Containers;
use pipeline::{Pipeline, Unit, Units, Commands, Command};

type Images<'a> = Vec<(InstanceIds<'a>, (String, String))>;

pub fn prepare(pipeline: &Pipeline) -> Containers {
    let mut images = Vec::new();

    for step in &pipeline.steps {
        prepare_unit(step, &pipeline.units, InstanceIds::new(), &mut images);
    }
    dbg!(images);

    Containers::default()
}

pub fn prepare_unit<'a>(unit: &'a Unit, units: &'a Units, instance_ids: InstanceIds<'a>, images: &mut Images<'a>) {
    prepare_cmds(&unit.commands, units, instance_ids, images);
}

fn prepare_cmds<'a>(cmds: &'a Commands, units: &'a Units, instance_ids: InstanceIds<'a>, images: &mut Images<'a>) {
    match cmds {
        Commands::Multiple { commands, .. } => {
            for cmd in commands {
                prepare_cmds(cmd, units, instance_ids.clone(), images);
            }
        }
        Commands::Single(cmd) => prepare_cmd(&cmd, units, instance_ids, images),
    }
}

fn prepare_cmd<'a>(cmd: &'a Command, units: &'a Units, mut instance_ids: InstanceIds<'a>, images: &mut Images<'a>) {
    match cmd {
        Command::Unit { id, instance_id, .. } => {
            instance_ids.push(instance_id);
            prepare_unit(&units[id], units, instance_ids, images);
        }
        Command::Oci { repository, image, instance_id, .. } => {
            instance_ids.push(instance_id);
            images.push((instance_ids, (repository.to_owned(), image.to_owned())));
        }
        _ => {}
    }
}
