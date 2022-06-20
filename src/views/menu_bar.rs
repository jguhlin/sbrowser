use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

use crate::core::states::*;

pub struct MenuBarPlugin;
impl Plugin for MenuBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ui_example);
    }
}

fn ui_example(mut egui_ctx: ResMut<EguiContext>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });
}

fn menu_buttons(mut commands: Commands) {}

fn setup_menu(mut commands: Commands) {}

fn close_menu(mut commands: Commands) {}
