// Taken from another project of mine
// But updated thanks to the bevy cheatbook: https://erasin.wang/books/bevy-cheatbook/cookbook/cursor2world.html

use bevy::{prelude::*, render::camera::Camera, window::CursorMoved};

use bevy_inspector_egui::Inspectable;

pub struct HoverPlugin;
impl Plugin for HoverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(cursor_moved.system())
            .add_system(hoverable.system())
            .init_resource::<CursorState>();
        // app.init_resource::<CursorMovedState>();
        // .add_system_to_stage(stage::PRE_UPDATE, cursor_state.system())
        // .add_system_to_stage(stage::UPDATE, cursor_transform.system())
        // .add_system_to_stage(stage::UPDATE, hoverable.system());
    }
}

#[derive(Default)]
pub struct CursorState {
    pub cursor_world: Vec2,
    cursor_moved: bool,
}

pub struct Cursor;

#[derive(Inspectable)]
pub struct Hoverable {
    pub is: bool,
    pub height: f32,
    pub width: f32,
    pub changed: bool,
    pub highlight: Option<Entity>,
}

impl Default for Hoverable {
    fn default() -> Self {
        Hoverable {
            is: false,
            height: 0.0,
            width: 0.0,
            changed: false,
            highlight: None,
        }
    }
}

fn cursor_moved(
    mut e_cursor_moved: EventReader<CursorMoved>,
    wnds: Res<Windows>,
    q_camera: Query<&Transform, With<Camera>>,
    mut cursor_state: ResMut<CursorState>,
) {
    let e = e_cursor_moved.iter();

    let camera_transform = q_camera.iter().next().unwrap();

    for i in e {
        let wnd = wnds.get(i.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let p = i.position - size / 2.0;
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        cursor_state.cursor_world = Vec2::new(pos_wld.x, pos_wld.y);
        cursor_state.cursor_moved = true;
    }
}

fn hoverable(
    mut cursor_state: ResMut<CursorState>,
    mut q_hoverable: Query<(Entity, &Transform, &mut Hoverable)>,
) {
    let cursor_pos = cursor_state.cursor_world;
    if cursor_state.cursor_moved {
        for (_entity, transform, mut hoverable) in q_hoverable.iter_mut() {
            let half_width = hoverable.width / 2.0;
            let half_height = hoverable.height / 2.0;

            if transform.translation.x - half_width < cursor_state.cursor_world.x
                && transform.translation.x + half_width > cursor_state.cursor_world.x
                && transform.translation.y - half_height < cursor_state.cursor_world.y
                && transform.translation.y + half_height > cursor_state.cursor_world.y
            {
                if !hoverable.is {
                    hoverable.is = true;
                    hoverable.changed = true;
                }
            } else {
                if hoverable.is {
                    hoverable.is = false;
                    hoverable.changed = true;
                }
            }
        }
        cursor_state.cursor_moved = false;
    }
}
