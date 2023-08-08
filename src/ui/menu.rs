use bevy::prelude::*;
use crate::GameState;
use crate::ui::button;


pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_ui)
            .add_systems(Update, button::text_button_system)
            .add_systems(OnEnter(GameState::Menu), show_menu)
            .add_systems(OnEnter(GameState::Playing), hide_menu);
    }
}

#[derive(Component)]
struct MainMenu;

fn show_menu(mut menu: Query<&mut Visibility, With<MainMenu>>) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Visible;
}

fn hide_menu(mut menu: Query<&mut Visibility, With<MainMenu>>) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Hidden;
}

pub fn game_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((NodeBundle {
        background_color: BackgroundColor(Color::Hsla {
            hue: 0.0,
            saturation: 0.0,
            lightness: 100.0,
            alpha: 0.2,
        }),
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }, MainMenu))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                background_color: BackgroundColor( Color::Hsla { hue: 0.0, saturation: 0.0, lightness: 100.0, alpha: 0.4 }),
                style: Style {
                    width: Val::Px(500.0),
                    height: Val::Px(350.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    position_type: PositionType::Absolute,
                    align_self: AlignSelf::Center,
                    border: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                button::spawn_button(parent, &asset_server, "New Game");
                button::spawn_button(parent, &asset_server, "Exit");
            });
        });
}