use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//Distinguishes PlayerIDs to numbers
type PlayerId = u64;

//What state the game itself could be in
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    PreGame,
    InGame,
    Ended,
}

//What token the board could place
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Token {
    Empty,
    Red,
    Yellow,
}

//All important information about a player
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub token: Token,
}

//All events that progresses the game
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    BeginGame { goes_first: PlayerId },
    WonGame { winner: PlayerId },
    PlayerJoined { player_id: PlayerId, name: String },
    PlayerLeft { player_id: PlayerId },
    PlaceToken { player_id: PlayerId, xPos: usize },
}

//Holds the current state of the game
pub struct GameState {
    pub stage: Stage,
    pub board: Vec<Token>,
    pub active_player: PlayerId,
    pub players: HashMap<PlayerId, Player>,
    pub history: Vec<GameEvent>,
}

//Default object for a game state
impl Default for GameState {
    fn default() -> Self {
        let width = 7;
        let height = 6;

        Self {
            stage: Stage::PreGame,
            board: vec![Token::Empty; (width*height) as usize],
            active_player: 0,
            players: HashMap::new(),
            history: Vec::new(),
        }
    }
}

impl GameState {

    //This function assumes the GameEvent being passed through is valid.
    fn validate( &self, event: &GameEvent ) -> bool {
        use GameEvent::*;

        match event {
            BeginGame { goes_first: PlayerId },
            WonGame { winner: PlayerId },
            PlayerJoined { player_id: PlayerId, name: String },
            PlayerLeft { player_id: PlayerId },
            PlaceToken { player_id: PlayerId, xPos: usize },

            PlayerJoined { player_id, name } => {
                if self.players.contains_key(player_id) {
                    return false;
                }
            }
        }

        true
    }

    //This function assumes the event being passed through is valid.
    //This will take the event and update the GameState accordingly.
    fn reduce( &mut self, valid_event: &GameEvent ) {
        use GameEvent::*;
        match valid_event {
            BeginGame { goes_first } => {
                self.active_player = goes_first;
                self.stage = Stage::InGame;
            }
            WonGame { winner } => {
                self.stage = Stage::Ended;
            }
            PlayerJoined { player_id, name } => {
                self.players.insert(
                    player_id,
                    Player {
                        name: name.to_string(),
                        //First player gets red, second gets yellow.
                        token: if self.players.len() > 0 {
                            Token::Red
                        } else {
                            Tile::Yellow
                        },
                    },
                );
            }
            PlayerLeft { player_id: PlayerId }
            PlaceToken { player_id: PlayerId, xPos: usize },
        }

        self.history.push(valid_event.clone());
    }

    //Checks if the event is valid: if it is, reduce. If not, return an error result.
    pub fn dispatch( &mut self, event: &GameEvent ) -> Result<(), ()> {
        if !self.validate(event) {
            return Err(());
        }

        self.reduce(event);
        return Ok(());
    }
}