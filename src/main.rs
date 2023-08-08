use bevy::prelude::*;
use snake::board::spawn_board;
use snake::controls::ControlsPlugin;
use snake::food::FoodPlugin;
use snake::snake::Snake;
use snake::{GameState, MainPlugin, reset_game};
use snake::scoring::ScorePlugin;
use snake::ui::hud::HudPlugin;
use snake::ui::menu::UiPlugin;


fn main() {
    App::new()
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_board)
        .insert_resource::<Snake>(Default::default())
        .insert_resource(ClearColor(Color::rgb(0.2, 0.5, 0.7)))
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window{
                    title: "snake".into(),
                    canvas: Some("#bevy".to_owned()),
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins(ControlsPlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(MainPlugin)
        .add_systems(OnEnter(GameState::Playing), reset_game)
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default());
}
