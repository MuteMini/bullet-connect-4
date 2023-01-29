use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientMessage {
    CreateRoom{ player_name: String },
    JoinRoom{ room_uuid: String, player_name: String },

    PlaceToken{ col: usize },
    BeginGame,

    //Chat{ message: String },
    Disconnected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerMessage {
    CreateFailed,
    JoinSuccess {

    },
    JoinFailed,
    PlayerConnect{},
    PlayerDisconnect{},
    BeginGame,

    PlayerTurn{ id: usize },
    PlayerTime{ time_left: Vec<i32> },
    
    PlaceToken{ id: usize, col: usize },
    PlaceFailed,

    WonGame{ winner: usize },

    //Chat {}
}




////////////////////////////////////////////////////////////////////////////

//What token the board could place
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Token {
    Empty,
    Red,
    Yellow,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub tokens: Vec<Token>,
}

impl Game {
    // Board width and height. 
    // Easily adjustable and will also change the canvas size accordingly.
    pub const WIDTH: u32 = 7;
    pub const HEIGHT: u32 = 6;

    pub fn check_win( &self, row: u32, col: u32 ) -> bool {       
        // Bits represent each direction's # of connected tokens.
        let mut concur_cnt: [u8; 4] = [1; 4];

        // Each bit represents if the direction should be explored or not.
        // From lelf to right, 0b[NW][N][NE][E]_[SE][S][SW][W]
        let mut concur_stop: u8 = 0b11111111;

        // The token we are checking that someone has won from.
        let cur_tok = self.tokens[ Self::get_index( row, col ) ];

        // Moves a difference of 1 to 3 moves away from each direction.m km njjolk,
        for diff in 1..=3 {
            for dir in 0..8 {
                let mask = 0b1 << dir;

                // If the direction should be explored,
                if concur_stop & mask != 0 { 

                    let mut row_check = row as i32;
                    let mut col_check = col as i32;

                    // Finds the row and column to check.
                    match dir {
                        0..=2 => row_check += diff,
                        4..=6 => row_check -= diff,
                        _ => {},
                    }
                    match dir {
                        0 | 6 | 7 => col_check -= diff,
                        2..=4     => col_check += diff,
                        _ => {},
                    }

                    // If the position is invalid or the token at the position is not what we are checking,
                    // Do not explore this direction any further.
                    if (row_check < 0 || row_check >= Self::HEIGHT as i32)
                            || (col_check < 0 || col_check >= Self::WIDTH as i32) 
                            || self.tokens[ Self::get_index(row_check as u32, col_check as u32) ] != cur_tok 
                    {
                        concur_stop &= !(mask);
                    }
                    // Else, this direction has the token we are checking.
                    else 
                    {
                        concur_cnt[dir % 4] += 1;
                    }
                }
            }

            // Counts how much tokens are in each direction. 
            // If there is at least four tokens in that direction => someone has won the game.
            for count in concur_cnt {
                if count >= 4 {
                    return true;
                }
            }
        }

        false
    }

    pub fn get_index( row: u32, col: u32 ) -> usize {
        (row*Self::WIDTH + col) as usize
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            tokens: vec![Token::Empty; (Self::WIDTH*Self::HEIGHT) as usize],
        }
    }
}