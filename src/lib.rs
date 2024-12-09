#![no_std]
use gstd::{msg, prelude::*, exec};
use pebbles_game_io::*;

static mut PEEBLES_GAME: Option<GameState> = None;

#[no_mangle]
pub extern "C" fn init() {
  
}

#[no_mangle]
pub extern "C" fn handle() {
 
}

#[no_mangle]
pub extern "C" fn state() {

}

