use bevy::{input::mouse::MouseWheel, pbr::AmbientLight, pbr::Light, prelude::*};

use structs::*;

use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::InspectorPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
use bevy_prototype_lyon::prelude::*;
use bevy::render::camera::*;

mod core;
mod genome;
mod hover;
mod structs;
mod views;
mod utils;

use crate::core::states::*;
use crate::views::*;
use crate::genome::*;
use crate::hover::*;
use crate::structs::*;
use crate::utils::label_placer::*;

fn main() {
    let genome = genome::get_genome();

    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5 / 5.0f32,
        })
        .insert_resource(genome)
        .insert_resource(UISetting::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(DebugEventsPickingPlugin)
        .add_plugin(LabelPlacerPlugin)
        //.add_plugin(HoverPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(MenuBarPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(SequenceOverviewPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<Hoverable>::new())
        .add_startup_system(setup.system())
        // .add_startup_system(draw_chromosome.system())
        // .add_plugin(NoCameraPlayerPlugin)
        .add_system(camera_move.system())
        .add_system(mouse_scroll.system())
        //.add_system(hover_highlight.system())
        .add_state(AppState::SequenceOverview)
        // .add_system(zoom_chromosome.system())
        .run();
}

#[derive(Default)]
pub struct MainCamera;

fn draw_chromosome(mut commands: Commands, genome: Res<Genome>, ui_settings: Res<UISetting>) {
    for chr in &genome.chromosomes {
        let zf = ui_settings.zoom_factor;
        let width = chr.length as f32 / zf; // 1024 bp per pixel

        let shape = shapes::Rectangle {
            width: width,
            height: 20.0,
            //        origin:  shapes::RectangleOrigin::TopLeft,
            ..shapes::Rectangle::default()
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(Color::TEAL, Color::BLACK),
                DrawMode::Fill(FillOptions::default()),
                /*
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(10.0),
                }, */
                Transform::default(),
            ))
            .insert(chr.clone())
            .insert(Hoverable {
                height: 20.0,
                width: width,
                ..Default::default()
            });

        for gene in &chr.genes {
            let width = (gene.end - gene.start) as f32 / zf;

            let shape = shapes::Rectangle {
                width: width,
                height: 10.0,
                //    origin:  shapes::RectangleOrigin::TopLeft,
                ..shapes::Rectangle::default()
            };

            //        println!("{}", gene.start as f32 / zf);
            let start = gene.start as f32 / zf;
            //        let transform = Transform::from_translation(Vec3::new(gene.start as f32 / 1024.0, -50.0, 1.0));
            //        let transform = Transform::default();

            let coords = calc_coords(&chr, zf, gene);
            println!("{:#?}", coords);

            //        let transform = Transform::from_translation(Vec3::new(start, -50.0, 1.0));
            let transform = Transform::from_translation(coords);

            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    ShapeColors::outlined(Color::RED, Color::BLACK),
                    DrawMode::Fill(FillOptions::default()),
                    /*DrawMode::Outlined {
                        fill_options: FillOptions::default(),
                        outline_options: StrokeOptions::default().with_line_width(1.0),
                    },*/
                    transform,
                ))
                .insert(Hoverable {
                    height: 10.0,
                    width: width,
                    ..Default::default()
                });
        }
    }
}

