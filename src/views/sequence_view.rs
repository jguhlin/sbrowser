use bevy::prelude::shape::CapsuleUvProfile;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_mod_picking::*;

use crate::core::states::*;
use crate::structs::*;
use crate::utils::label_placer::*;
use crate::*;

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
                .with_system(draw_primary.system())
                .with_system(draw_gff3_track.system()),
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

fn draw_gff3_track(
    mut commands: Commands,
    bstate: Res<BrowserState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    ui_setting: Res<UISetting>,
) {
    if bstate.landmark.is_none() {
        return;
    }

    let landmark = bstate.landmark.clone().unwrap();
    let (features, length) = bstate
        .gff3
        .as_ref()
        .unwrap()
        .parse_region(&landmark)
        .unwrap();

    println!("{}", features.len());

    for feature in features {
        if feature.feature_type != "gene" {
            continue;
        }

        let coords = calc_coords_primitive(86460390 as f32, ui_setting.zoom_factor, feature.start as f32, feature.end as f32);

        let id = commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Quad {
                    size: Vec2::new(0.12, 2.0), //(feature.end - feature.start) as f32),
                    flip: false,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..Default::default()
                }),
                transform: Transform {
                    rotation: Quat::from_rotation_ypr(0., 0., std::f32::consts::FRAC_PI_2), // 1.5708),
                    //translation: Vec3::new(0., 10., 0.),
                    translation: coords,
                    scale: Vec3::new(1., 1., 1.),
                },
                ..Default::default()
            })
            .insert_bundle(PickableBundle::default())
            .insert(BoundVol::default())
            .insert(LabelBase)
            .insert(SequenceViewItem)
            .insert(Name::from("Gene"))
            .id();
    }
}

fn cleanup(mut commands: Commands, q: Query<(Entity), With<SequenceViewItem>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
