use wasm_bindgen::prelude::*;

//From https://rustwasm.github.io/docs/book/game-of-life/debugging.html
extern crate web_sys;
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    Empty = 0,
    Red = 1,
    Yellow = 2,
}

#[wasm_bindgen]
pub struct Board {
    width: u16,
    height: u16,
    tokens: Vec<Token>,
}

#[wasm_bindgen]
impl Board {
    pub fn take_input( &mut self, col: u16, token: Token ) -> u16 {
        for row in 0..self.height {
            let idx = self.get_index( row, col );
            if self.tokens[idx] == Token::Empty {
                self.tokens[idx] = token;
                return row;
            }
        }
        self.height
    }

    pub fn check_win( &self, row: u16, col: u16 ) -> bool {      
        //Bits represent each dir's # of connected tokens
        let mut concur_cnt: [u8; 4] = [1; 4];

        //Each bit represents if the direction should be explored or not.
        //From lelf to right, 0b[NW][N][NE][E]_[SE][S][SW][W]
        let mut concur_stop: u8 = 0b11111111;

        let cur_tok = self.tokens[ self.get_index( row, col ) ];

        log!("Row: {} Col: {}", row, col);

        for diff in 1..=3 {
            for dir in 0..8 {
                let mask = 0b1 << dir;
                if concur_stop & mask != 0 { 
                    let mut row_check = row as i16;
                    let mut col_check = col as i16;

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

                    if (row_check < 0 || row_check >= self.height as i16)
                        || (col_check < 0 || col_check >= self.width as i16) 
                        || self.tokens[ self.get_index(row_check as u16, col_check as u16) ] != cur_tok {
                        concur_stop &= !(mask);
                    }
                    else {
                        concur_cnt[dir % 4] += 1;
                    }

                    // log!("Diff: {}, Dir: {}, Cnt: {}", diff, dir, concur_cnt[dir % 4]);
                    // log!("row_check: {}, col_check: {}, token: {:?}", row_check, col_check, debug_token); 
                }
            }

            for count in concur_cnt {
                if count >= 4 {
                    return true;
                }
            }
        }

        false
    }

    fn get_index ( &self, row: u16, col: u16 ) -> usize {
        (row*self.width + col) as usize
    }

    pub fn new() -> Board {
        let width: u16 = 7;
        let height: u16 = 6;
        
        let tokens = vec![Token::Empty; (width*height) as usize];

        // let tokens = (0..width*height)
        //     .map(|i| {
        //         if i % 3 == 0 {
        //             Token::Empty
        //         }
        //         else if i % 3 == 1 {
        //             Token::Red
        //         }
        //         else {
        //             Token::Yellow
        //         }
        //     })
        //     .collect();
        
        Board {
            width,
            height,
            tokens,
        }
    }

    pub fn width( &self ) -> u16 {
        self.width
    }

    pub fn height( &self ) -> u16 {
        self.height
    }

    pub fn token( &self ) -> *const Token {
        self.tokens.as_ptr()
    }
}