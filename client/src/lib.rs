use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::f64;

use bullet_common::{ Token, Game };

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
    game: Game,

    client_time: i32,
    server_time: i32,
    elapsed: f64,

    game_won: bool,
    player_token: Token,

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
        for row in 0..Game::HEIGHT {
            for col in 0..Game::WIDTH {
                let idx = Game::get_index( row, col );

                let tmp;
                self.context.begin_path();
                self.context.set_fill_style({
                    match self.game.tokens[idx] {
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
        let col = std::cmp::min( ((canvas_left/ (TOKEN_RADIUS*2 + PADDING)) as f64).floor() as u32, Game::WIDTH - 1 );

        // Loops through the rows from bottom to top to find if a token can be placed.
        for row in 0..Game::HEIGHT {
            let idx = Game::get_index( row, col );

            // If it can be placed, place the token and switch players.
            if self.game.tokens[idx] == Token::Empty {
                self.game.tokens[idx] = self.player_token;
                self.player_token = if self.player_token == Token::Yellow { Token::Red } else { Token::Yellow };

                // Check if the move was a winning move.
                self.game_won = self.game.check_win( row, col );

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

    pub fn new() -> Board {        
        // Grabs the canvas element from the document.
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("connect-4-canvas").unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

        // Sets the canvas width and height using the width and height given.
        let canvas_width = ((TOKEN_RADIUS*2 + PADDING)*Game::WIDTH + 1) as u32;
        let canvas_height = ((TOKEN_RADIUS*2 + PADDING)*Game::HEIGHT + 1) as u32;
        canvas.set_width( canvas_width );
        canvas.set_height( canvas_height );
    
        // Stores the canvas context for rendering.
        let context: web_sys::CanvasRenderingContext2d = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Board {
            game: Game::default(),
            client_time: 10000,
            server_time: 10000,
            elapsed: js_sys::Date::now(),
            game_won: false,
            player_token: Token::Yellow,
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