/*

fn draw_chromosome(mut commands: Commands, genome: Res<Genome>, ui_settings: Res<UISetting>) {
    for chr in &genome.chromosomes {
        let zf = ui_settings.zoom_factor;
        let width = chr.length as f32 / zf; // 1024 bp per pixel

        let shape = shapes::Rectangle {
            width: width,
            height: 20.0,
            //        origin:  shapes::RectangleOrigin::TopLeft,
            ..shapes::Rectangle::default()
        };

        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                ShapeColors::outlined(Color::TEAL, Color::BLACK),
                DrawMode::Fill(FillOptions::default()),
                /*
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(10.0),
                }, */
                Transform::default(),
            ))
            .insert(chr.clone())
            .insert(Hoverable {
                height: 20.0,
                width: width,
                ..Default::default()
            });

        for gene in &chr.genes {
            let width = (gene.end - gene.start) as f32 / zf;

            let shape = shapes::Rectangle {
                width: width,
                height: 10.0,
                //    origin:  shapes::RectangleOrigin::TopLeft,
                ..shapes::Rectangle::default()
            };

            //        println!("{}", gene.start as f32 / zf);
            let start = gene.start as f32 / zf;
            //        let transform = Transform::from_translation(Vec3::new(gene.start as f32 / 1024.0, -50.0, 1.0));
            //        let transform = Transform::default();

            let coords = calc_coords(&chr, zf, gene);
            println!("{:#?}", coords);

            //        let transform = Transform::from_translation(Vec3::new(start, -50.0, 1.0));
            let transform = Transform::from_translation(coords);

            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shape,
                    ShapeColors::outlined(Color::RED, Color::BLACK),
                    DrawMode::Fill(FillOptions::default()),
                    /*DrawMode::Outlined {
                        fill_options: FillOptions::default(),
                        outline_options: StrokeOptions::default().with_line_width(1.0),
                    },*/
                    transform,
                ))
                .insert(Hoverable {
                    height: 10.0,
                    width: width,
                    ..Default::default()
                });
        }
    }
} */

fn setup(mut commands: Commands) {
    /*    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    }; */

    // commands.spawn_bundle(PerspectiveCameraBundle {
    //    transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //    ..Default::default()
    //}).insert(Camera).insert(FlyCam);

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0., 5., 5.)),
        /*         light: Light {
           intensity: 8000.,
           range: 1000.,
           ..Default::default()
        }, */
        ..Default::default()
    });

    //let mut camera_bundle = OrthographicCameraBundle::new_3d();
    let mut camera_bundle = PerspectiveCameraBundle::default();
    camera_bundle.transform = Transform::from_xyz(0., 0., 15.)
        .looking_at(Vec3::splat(0.0), camera_bundle.transform.local_y());

    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(camera_bundle)
        .insert(MainCamera)
        .insert(FlyCam)
        .insert_bundle(PickingCameraBundle::default());

    /*    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::TEAL, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(10.0),
        },
        Transform::default(),
    )); */
}

fn calc_coords(chr: &Chromosome, zf: f32, gene: &Gene) -> Vec3 {
    let width = chr.length as f32;

    let zero = -width / 2.0;

    let start_loc = zero + (gene.start as f32);
    //    let end_loc = zero + (gene.end as f32 / zf);

    let center = start_loc + ((gene.end - gene.start) as f32 / 2.0);

    Vec3::new(center / zf, -50.0, 1.0)
    //    Vec3::new(zero, -50.0, 1.0)
}

fn camera_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();

    for (_camera, mut transform) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let vert = Vec3::new(0.0, 1., 0.0);
        let horiz = Vec3::new(1., 0.0, 0.0);

        for key in keys.get_pressed() {
            match key {
                KeyCode::W => velocity += vert,
                KeyCode::S => velocity -= vert,
                KeyCode::A => velocity -= horiz,
                KeyCode::D => velocity += horiz,
                _ => (),
            }
        }

        velocity = velocity.normalize();

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds()
        }
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut ui_setting: ResMut<UISetting>,
    mut query: Query<(&Camera, &mut Transform), With<MainCamera>>,
) {
    let window = windows.get_primary().unwrap();

    let (_camera, mut transform) = query.single_mut().unwrap();

    for event in mouse_wheel_events.iter() {
        ui_setting.zoom_factor + event.y;
        // transform.scale += -event.y * Vec3::new(0.01, 0.00, 0.0);
        transform.scale.x += -event.y * 0.01 * transform.scale.x;
    }

    /*

    for (_camera, mut transform) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let vert = Vec3::new(0.0, 1.0, 0.0);
        let horiz = Vec3::new(1.0, 0.0, 0.0);

        for key in keys.get_pressed() {
            match key {
                KeyCode::W => velocity += vert,
                KeyCode::S => velocity -= vert,
                KeyCode::A => velocity -= horiz,
                KeyCode::D => velocity += horiz,
                _ => (),
            }
        }

        velocity = velocity.normalize();

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds() * 100.0
        }
    } */
}

pub struct Highlight;

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
}
