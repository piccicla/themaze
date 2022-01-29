////////////////
/// main.rs
/// The program entry point and main logic
/// authors: claudio piccinini picci2001@yahoo.it
/// updated:29/01/22
/// 
mod map;
mod player;
mod prelude{
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}
use prelude::*;

////////////////

struct State{
    map: Map,
    player: Player,
}
impl State{
    fn new() -> State{
        State{
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2)),
        }
    }
}
impl GameState  for State{
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    //println!("Hello, world!");
    let context = BTermBuilder::simple80x50()
    .with_title("The Maze")
    .with_fps_cap(30.0)
    .build()?;

    main_loop(context, State::new())
}


