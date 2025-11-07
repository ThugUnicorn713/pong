use bevy::math::Vec2;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]

pub struct GameState{
    pub ball: Vec2,
    pub paddle_one: Vec2,
    pub paddle_two: Vec2,
}