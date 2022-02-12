////////////////
/// components.rs
/// 
/// authors: claudio piccinini picci2001@yahoo.it
/// updated:12/02/22
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