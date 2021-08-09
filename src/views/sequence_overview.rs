use bevy::prelude::shape::CapsuleUvProfile;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_mod_picking::*;

use crate::core::states::*;
use crate::parsers::*;
use crate::utils::label_placer::*;
use crate::structs::*;

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

struct SequenceOverviewItem;

pub struct SequenceOverviewPlugin;
impl Plugin for SequenceOverviewPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::SequenceOverview)
                .with_system(ui_example.system())
                .with_system(print_events.system())
                .with_system(menu_buttons.system()),
        )
        .add_system_set(SystemSet::on_enter(AppState::SequenceOverview).with_system(setup.system()))
        .add_system_set(
            SystemSet::on_exit(AppState::SequenceOverview).with_system(cleanup.system()),
        );
    }
}

pub fn print_events(
    mut events: EventReader<PickingEvent>,
    mut state: ResMut<State<AppState>>,
    query: Query<(&ClickableLandmark)>,
) {
    for event in events.iter() {
        if let PickingEvent::Selection(SelectionEvent::JustSelected(x)) = *event {
            println!("Got event...");
            // TODO: Should fire off an event or config to load up the new sequence...
            state.replace(AppState::SequenceView).unwrap();
            let j = query.get(x).unwrap();
            println!("{:#?}", j);
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
    genome: Option<Res<Gff3>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 16.,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment::default();

    if genome.is_none() {
        return;
    }

    let genome = genome.unwrap();

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
                    rotation: Quat::from_rotation_ypr(0., 0., std::f32::consts::FRAC_PI_2),                   
                    translation: Vec3::new(-8. + col as f32 * 4., row as f32 * -1., 0.),
                    scale: Vec3::new(1., 1., 1.),
                },
                ..Default::default()
            })
            .insert_bundle(PickableBundle::default())
            .insert(BoundVol::default())
            .insert(LabelBase)
            .insert(SequenceOverviewItem)
            .insert(ClickableLandmark::from(&landmark.0))
            .id();

        commands
            .spawn_bundle(TextBundle {
                text: Text::with_section(landmark.0.to_string(), text_style.clone(), text_alignment),
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
            .insert(SequenceOverviewItem);
    }
}

fn cleanup(mut commands: Commands, q: Query<(Entity), With<SequenceOverviewItem>>) {
    println!("Cleaning!");

    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
