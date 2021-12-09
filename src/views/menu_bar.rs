use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

use crate::core::states::*;

pub struct MenuBarPlugin;
impl Plugin for MenuBarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(ui_example.system());
    }
}

fn ui_example(egui_ctx: Res<EguiContext>) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
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
