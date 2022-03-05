////////////////
/// components.rs
/// 
/// authors: claudio piccinini picci2001@yahoo.it
/// updated:26/02/22
///
pub use crate::prelude::*;  //TODO: why is this public?
///////////////
///
/// 

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove{
    pub entity: Entity,
    pub destination: Point
}

