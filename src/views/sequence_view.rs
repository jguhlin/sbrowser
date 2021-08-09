use bevy::prelude::shape::CapsuleUvProfile;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_mod_picking::*;

use crate::core::states::*;
use crate::utils::label_placer::*;

pub struct SequenceViewItem;

pub struct SequenceViewPlugin;
impl Plugin for SequenceViewPlugin {
    fn build(&self, app: &mut AppBuilder) {
        //        app.add_system_set(
        //            SystemSet::on_update(AppState::SequenceView)
        //                .with_system(draw_primary.system()),
        //        )

        app.add_system_set(
            SystemSet::on_enter(AppState::SequenceView)
                .with_system(setup.system())
                .with_system(draw_primary.system()),
        )
        .add_system_set(SystemSet::on_exit(AppState::SequenceView).with_system(cleanup.system()));
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let id = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(0.2, 10.),
                flip: false,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::BISQUE,
                // emissive: Color::WHITE * 10.0f32,
                ..Default::default()
            }),
            transform: Transform {
                rotation: Quat::from_rotation_ypr(0., 0., std::f32::consts::FRAC_PI_2), // 1.5708),
                translation: Vec3::new(0., 5., 0.),
                scale: Vec3::new(1., 1., 1.),
            },

            //        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            //        material: materials.add(StandardMaterial {
            //            base_color: Color::hex("ffd891").unwrap(),
            // vary key PBR parameters on a grid of spheres to show the effect
            //            ..Default::default()
            //        }),
            //        transform: Transform::from_xyz(-5.0, -2.5, 0.0),
            ..Default::default()
        })
        .insert_bundle(PickableBundle::default())
        .insert(BoundVol::default())
        .insert(LabelBase)
        .insert(SequenceViewItem)
        .id();
}

fn draw_primary(mut commands: Commands) {}

fn cleanup(mut commands: Commands, q: Query<(Entity), With<SequenceViewItem>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
