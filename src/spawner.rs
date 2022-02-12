////////////////
/// spawner.rs
/// 
/// authors: claudio piccinini picci2001@yahoo.it
/// updated:12/02/22
///
use crate::prelude::*;
///////////////
///
/// 


pub fn spawn_player(ecs:&mut World, pos: Point){
    ecs.push(
        (
            Player,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            }
        )
    );
}