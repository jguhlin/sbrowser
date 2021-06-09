use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

use crate::core::states::*;

pub struct SequenceOverviewPlugin;
impl Plugin for SequenceOverviewPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::SequenceOverview)
                .with_system(ui_example.system())
                .with_system(menu_buttons.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::SequenceOverview).with_system(setup.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::SequenceOverview).with_system(close_menu.system()),
        );
    }
}

fn ui_example(egui_ctx: Res<EguiContext>) {}

fn menu_buttons(mut commands: Commands) {}

fn setup(mut commands: Commands,     mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    println!("Run...");
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 125.,
            subdivisions: 1,
        })),
        
        material: materials.add(StandardMaterial {
            base_color: Color::YELLOW,
            // emissive: Color::WHITE * 10.0f32,
            ..Default::default()
        }),

//        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//        material: materials.add(StandardMaterial {
//            base_color: Color::hex("ffd891").unwrap(),
            // vary key PBR parameters on a grid of spheres to show the effect
//            ..Default::default()
//        }),
//        transform: Transform::from_xyz(-5.0, -2.5, 0.0),
        ..Default::default()
    });
}

fn close_menu(mut commands: Commands) {}
