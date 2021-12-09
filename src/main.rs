use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[macro_use]
extern crate jetscii;

use bevy::{input::mouse::MouseWheel, pbr::AmbientLight, pbr::Light, prelude::*};
use bevy_prototype_debug_lines::*;

use bevy::render::camera::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
// use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorParams, WorldInspectorPlugin};
use bevy_mod_picking::*;

mod core;
mod genome;
mod hover;
mod parsers;
mod structs;
mod utils;
mod views;

use structs::*;

use crate::core::states::*;
use crate::genome::*;
use crate::hover::*;
use crate::parsers::feature;
use crate::parsers::*;
use crate::structs::*;
use crate::utils::label_placer::*;
use crate::views::*;

fn main() {
    // let genome = genome::get_genome_from_gff3("converted.sorted.s.gff3");

    //let genome = Gff3::parse("converted.sorted.s.gff3").expect("Unable to parse GFF3");
    let genome = Gfa::parse("/mnt/data/kakapo-assemblies/RichardHenry/out.gfa")
        .expect("Unable to parse GFA");

    let mut bstate = BrowserState::default();
    //bstate.gff3 = Some(genome.clone());
    bstate.gfa = Some(genome.clone());

    let mut app = App::build();

    app.insert_resource(Msaa { samples: 8 })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5f32,
        })
        .insert_resource(genome)
        .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(genome)
        .insert_resource(UISetting::default())
        .insert_resource(bstate)
        .add_event::<CameraMoved>()
        .add_event::<LoadLandmark>()
        .add_plugins(DefaultPlugins)
        .insert_resource(EntityRegistry::default())
        .insert_resource(ExpansionRounds { round: 0 }) // DEBUG: Probably a temporary thing...
        .insert_resource(WorldInspectorParams {
            despawnable_entities: true,
            ..Default::default()
        })
        .add_plugin(EguiPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(DebugEventsPickingPlugin)
        .add_plugin(LabelPlacerPlugin)
        .add_plugin(DebugLinesPlugin)
        // .insert_resource(DebugLines { depth_test: true, ..Default::default() })
        //.add_plugin(HoverPlugin)
        .add_plugin(MenuBarPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(SequenceOverviewPlugin)
        .add_plugin(SequenceViewPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<Hoverable>::new())
        .add_startup_system(setup.system())
        // .add_startup_system(draw_chromosome.system())
        // .add_plugin(NoCameraPlayerPlugin)
        .add_system(camera_move.system())
        .add_system(mouse_scroll.system())
        //.add_system(hover_highlight.system())
        .add_state(AppState::SequenceOverview);
    // .add_system(zoom_chromosome.system())

    // getting registry from world
    let mut registry = app
        .world_mut()
        .get_resource_or_insert_with(InspectableRegistry::default);

    // registering custom component to be able to edit it in inspector
    registry.register::<Label>();
    registry.register::<Feature>();

    app.run();
}

#[derive(Default)]
pub struct MainCamera;

fn setup(mut commands: Commands, mut ev_cameramoved: EventWriter<CameraMoved>) {
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0., 5., 5.)),
        ..Default::default()
    });

    let mut camera_bundle = PerspectiveCameraBundle::default();
    camera_bundle.transform = Transform::from_xyz(0., 0., 15.)
        .looking_at(Vec3::splat(0.0), camera_bundle.transform.local_y());

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(camera_bundle)
        .insert(MainCamera)
        // .insert(FlyCam)
        .insert_bundle(PickingCameraBundle::default());

    // Trigger label placements
    ev_cameramoved.send(CameraMoved);
}

pub fn calc_coords(chr: &Chromosome, zf: f32, gene: &Gene) -> Vec3 {
    let width = chr.length as f32;

    let zero = -width / 2.0;

    let start_loc = zero + (gene.start as f32);

    let center = start_loc + ((gene.end - gene.start) as f32 / 2.0);

    Vec3::new(center / zf, -50.0, 1.0)
}

pub fn calc_coords_primitive(chr_length: f32, gene_start: f32, gene_end: f32) -> Vec3 {
    let zero = -chr_length as f32 / 2.0;

    let start_loc = zero + (gene_start as f32);

    let center = start_loc + ((gene_end - gene_start) as f32 / 2.0);

    //Vec3::new(center / zf, 2.0, 1.0)
    Vec3::new(center, 2.0, 0.0)
}

fn camera_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut query: Query<(&Camera, &mut Transform), With<MainCamera>>,
    mut ev_cameramoved: EventWriter<CameraMoved>,
    ui_setting: Res<UISetting>,
) {
    if keys.get_pressed().len() == 0 {
        return;
    }

    let window = windows.get_primary().unwrap();

    for (_camera, mut transform) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let vert = Vec3::new(0.0, 0.8, 0.0);
        let horiz = Vec3::new(1.05, 0.0, 0.0);
        let zoom = Vec3::new(0.0, 0.0, 0.05);

        for key in keys.get_pressed() {
            match key {
                KeyCode::W => velocity += vert,
                KeyCode::S => velocity -= vert,
                KeyCode::A => velocity -= horiz * ui_setting.zoom_factor,
                KeyCode::D => velocity += horiz * ui_setting.zoom_factor,
                KeyCode::Z => velocity += zoom * ui_setting.zoom_factor,
                KeyCode::X => velocity -= zoom * ui_setting.zoom_factor,
                _ => (),
            }
        }

        // println!("ZF: {}", ui_setting.zoom_factor);

        // velocity = velocity.normalize();

        if !velocity.is_nan() && velocity.abs() > Vec3::ZERO {
            transform.translation += velocity * time.delta_seconds();
            ev_cameramoved.send(CameraMoved);
        }
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut ui_setting: ResMut<UISetting>,
    mut query: Query<(&Camera, &mut Transform), With<MainCamera>>,
    mut ev_cameramoved: EventWriter<CameraMoved>,
) {
    let window = windows.get_primary().unwrap();

    let (_camera, mut transform) = query.single_mut().unwrap();

    for event in mouse_wheel_events.iter() {
        ui_setting.zoom_factor += -event.y;
        // transform.scale += -event.y * Vec3::new(0.01, 0.00, 0.0);
        transform.scale.x += -event.y * 0.01 * transform.scale.x;

        ev_cameramoved.send(CameraMoved);

        // TODO: Maybe remove this? This scales vertical, useful for debugging....
        // transform.scale.y += -event.y * 0.01 * transform.scale.y;
    }
}

pub struct Highlight;
/* 
fn hover_highlight(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Hoverable, &mut ShapeColors, &Transform), (Changed<Hoverable>)>,
) {
    for (e, mut hov, mut sc, transform) in q.iter_mut() {
        if hov.changed && hov.is {
            // Display a highlight

            let shape = shapes::Rectangle {
                width: hov.width,
                height: hov.height,
                ..shapes::Rectangle::default()
            };

            let mut transform = transform.clone();
            transform.translation.z = 2.0;

            let highlight = commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    ShapeColors::outlined(Color::YELLOW, Color::YELLOW),
                    DrawMode::Fill(FillOptions::default()),
                    transform,
                ))
                .insert(Highlight)
                .id();

            hov.highlight = Some(highlight);
            hov.changed = false;
        }

        if hov.changed && !hov.is {
            commands.entity(hov.highlight.take().unwrap()).despawn();

            hov.changed = false;
        }
    }
} */
