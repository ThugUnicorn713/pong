use bevy::math::Vec2;
use serde::{Serialize, Deserialize};
use bevy::prelude::Resource;

#[derive(Resource, Serialize, Deserialize, Debug, Default)]

pub struct GameState{
    pub ball: Vec2,
    pub paddle_one: Vec2,
    pub paddle_two: Vec2,
}