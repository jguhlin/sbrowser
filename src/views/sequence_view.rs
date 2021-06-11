use bevy::prelude::*;
use bevy::prelude::shape::CapsuleUvProfile;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_mod_picking::*;
use bevy::render::camera::*;

use crate::core::states::*;
use crate::utils::label_placer::*;

pub struct SequenceViewItem;

pub struct SequenceViewPlugin;
impl Plugin for SequenceViewPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::SequenceView)
                .with_system(ui_example.system())
                .with_system(print_events.system())
                .with_system(menu_buttons.system()),
        )
        .add_system_set(SystemSet::on_enter(AppState::SequenceView).with_system(setup.system()))
        .add_system_set(
            SystemSet::on_exit(AppState::SequenceView).with_system(cleanup.system()),
        );
    }
}

pub fn print_events(mut events: EventReader<PickingEvent>,
    mut state: ResMut<State<AppState>>,
    //    query: Query<()>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(SelectionEvent::JustSelected(x)) = *event {
            println!("Got event...");
            // TODO: Should fire off an event or config to load up the new sequence...
            state.replace(AppState::ChromosomeView).unwrap();
        }
    }
}

fn ui_example(egui_ctx: Res<EguiContext>) {}

fn menu_buttons(mut commands: Commands) {}

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
                    translation: Vec3::new(-8. + i as f32 * 4., 0., 0.),
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
            .insert(SequenceViewItem)
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
        .insert(Label::belongs_to(id).with_offset(Vec3::new(0., 7.0, 0.)))
        .insert(SequenceViewItem);
    } 
}

fn cleanup(mut commands: Commands,
    q: Query<(Entity), With<SequenceViewItem>>,) {

    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}