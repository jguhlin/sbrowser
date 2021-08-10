use super::*;
use crate::structs::LoadLandmark;
use crate::*;

use bevy::prelude::*;

/*
pub struct Gff3Plugin;
impl Plugin for Gff3Plugin {
    fn build(&self, app: &mut AppBuilder) {

        app.add_system_set(
            SystemSet::on_enter(AppState::SequenceView)
                .with_system(setup.system())
                .with_system(parse_gff3.system()),
        )
        .add_system_set(SystemSet::on_exit(AppState::SequenceView).with_system(cleanup.system()));
    }
} */
