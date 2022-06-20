// Heavily based on https://github.com/aevyrie/bevy_world_to_screenspace/blob/master/src/main.rs

use bevy::prelude::*;
use bevy::render::camera::*;
use bevy_inspector_egui::Inspectable;

use crate::core::states::*;
use crate::structs::*;
use crate::MainCamera;

#[derive(Inspectable, Component)]
pub struct Label {
    placed: bool,
    belongs_to: Entity,
    offset: Vec3,
}

impl Label {
    pub fn belongs_to(id: Entity) -> Label {
        Label {
            placed: false,
            belongs_to: id,
            offset: Vec3::ZERO,
        }
    }

    pub fn with_offset(mut self, offset: Vec3) -> Label {
        self.offset = offset;
        self
    }
}

#[derive(Component)]
pub struct LabelBase;

pub struct LabelPlacerPlugin;
impl Plugin for LabelPlacerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(label_placer.system());
    }
}

// TODO: Add changed<> detection
fn label_placer(
    windows: Res<Windows>,
    mut label_query: Query<
        (&mut Style, &CalculatedSize, &Label, &ComputedVisibility)
    >,
    lb_query: Query<(&Transform, &ComputedVisibility), (With<LabelBase>)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut ev_cameramoved: EventReader<CameraMoved>,
    images: Res<Assets<bevy::prelude::Image>>
) // Main 3d camera needs struct "MainCamera"
{
    // let window = windows.get_primary().unwrap();
    if ev_cameramoved.iter().next().is_some() {
        for (camera, camera_transform) in camera_query.iter() {
            for (mut style, calculated, label, cv) in label_query.iter_mut() {
                if !cv.is_visible {
                    continue
                }

                let (lb_position, cv) = lb_query.get(label.belongs_to).unwrap();
                if !cv.is_visible {
                    continue
                }
                match camera.world_to_screen(&windows, &images, camera_transform, lb_position.translation) {  
                    Some(coords) => {
                        style.position.left =
                            Val::Px(coords.x - calculated.size.width / 2.0 + label.offset.x);
                        style.position.bottom =
                            Val::Px(coords.y - calculated.size.height / 2.0 + label.offset.y);
                    }
                    None => {
                        style.position.bottom = Val::Px(-1000.0);
                    }
                }
            }
        }
    }
}
