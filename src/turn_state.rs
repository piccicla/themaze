////////////////
/// turn_state.rs
/// 
/// authors: claudio piccinini picci2001@yahoo.it
/// updated:26/02/22
///
///////////////
///
/// 

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn
}