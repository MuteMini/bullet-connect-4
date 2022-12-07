use wasm_bindgen::prelude::*;

extern crate web_sys;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudState {
    Main = 0,
    Join = 1,
    Make = 2,
    Wait = 3,
    Hide = 99,
}

#[wasm_bindgen]
pub struct Hud {
    state: HudState,
    hud_div: web_sys::Element
}

#[wasm_bindgen]
impl Hud {
    pub fn new() -> Hud {
        let document = web_sys::window().unwrap().document().unwrap();
        let hud_div = document.get_element_by_id("hud-menu").unwrap();

        Hud {
            state: HudState::Hide,
            hud_div
        }
    }
}