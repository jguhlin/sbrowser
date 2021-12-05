use bevy::prelude::shape::CapsuleUvProfile;
use bevy::prelude::*;
use bevy::render::camera::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use bevy_mod_picking::*;
use rayon::prelude::*;

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
                .with_system(create_gff3_entities.system()),
        )
        .add_system_set(SystemSet::on_exit(AppState::SequenceView).with_system(cleanup.system()))
        .add_system_set(
            SystemSet::on_update(AppState::SequenceView).with_system(draw_feature.system()),
        );
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    bstate: Res<BrowserState>,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<(&Camera, &mut Transform), With<MainCamera>>,
    ui_setting: Res<UISetting>,
) {
    // Draw 3d chromosome on the main camera (could be another, for example if only looking at a gene, or something)

    let (landmark, length) = bstate.landmark.clone().unwrap();

    println!("Chr Length: {}", length);

/*    commands.spawn().insert(bevy::pbr::AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
     }).insert(Name::from("AmbientLight")); */

    let id = commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: Vec2::new(length as f32, 0.4),
                flip: false,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::BISQUE,
                // emissive: Color::WHITE * 10.0f32,
                ..Default::default()
            }),
            transform: Transform {
                rotation: Quat::from_rotation_ypr(0., 0., 0.), // std::f32::consts::FRAC_PI_2), // 1.5708),
                translation: Vec3::new(length as f32 / 2.0, 0., 0.),
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
        .insert(Name::from("Chromosome"))
        .id();

    // Draw vertical lines every 10kbp bases...
    // TODO: But only in visible region
    //for i in (0..length).step_by(100) {
    for i in (0..length).step_by(10000).take(10) {
        let x = i as f32;
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Quad {
                    size: Vec2::new(0.1, 5.5),
                    flip: false,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    ..Default::default()
                }),
                transform: Transform {
                    rotation: Quat::from_rotation_ypr(0., 0., 0.), // std::f32::consts::FRAC_PI_2), // 1.5708),
                    translation: Vec3::new(x, 0., 0.),
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
            .insert(Name::from("Tick"))
            .id();
    }

    let (camera, mut transform) = camera_query.single_mut().unwrap();
    
    println!("Camera: {:#?}", camera.projection_matrix);
    println!("Transform: {:#?}", transform);
    //*transform = transform.looking_at(Vec3::new(length as f32 / 2.0, 0., 0.), Vec3::new(0., 1., 0.));
    transform.translation = Vec3::new(length as f32 / 2.0, 0., 15.);
    println!("After Transform: {:#?}", transform);
    let x = camera.projection_matrix.transform_point3(Vec3::new(length as f32 / 2., 0., 0.));
    let scale = x.x / 15.;
    transform.scale.x = scale;
    println!("{:#?}", camera.projection_matrix.transform_point3(Vec3::new(length as f32 / 2., 0., 0.)));
    // transform.translation.x -= length as f32 / 2.0;
}

fn draw_ticks(
    mut commands: Commands,
    windows: Res<Windows>,
    mut camera_query: Query<(&Camera, &mut Transform), With<MainCamera>>,
) {
    let (camera, mut transform) = camera_query.single_mut().unwrap();
    let window = windows.get(camera.window).unwrap();

    // Have to calculate viewable space
    // Update at the start, then update when zoom in/out

    // TODO: Move to a less-frequently run system
}

fn draw_primary(mut commands: Commands) {}

fn create_gff3_entities(mut commands: Commands, bstate: Res<BrowserState>) {
    if bstate.landmark.is_none() {
        println!("bstate landmark is none");
        return;
    }

    let (landmark, length) = bstate.landmark.clone().unwrap();
    let features = bstate
        .gff3
        .as_ref()
        .unwrap()
        .parse_region(&landmark)
        .unwrap();

    println!("Drawing {}", features.len());

    commands.spawn_batch(features.into_iter().map(entity_bundle_from_gff3_feature));
}

fn entity_bundle_from_gff3_feature(feature: Feature) -> (SequenceViewItem, Name, Feature) {
    (SequenceViewItem, Name::from("Gene"), feature)
}

pub struct SequenceViewItemDrawn;

fn draw_feature(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Feature), (With<SequenceViewItem>, Without<SequenceViewItemDrawn>)>,
) {
    // TODO: Parallel in 0.6
    for (e, feature) in query.iter() {
        let mut entity = commands.entity(e);

        let width = (feature.end - feature.start) as f32;
        let width = width.abs();

        let coords = Vec3::new(
            ((feature.start as f32 + feature.end as f32) / 2.0) - 1.,
            -2.,
            0.0,
        );

        entity
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Quad {
                    size: Vec2::new(0.20, width), //(feature.end - feature.start) as f32),
                    flip: false,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..Default::default()
                }),
                transform: Transform {
                    rotation: Quat::from_rotation_ypr(0., 0., std::f32::consts::FRAC_PI_2), // 1.5708),
                    translation: coords,
                    scale: Vec3::new(1., 1., 1.),
                },
                ..Default::default()
            })
            .insert(SequenceViewItemDrawn)
            .insert_bundle(PickableBundle::default())
            .insert(BoundVol::default());
    }
}

/*
fn _draw_gff3_track(
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

    let (landmark, length) = bstate.landmark.clone().unwrap();
    let features = bstate
        .gff3
        .as_ref()
        .unwrap()
        .parse_region(&landmark)
        .unwrap();

    let mut y_offset: Vec<usize> = vec![0; features.len()];

    println!("{}", features.len());


    // TODO: Replace with bevy's par_iter_combinations in 0.6
    // TODO: Probably parallelize this or something...
    for feature in features.iter() {

        let fstart = std::cmp::min(feature.start, feature.end);
        let fend = std::cmp::max(feature.start, feature.end);

        for (n, f2) in features.iter().enumerate() {
            if feature == f2 {
                continue
            }

            let f2start = std::cmp::min(f2.start, f2.end);
            let f2end = std::cmp::max(f2.start, f2.end);

            // If f2 is contained completely within f1, it drops down...
            // TODO: Need to check for any overlap...
            if f2start >= fstart && f2end <= fend {
                y_offset[n] += 1;
            }
        }
    }

    for (n, feature) in features.iter().enumerate() {
        if feature.feature_type != "gene" {
            continue;
        }

        let width = (feature.end - feature.start) as f32;
        println!("Gene Length: {}", width);

        let mut coords = calc_coords_primitive(length as f32, feature.start as f32, feature.end as f32);

        coords.y += y_offset[n] as f32 * 0.1;
        // println!("{:#?}", coords);

        let id = commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Quad {
                    size: Vec2::new(0.10, width), //(feature.end - feature.start) as f32),
                    flip: false,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..Default::default()
                }),
                transform: Transform {
                    rotation: Quat::from_rotation_ypr(0., 0., std::f32::consts::FRAC_PI_2), // 1.5708),
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
} */

fn cleanup(mut commands: Commands, q: Query<Entity, With<SequenceViewItem>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
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
