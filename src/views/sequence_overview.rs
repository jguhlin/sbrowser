use bevy::prelude::shape::CapsuleUvProfile;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_mod_picking::*;

use crate::core::states::*;
use crate::parsers::*;
use crate::structs::*;
use crate::utils::label_placer::*;

enum SequenceType {
    Genome,
    Protein,
    Transcript,
    RNA,
}

pub struct Sequence {
    seq_type: SequenceType,
    // file_path: String,
}

#[derive(Component)]
struct SequenceOverviewItem;

pub struct SequenceOverviewPlugin;
impl Plugin for SequenceOverviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::SequenceOverview)
                .with_system(ui_example)
                .with_system(print_events)
                .with_system(menu_buttons),
        )
        .add_system_set(SystemSet::on_enter(AppState::SequenceOverview).with_system(setup))
        .add_system_set(
            SystemSet::on_exit(AppState::SequenceOverview).with_system(cleanup),
        );
    }
}

pub fn print_events(
    mut events: EventReader<PickingEvent>,
    mut state: ResMut<State<AppState>>,
    mut ev: EventWriter<LoadLandmark>,
    query: Query<&ClickableLandmark>,
    mut bstate: ResMut<BrowserState>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(SelectionEvent::JustSelected(x)) = *event {
            println!("Got event...");
            // TODO: Should fire off an event or config to load up the new sequence...
            state.replace(AppState::SequenceView).unwrap();
            let j = query.get(x).unwrap();
            println!("{:#?}", j);
            ev.send(LoadLandmark { id: j.id.clone() });
            bstate.landmark = Some((j.id.clone(), j.length));
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
    gff3: Option<Res<Gff3>>,
    gfa: Option<Res<Gfa>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 16.,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment::default();

    if gff3.is_none() && gfa.is_none() {
        return;
    }

    if gff3.is_some() {
        let genome = gff3.unwrap();

        for (i, landmark) in genome.landmarks.iter().enumerate() {
            // 5 per row
            let row = i / 5;
            let col = i % 5;

            println!("{:#?} {:#?}", row, col);

            let id = commands
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
                        rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., std::f32::consts::FRAC_PI_2),
                        translation: Vec3::new(-8. + col as f32 * 4., row as f32 * -1., 0.),
                        scale: Vec3::new(1., 1., 1.),
                    },
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(LabelBase)
                .insert(SequenceOverviewItem)
                .insert(ClickableLandmark::from(&landmark.0, landmark.3))
                .id();

            commands
                .spawn_bundle(TextBundle {
                    text: Text::from_section(
                        landmark.0.to_string(),
                        text_style.clone(),
                    ).with_alignment(text_alignment),
                    style: Style {
                        position: UiRect {
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
                .insert(SequenceOverviewItem);
        }
    }

    if gfa.is_some() {
        let genome = gfa.unwrap();

        for (i, landmark) in genome.segments.keys().enumerate() {
            let length = *genome.lengths.get(landmark).unwrap();
            if length < 500 {
                // IMPORTANT: Filter out ones 500bp and below
                continue;
            }

            // 5 per row
            let row = i / 5;
            let col = i % 5;

            println!("{:#?} {:#?}", row, col);

            let id = commands
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
                        rotation: Quat::from_euler(EulerRot::XYZ, 0., 0., std::f32::consts::FRAC_PI_2),
                        translation: Vec3::new(-8. + col as f32 * 4., row as f32 * -1., 0.),
                        scale: Vec3::new(1., 1., 1.),
                    },
                    ..Default::default()
                })
                .insert_bundle(PickableBundle::default())
                .insert(LabelBase)
                .insert(SequenceOverviewItem)
                .insert(ClickableLandmark::from(
                    &landmark,
                    *genome.lengths.get(landmark).unwrap(),
                ))
                .id();

            commands
                .spawn_bundle(TextBundle {
                    text: Text::from_section(
                        landmark.to_string(),
                        text_style.clone(),
                    ).with_alignment(text_alignment),
                    style: Style {
                        position: UiRect {
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
                .insert(SequenceOverviewItem);
        }
    }
}

fn cleanup(mut commands: Commands, q: Query<(Entity), With<SequenceOverviewItem>>) {
    println!("Cleaning!");

    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
