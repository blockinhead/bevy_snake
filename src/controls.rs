use bevy::app::App;
use bevy::prelude::{in_state, Input, IntoSystemConfigs, KeyCode, Plugin, Res, ResMut, Resource, Update};

#[derive(Resource)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}
use Direction::*;
use crate::GameState;

impl Default for Direction {
    fn default() -> Self {
        Right
    }
}

fn user_input(
    input: Res<Input<KeyCode>>,
    mut last_pressed: ResMut<Direction>
) {
    if input.pressed(KeyCode::Up) { *last_pressed = Up }
    else if input.pressed(KeyCode::Down) { *last_pressed = Down }
    else if input.pressed(KeyCode::Left) { *last_pressed = Left }
    else if input.pressed(KeyCode::Right) { *last_pressed = Right }
}

pub struct ControlsPlugin;
impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Direction>()
            .add_systems(Update, user_input.run_if(in_state(GameState::Playing)));
    }
}
