use std::time::Duration;
use bevy::prelude::*;
use rand::Rng;
use crate::board::{Board, Position, SpawnSnakeSegment};
use crate::food::{Food, FoodType, NewFoodEvent};
use crate::scoring::Score;
use crate::snake::Snake;

pub mod board;
pub mod colors;
pub mod snake;
pub mod food;
pub mod controls;
pub mod ui;
pub mod scoring;


#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    #[default]
    Playing,
}

#[derive(PartialEq, Eq, Debug)]
enum GameOverReason {
    HitWall,
    HitSnake,
    Win,
}

#[derive(Resource)]
pub struct Countdown {
    pub main_timer: Timer,
    pub default_duration: Duration,
    pub change_step_duration: Duration,
    pub min_duration: Duration,
}

impl Countdown {
    pub fn new(duration_millis: u32, min_duration_millis: u64, change_step_millis: u64) -> Self {
        Self {
            main_timer: Timer::from_seconds(duration_millis as f32 / 1000.0, TimerMode::Repeating),
            default_duration: Duration::from_millis(duration_millis as u64),
            change_step_duration: Duration::from_millis(change_step_millis),
            min_duration: Duration::from_millis(min_duration_millis),
        }
    }
}

impl Default for Countdown {
    fn default() -> Self {
        Self::new(400, 100, 50)
    }
}

pub struct MainPlugin;
impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Countdown>()
            .add_systems(Update, tick.run_if(in_state(GameState::Playing)));
    }
}

pub fn tick(
    mut commands: Commands,
    time: Res<Time>,
    mut countdown: ResMut<Countdown>,
    mut snake: ResMut<Snake>,
    positions: Query<(Entity, &Position)>,
    input: Res<controls::Direction>,
    query_food: Query<(Entity, &Position, &Food), With<Food>>,
    mut food_event: EventWriter<NewFoodEvent>,
    query_board: Query<&Board>,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>
) {
    countdown.main_timer.tick(time.delta());
    if !countdown.main_timer.finished() {
        return;
    }

    let board = query_board.single();

    let mut next_position = snake.segments[0].clone();
    let hit_wall = match *input {
        controls::Direction::Up if next_position.y == board.size - 1 => { Some(GameOverReason::HitWall) }
        controls::Direction::Down if next_position.y == 0 => { Some(GameOverReason::HitWall) }
        controls::Direction::Left if next_position.x == 0 => { Some(GameOverReason::HitWall) }
        controls::Direction::Right if next_position.x == board.size - 1 => { Some(GameOverReason::HitWall) }
        controls::Direction::Up => { next_position.y += 1; None }
        controls::Direction::Down => { next_position.y -= 1; None }
        controls::Direction::Left => { next_position.x -= 1; None }
        controls::Direction::Right => { next_position.x += 1; None }
    };
    let hit_self = if snake.segments.contains(&next_position) {
        Some(GameOverReason::HitSnake)
    } else { None };
    let has_won = if snake.segments.len() == board.size as usize * board.size as usize {
        Some(GameOverReason::Win)
    } else {
        None
    };

    if hit_wall.is_some() || hit_self.is_some() || has_won.is_some() {
        next_state.set(GameState::Menu);
        return;
    }

    snake.segments.push_front(next_position);

    commands.add(SpawnSnakeSegment {position: next_position});
    let is_food = query_food.iter().find(|(_, pos, _)| &&next_position == pos );
    match is_food {
        None => {
            let old_tail = snake.segments.pop_back().unwrap();
            if let Some((entry, _)) = positions.iter().find(|(_, pos)| pos == &&old_tail) {
                commands.entity(entry).despawn_recursive();
            }
        }
        Some((entity, _, food)) => {
            commands.entity(entity).despawn_recursive();
            score.score += 1;
            let mut rng = rand::thread_rng();
            let v: f32 = rng.gen();
            let new_food = if v > 0.7 { FoodType::SpeedStopper } else { FoodType::General };
            food_event.send(NewFoodEvent {food_type: new_food});

            match food.food_type {
                FoodType::SpeedStopper => {
                    let def_duration = countdown.default_duration;
                    countdown.main_timer.set_duration(def_duration);
                }
                FoodType::General => {
                        let current_duration = countdown.main_timer.duration();
                        if current_duration > countdown.min_duration {
                            let new_duration = current_duration - countdown.change_step_duration;
                            countdown.main_timer.set_duration(new_duration);
                    }
                }
            }
        }
    }
}

pub fn reset_game(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    positions: Query<Entity, With<Position>>,
    mut last_pressed: ResMut<controls::Direction>,
    mut food_events: EventWriter<NewFoodEvent>,
    mut score: ResMut<Score>,
    mut countdown: ResMut<Countdown>,
) {
    for entity in positions.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let def_duration = countdown.default_duration;
    countdown.main_timer.set_duration(def_duration);

    food_events.send(NewFoodEvent {food_type: FoodType::General});
    *snake = Default::default();
    *last_pressed = Default::default();
    *score = Default::default();
}

