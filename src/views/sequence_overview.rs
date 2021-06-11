use bevy::prelude::*;
use bevy::prelude::shape::CapsuleUvProfile;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_mod_picking::*;
use bevy::render::camera::*;

use crate::core::states::*;
use crate::utils::label_placer::*;

pub struct SequenceOverviewPlugin;
impl Plugin for SequenceOverviewPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::SequenceOverview)
                .with_system(ui_example.system())
                .with_system(menu_buttons.system()),
        )
        .add_system_set(SystemSet::on_enter(AppState::SequenceOverview).with_system(setup.system()))
        .add_system_set(
            SystemSet::on_exit(AppState::SequenceOverview).with_system(close_menu.system()),
        );
    }
}

fn ui_example(egui_ctx: Res<EguiContext>) {}

fn menu_buttons(mut commands: Commands) {}

// To put UI text where it goes:
// https://github.com/aevyrie/bevy_world_to_screenspace/blob/master/src/main.rs

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 16.,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment::default();

    for i in 0..5 {
        let id = 
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    // latitudes: 2,
                    // longitudes: 16,
                    depth: 1.4,
                    radius: 0.4,
                    uv_profile: CapsuleUvProfile::Uniform,
                    ..Default::default() //subdivisions: 4,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::BISQUE,
                    // emissive: Color::WHITE * 10.0f32,
                    ..Default::default()
                }),
                transform: Transform {
                    rotation: Quat::from_rotation_ypr(0., 0., 1.5708),
                    translation: Vec3::new(i as f32 * 4., 0., 0.),
                    scale: Vec3::new(1., 1., 1.),
                    ..Default::default()
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
            .id();

        commands.spawn_bundle(TextBundle {
            text: Text::with_section("translation", text_style.clone(), text_alignment),
            style: Style {
                position: Rect {
                    bottom: Val::Px(0.),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                flex_grow: 0.,
                flex_shrink: 0.,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            transform: Transform::default(),
            ..Default::default()
        })
        .insert(Label::belongs_to(id));
    } 
}

fn close_menu(mut commands: Commands) {}
