use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
// use clap::{AppSettings, Clap};

use crate::core::states::*;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(AppState::MainMenu)
                .with_system(ui_example.system())
                .with_system(menu_buttons.system()),
        )
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup_menu.system()))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(close_menu.system()));
    }
}

fn ui_example(egui_ctx: Res<EguiContext>) {}

fn menu_buttons(mut commands: Commands) {}

fn setup_menu(mut commands: Commands) {}

fn close_menu(mut commands: Commands) {}
