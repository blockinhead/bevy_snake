use bevy::prelude::*;
use crate::scoring::Score as ScoringScore;
use crate::scoring::Timer as ScoringTimer;
use crate::scoring::HighScore as ScoringHighScore;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, build_hud)
            .add_systems(Update, (update_score, update_high_score));
    }
}

#[derive(Component)]
pub struct Score;

#[derive(Component)]
pub struct HighScore;

#[derive(Component)]
pub struct Timer;

#[derive(Component)]
pub struct HighTimer;

fn build_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        background_color: HUD_COLOR.into(),
        style: HUD_STYLE,
        ..default()
    }).with_children(|parent| {
           // build score plate
            parent.spawn(NodeBundle{
                // background_color: HUD_INNER_COLOR.into(),
                style: HUD_INNER_STYLE,
                ..default() })
                .with_children(|parent| {
                    build_label(parent, &asset_server, "Score");
                    parent.spawn(TextBundle {
                        text: text_section("0000", &asset_server, 18.0, Color::WHITE).with_alignment(TextAlignment::Center),
                        ..default()
                    }).insert(Score);

                    build_label(parent, &asset_server, "Time");
                    parent.spawn(TextBundle {
                        text: text_section("0000", &asset_server, 18.0, Color::WHITE).with_alignment(TextAlignment::Center),
                        ..default()
                    }).insert(Timer);
                });
        // build highscore plate
        parent.spawn(NodeBundle{
            // background_color: HUD_INNER_COLOR.into(),
            style: HUD_INNER_STYLE,
            ..default() })
            .with_children(|parent| {
                build_label(parent, &asset_server, "HighScore");
                parent.spawn(TextBundle {
                    text: text_section("0000", &asset_server, 18.0, Color::WHITE).with_alignment(TextAlignment::Center),
                    ..default()
                }).insert(HighScore);

                build_label(parent, &asset_server, "BestTime");
                parent.spawn(TextBundle {
                    text: text_section("0000", &asset_server, 18.0, Color::WHITE).with_alignment(TextAlignment::Center),
                    ..default()
                }).insert(HighTimer);
            });
    });
}

fn update_score(
    score: Res<ScoringScore>,
    timer: Res<ScoringTimer>,
    mut query_score_hud: Query<(&Score, &mut Text), Without<Timer>>,
    mut query_timer_hud: Query<(&Timer, &mut Text), Without<Score>>
) {
    query_score_hud.single_mut().1.sections[0].value = format!("{:04}", score.score);

    match *timer {
        ScoringTimer { start: _, runtime: Some(val) } => { query_timer_hud.single_mut().1.sections[0].value = format!("{}", val.as_secs())}
        ScoringTimer { start: Some(val), runtime: None } => { query_timer_hud.single_mut().1.sections[0].value = format!("{}", val.elapsed().as_secs())}
        _ => {}
    }
}

fn update_high_score(
    highsocre: Res<ScoringHighScore>,
    mut query_highscore_hud: Query<(&HighScore, &mut Text), Without<HighTimer>>,
    mut query_hightimer_hud: Query<(&HighTimer, &mut Text), Without<HighScore>>,
) {
    query_highscore_hud.single_mut().1.sections[0].value = format!("{:04}", highsocre.score);
    query_hightimer_hud.single_mut().1.sections[0].value = format!("{:04}", highsocre.time.as_secs());
}



fn build_label(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, label: &str) {
    parent.spawn(TextBundle {
        text: Text::from_section(label, TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 20.0,
            color: Color::WHITE,
        }).with_alignment(TextAlignment::Center),
        ..default()
    });
}

fn text_section(value: &str, asset_server: &Res<AssetServer>, font_size: f32, color: Color) -> Text {
    Text::from_section(value,
                       TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size,
                                color
                       })
}

const HUD_COLOR: Color = Color::hsla(0.0, 0.0, 0.8, 1.0);
// const HUD_INNER_COLOR: Color = Color::hsla(0.0, 0.0, 0.9, 1.0);
const HUD_INNER_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap =  Val::Px(10.0);
    style
};
const HUD_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.width = Val::Px(200.0);
    style.height = Val::Px(250.0);
    style.position_type = PositionType::Absolute;
    style.right = Val::Percent(2.0);
    style.top = Val::Percent(2.0);
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.row_gap = Val::Px(15.0);
    style
};