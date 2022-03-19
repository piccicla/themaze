////////////////
/// main.rs
/// The program entry point and main logic
/// authors: claudio piccinini picci2001@yahoo.it
/// updated:29/01/22
/// 
mod map;
mod map_builder;
//mod player;
mod camera;
mod components;
mod spawner;
mod turn_state;
mod systems;

mod prelude{
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    //pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;

    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;

    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::turn_state::*;
    pub use crate::systems::*;
}
use prelude::*;

////////////////

struct State{
    //map: Map,
    //player: Player,
    //camera: Camera
    ecs: World,
    resources: Resources,
    //systems: Schedule,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule
}

impl State{
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        
        map_builder.rooms.iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        Self {
            //map: Map::new(),
            //player: Player::new(Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2)),
            //map: map_builder.map,
            //player: Player::new(map_builder.player_start),
            //camera: Camera::new(map_builder.player_start)
            ecs,
            resources,
            //systems: build_scheduler() //TODO: add function
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler()          
        }
    }
}
impl GameState  for State{
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();       
        //self.player.update(ctx, &self.map, &mut self.camera);
        //self.map.render(ctx, &self.camera);
        //self.player.render(ctx, &self.camera);

        self.resources.insert(ctx.key);
        //self.systems.execute(&mut self.ecs, &mut self.resources);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state{
            TurnState::AwaitingInput => self.input_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self.player_systems.execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self.monster_systems.execute(&mut self.ecs, &mut self.resources)
        }
        render_draw_buffer(ctx).expect("render error");
    }
}

fn main() -> BError {
    //println!("Hello, world!");
    /*let context = BTermBuilder::simple80x50()
    .with_title("The Maze")
    .with_fps_cap(30.0)
    .build()?;*/

    let context = BTermBuilder::new()
    .with_title("The Maze")
    .with_fps_cap(30.0)
    .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    .with_tile_dimensions(32, 32)
    .with_resource_path("resources/")
    .with_font("dungeonfont.png", 32, 32)
    .with_font("terminal8x8.png", 8, 8)
    .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
    .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
    .with_simple_console_no_bg(SCREEN_WIDTH*2, SCREEN_HEIGHT*2, "terminal8x8.png")  
    .build()?;

    main_loop(context, State::new())
}


