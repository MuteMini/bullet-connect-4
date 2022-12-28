use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::f64;

use common::{ Token };

extern crate js_sys;
extern crate web_sys;

// From https://rustwasm.github.io/docs/book/game-of-life/debugging.html
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Constants for drawing on canvas.
const TOKEN_RADIUS: u32 = 50;
const PADDING: u32 = 15;

// Holds the information about the game
#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,

    client_time: i32,
    server_time: i32,
    elapsed: f64,

    game_won: bool,
    player_token: Token,
    tokens: Vec<Token>,

    canvas_height: u32,
    canvas_width: u32,
    context: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Board {    

    // Draws the game using the canvas context.
    pub fn draw_game( &self ) {

        // Sets the background as blue.
        self.context.begin_path();
        self.context.set_fill_style( &JsValue::from("#0000FF") );
        self.context.rect(0.0, 0.0, self.canvas_width as f64, self.canvas_height as f64);
        self.context.fill();

        // Draws each token.
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index( row, col );

                let tmp;
                self.context.begin_path();
                self.context.set_fill_style({
                    match self.tokens[idx] {
                        Token::Yellow => tmp = JsValue::from("#FFFF00"),
                        Token::Red => tmp = JsValue::from("#FF0000"),
                        _ => tmp = JsValue::from("#FFFFFF"),
                    };
                    &tmp
                });
                
                self.context.arc(
                    (2*col*(TOKEN_RADIUS + PADDING/2) + TOKEN_RADIUS + PADDING/2) as f64,
                    (self.canvas_height - 2*(row + 1)*(TOKEN_RADIUS + PADDING/2) + TOKEN_RADIUS + PADDING/2) as f64,
                    TOKEN_RADIUS as f64, 
                    0.0, f64::consts::PI*2.0
                ).unwrap();

                self.context.fill();
            }
        }

        // Draws a temporary winning screen.
        if self.game_won {
            self.context.set_font("48px serif");
            self.context.set_fill_style( &JsValue::from("#FF00FF") );
            self.context.fill_text(
                        "SOMEONE WON OMG", 
                        (self.canvas_width/2 - 200) as f64, 
                        (self.canvas_height/2) as f64
            ).unwrap();
        }
    }

    pub fn take_input( &mut self, canvas_left: u32 ) {

        // Calculates what column the player clicked on.
        let col = std::cmp::min( ((canvas_left/ (TOKEN_RADIUS*2 + PADDING)) as f64).floor() as u32, self.width - 1 );

        // Loops through the rows from bottom to top to find if a token can be placed.
        for row in 0..self.height {
            let idx = self.get_index( row, col );

            // If it can be placed, place the token and switch players.
            if self.tokens[idx] == Token::Empty {
                self.tokens[idx] = self.player_token;
                self.player_token = if self.player_token == Token::Yellow { Token::Red } else { Token::Yellow };

                // Check if the move was a winning move.
                self.game_won = self.check_win( row, col );

                // Leaves the for loop after placing the token
                break;  
            }
        }
    }

    pub fn tick_time( &mut self ) {

        // Get the amount of time elapsed.
        let time = js_sys::Date::now() - self.elapsed;
        self.elapsed = js_sys::Date::now();

        // Make sure some weird error isn't occuring.
        if time < 0.0 {
            log!("error: time elapsed is 0");
            return;
        }

        // Find what player's time needs to be manipulated
        let player_time = match self.player_token {
            Token::Yellow => {
                &mut self.client_time
            },
            Token::Red => {
                &mut self.server_time
            },
            _ => { 
                log!("error: trying to time empty placement");
                return;
            },
        };
        
        // Decrement the player's time
        *player_time -= time as i32;

        // If the player's time is negative, set time to zero and game has finished.
        if *player_time < 0 {
            *player_time = 0;
            self.game_won = true
        }
    }

    fn check_win( &self, row: u32, col: u32 ) -> bool {      
         
        // Bits represent each direction's # of connected tokens.
        let mut concur_cnt: [u8; 4] = [1; 4];

        // Each bit represents if the direction should be explored or not.
        // From lelf to right, 0b[NW][N][NE][E]_[SE][S][SW][W]
        let mut concur_stop: u8 = 0b11111111;

        // The token we are checking that someone has won from.
        let cur_tok = self.tokens[ self.get_index( row, col ) ];

        //log!("Row: {} Col: {}", row, col);

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
                    if (row_check < 0 || row_check >= self.height as i32)
                            || (col_check < 0 || col_check >= self.width as i32) 
                            || self.tokens[ self.get_index(row_check as u32, col_check as u32) ] != cur_tok 
                    {
                        concur_stop &= !(mask);
                    }
                    // Else, this direction has the token we are checking.
                    else 
                    {
                        concur_cnt[dir % 4] += 1;
                    }

                    // log!("Diff: {}, Dir: {}, Cnt: {}", diff, dir, concur_cnt[dir % 4]);
                    // log!("row_check: {}, col_check: {}, token: {:?}", row_check, col_check, debug_token); 
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

    fn get_index( &self, row: u32, col: u32 ) -> usize {
        (row*self.width + col) as usize
    }

    pub fn new() -> Board {
        // Board width and height. 
        // Easily adjustable and will also change the canvas size accordingly.
        let width: u32 = 7;
        let height: u32 = 6;
        
        // Initializes a token vector to be all empty.
        let tokens = vec![Token::Empty; (width*height) as usize];

        // Grabs the canvas element from the document.
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("connect-4-canvas").unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

        // Sets the canvas width and height using the width and height given.
        let canvas_width = ((TOKEN_RADIUS*2 + PADDING)*width + 1) as u32;
        let canvas_height = ((TOKEN_RADIUS*2 + PADDING)*height + 1) as u32;
        canvas.set_width( canvas_width );
        canvas.set_height( canvas_height );
    
        // Stores the canvas context for rendering.
        let context: web_sys::CanvasRenderingContext2d = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Board {
            width,
            height,
            client_time: 10000,
            server_time: 10000,
            elapsed: js_sys::Date::now(),
            game_won: false,
            player_token: Token::Yellow,
            tokens,
            canvas_width,
            canvas_height,
            context,
        }
    }

    pub fn client_time( &self ) -> i32 {
        self.client_time
    }

    pub fn server_time( &self ) -> i32 {
        self.server_time
    }

    pub fn game_won( &self ) -> bool {
        self.game_won
    }
}