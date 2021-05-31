use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::prelude::*;

mod genome;

use crate::genome::*;

struct UISetting {
    zoom_factor: f32,
}

fn main() {

    let genome = genome::get_genome();

    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(genome)
        .insert_resource(UISetting { zoom_factor: 256.0 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_startup_system(draw_chromosome.system())
        .add_system(camera_move.system())
        .run();
}

#[derive(Default)]
pub struct Camera;

fn draw_chromosome(mut commands: Commands, genome: Res<Genome>, ui_settings: Res<UISetting>) {

    let zf = ui_settings.zoom_factor;
    let width = genome.length as f32 / zf; // 1024 bp per pixel

    println!("{}", width);

    let shape = shapes::Rectangle {
        width: width,
        height: 20.0,
        origin:  shapes::RectangleOrigin::TopLeft,
//        ..shapes::Rectangle::default()
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::TEAL, Color::BLACK),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(10.0),
        },
        Transform::default(),
    )); 

    for gene in &genome.genes {

        let width = (gene.end - gene.start) as f32 / zf;

        let shape = shapes::Rectangle {
            width: 4.0,
            height: 10.0,
            origin:  shapes::RectangleOrigin::TopLeft,
        //            ..shapes::Rectangle::default()
        };

        println!("{}", gene.start as f32 / zf);
        let start = gene.start as f32 / zf;
//        let transform = Transform::from_translation(Vec3::new(gene.start as f32 / 1024.0, -50.0, 1.0));
//        let transform = Transform::default();
        let transform = Transform::from_translation(Vec3::new(start, -50.0, 1.0));
    
        commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::RED, Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(2.0),
            },
            transform,
        )); 
    
    }

}

fn setup(mut commands: Commands) {
/*    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    }; */

    commands.spawn_bundle(OrthographicCameraBundle::new_2d()).insert(Camera);
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

fn camera_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    let window = windows.get_primary().unwrap();
    for (_camera, mut transform) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let forward = Vec3::new(0.0, 1.0, 0.0);
        let right = Vec3::new(1.0, 0.0, 0.0);

        for key in keys.get_pressed() {
            match key {
                KeyCode::W => velocity += forward,
                KeyCode::S => velocity -= forward,
                KeyCode::A => velocity -= right,
                KeyCode::D => velocity += right,
                _ => (),
            }
        }

        velocity = velocity.normalize();

        if !velocity.is_nan() {
            transform.translation += velocity * time.delta_seconds() * 100.0
        }
    }
}
