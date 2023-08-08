use bevy::app::AppExit;
use bevy::prelude::*;
use crate::GameState;


const NORMAL_BUTTON: Color = Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.0,
    alpha: 1.0,
};
const HOVERED_BUTTON: Color = Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.25,
    alpha: 1.0,
};
const PRESSED_BUTTON: Color = Color::Hsla {
    hue: 0.0,
    saturation: 0.0,
    lightness: 0.0,
    alpha: 1.0,
};

pub fn text_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    text_query: Query<&Text>,
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match text.sections[0].value.as_str() {
                    "New Game" => { next_state.set(GameState::Playing) }
                    "Exit" => { exit.send(AppExit) }
                    _ => { unimplemented!("Button goes nowhere") }
                }
            }
            Interaction::Hovered => { *color = HOVERED_BUTTON.into() }
            Interaction::None => { *color = NORMAL_BUTTON.into() }
        }
    }
}


pub fn spawn_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
) {
    parent.spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(65.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}