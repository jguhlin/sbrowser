// Heavily based on https://github.com/aevyrie/bevy_world_to_screenspace/blob/master/src/main.rs

use bevy::prelude::*;
use bevy::render::camera::*;
use bevy_inspector_egui::Inspectable;

use crate::core::states::*;
use crate::MainCamera;

#[derive(Inspectable)]
pub struct Label {
    placed: bool,
    belongs_to: Entity,
    offset: Vec3,
}

impl Label {
    pub fn belongs_to(id: Entity) -> Label {
        Label { placed: false, belongs_to: id, offset: Vec3::ZERO } 
    }

    pub fn with_offset(mut self, offset: Vec3) -> Label {
        self.offset = offset;
        self
    }
}

pub struct LabelBase;

pub struct LabelPlacerPlugin;
impl Plugin for LabelPlacerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(label_placer.system());
    }
}

// TODO: Add changed<> detection
fn label_placer(
    windows: Res<Windows>,
    mut label_query: Query<(&mut Style, &CalculatedSize, &Label)>,
    lb_query: Query<&Transform, With<LabelBase>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,) // Main 3d camera needs struct "Camera"
{
    // let window = windows.get_primary().unwrap();
    for (camera, camera_transform) in camera_query.iter() {
        for (mut style, calculated, label) in label_query.iter_mut() {
            let lb_position = lb_query.get(label.belongs_to).unwrap();
            match camera.world_to_screen(&windows, camera_transform, lb_position.translation)
            {
                Some(coords) => {
                    style.position.left = Val::Px(coords.x - calculated.size.width / 2.0 + label.offset.x);
                    style.position.bottom = Val::Px(coords.y - calculated.size.height / 2.0 + label.offset.y);
                }
                None => {
                    style.position.bottom = Val::Px(-1000.0);
                }
            }
        }
    }
}