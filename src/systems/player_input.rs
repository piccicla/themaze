////////////////
/// player_input.rs
/// 
/// authors: claudio piccinini picci2001@yahoo.it
/// updated:26/03/22
///
 
use crate::prelude::*; 
///////////////
///

#[system]
//#[write_component(Point)]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input( ecs: &mut SubWorld,
                    //#[resource] map: &Map,
                    commands: &mut CommandBuffer,
                    #[resource] key: &Option<VirtualKeyCode>,
                    //#[resource] camera: &mut Camera,
                    #[resource] turn_state: &mut TurnState)
                    {

                        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

                        if let Some(key) = *key {
                            
                            let delta = match key {
                                VirtualKeyCode::Left => Point::new(-1,0),
                                VirtualKeyCode::Up => Point::new(0,-1),
                                VirtualKeyCode::Right => Point::new(1,0),
                                VirtualKeyCode::Down => Point::new(0,1),
                                _ => Point::new(0,0)

                            };

                            let (player_entity, destination) = players  
                                .iter(ecs)
                                .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
                                .unwrap();
                            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
                            if delta.x != 0 || delta.y != 0 {
                                let mut hit_something = false;
                                enemies
                                    .iter(ecs)
                                    .filter(|(_, pos)|{
                                        **pos == destination
                                    })
                                    .for_each(|(entity,_)|{
                                        hit_something = true;
                                        commands
                                            .push(((), WantsToAttack {
                                                attacker: player_entity,
                                                victim: *entity
                                            }));
                                    });

                                if !hit_something{
                                    commands
                                        .push(((), WantsToMove{
                                            entity: player_entity,
                                            destination
                                        }));
                                }

                            }

                            *turn_state = TurnState::PlayerTurn;

                            /*if delta.x != 0 || delta.y != 0 {
                                //let mut players = <&mut Point>::query().filter(component::<Player>());
                                /*players.iter_mut(ecs).for_each(|pos| {
                                    let destination = *pos + delta;
                                    if map.can_enter_tile(destination){
                                        *pos = destination;
                                        camera.on_player_move(destination);
                                        *turn_state = TurnState::PlayerTurn;
                                    }
                                });*/
                                players.iter_mut(ecs).for_each(|(entity,pos)| {
                                    let destination = *pos + delta;
                                    commands.push(((), WantsToMove{ entity: *entity, destination}));
                                 });
                                 *turn_state = TurnState::PlayerTurn;
                        
                            }*/
                        }
                    }