use bevy::ecs::system::Command;
use bevy::prelude::*;
use itertools::Itertools;

use crate::colors::COLORS;
use crate::food::{Food, FoodType};

const TILE_SIZE: f32 = 30.0;
const TILE_SPACER: f32 = 1.0;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, Component)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
pub struct Board {
    pub size: u8,
    physical_size: f32,
}

impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;

        Board {
            size,
            physical_size,
        }
    }

    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * TILE_SIZE;

        offset + f32::from(pos) * TILE_SIZE
               + f32::from(pos + 1) * TILE_SPACER
    }
}

pub fn spawn_board(
    mut commands: Commands,
    // snake: Res<Snake>,
    // mut food_event: EventWriter<NewFoodEvent>
) {
    let board = Board::new(20);

    commands.spawn(
        SpriteBundle {
            sprite: Sprite {
                color: COLORS.board,
                custom_size: Some(Vec2::new(board.physical_size, board.physical_size)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            for (x, y) in (0..board.size).cartesian_product(0..board.size) {
                builder.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: if (x + y) % 2 == 0 { COLORS.tile_placeholder }
                               else { COLORS.tile_placeholder_dark },
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE )),
                        ..default()},
                    transform: Transform::from_xyz(
                        board.cell_position_to_physical(x),
                        board.cell_position_to_physical(y),
                        1.0),
            ..default()
        });
    }})
        .insert(board);
}

pub struct SpawnSnakeSegment {
    pub position: Position
}

impl Command for SpawnSnakeSegment {
    fn apply(self, world: &mut World) {
        let board = world.query::<&Board>().iter(&world).next().unwrap();
        world.spawn(
            SpriteBundle {
                sprite: Sprite {
                    color: COLORS.snake,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(self.position.x),
                    board.cell_position_to_physical(self.position.y),
                    2.0
                ),
                ..default()
            }
        )
            .insert(self.position);
    }
}


pub struct SpawnApple {
    pub position: Position,
    pub food_type: FoodType
}

impl Command for SpawnApple {
    fn apply(self, world: &mut World) {
        let board = world.query::<&Board>().iter(&world).next().unwrap();
        let food_color = match self.food_type {
            FoodType::General => {COLORS.food_general}
            FoodType::SpeedStopper => {COLORS.food_special}
        };
        world.spawn(
            SpriteBundle {
                sprite: Sprite {
                    color: food_color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(self.position.x),
                    board.cell_position_to_physical(self.position.y),
                    2.0
                ),
                ..default()
            }
        )
            .insert(self.position)
            .insert(Food {food_type: self.food_type});
    }
}
