use instant::{Duration, Instant};
use bevy::app::App;
use bevy::prelude::*;
use crate::GameState;

#[derive(Resource, Default)]
pub struct Score {
    pub score: u32
}

#[derive(Resource, Default)]
pub struct HighScore {
    pub score: u32,
    pub time: Duration
}

#[derive(Resource, Debug)]
pub struct Timer {
    pub start: Option<Instant>,
    pub runtime: Option<Duration>,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            start: None,
            runtime: None,
        }
    }
}

pub struct ScorePlugin;



impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScore>()
            .init_resource::<Timer>()
            .add_systems(OnEnter(GameState::Playing), start_timer)
            .add_systems(OnExit(GameState::Playing), close_timer);
    }
}


fn start_timer(mut timer: ResMut<Timer>) {
    *timer = Timer {
        start: Some(Instant::now()),
        runtime: None,
    };
}

fn close_timer(
    mut timer: ResMut<Timer>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
) {
    let elapsed = timer.start.unwrap().elapsed();
    timer.runtime = Some(elapsed);
    if score.score > high_score.score || score.score == high_score.score && elapsed < high_score.time
    {
        *high_score = HighScore {
            score: score.score,
            time: elapsed,
        }
    }
}
