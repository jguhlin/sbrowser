use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::Inspectable;
use bevy_prototype_lyon::prelude::*;

mod genome;

use crate::genome::*;

fn main() {

    let genome = genome::get_genome();

    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(genome)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup.system())
        .add_startup_system(draw_chromosome.system())
        .run();
}

fn draw_chromosome(mut commands: Commands, genome: Res<Genome>) {

    let width = genome.length as f32 / 1024.0; // 1024 bp per pixel

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

        let width = (gene.end - gene.start) as f32 / 1024.0;

        let shape = shapes::Rectangle {
            width: 4.0,
            height: 10.0,
            origin:  shapes::RectangleOrigin::TopLeft,
        //            ..shapes::Rectangle::default()
        };

        println!("{}", gene.start as f32 / 1024.0);
        let start = gene.start as f32 / 1024.0;
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

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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