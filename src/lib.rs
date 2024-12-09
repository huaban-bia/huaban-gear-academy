#![no_std]
use gstd::{msg, prelude::*, exec};
use pebbles_game_io::*;

static mut PEEBLES_GAME: Option<GameState> = None;

#[no_mangle]
pub extern "C" fn init() {
    let init_data: PebblesInit = msg::load().expect("Unable to load init data");
    
    assert!(init_data.pebbles_count > 0, "Pebbles count must be greater than 0");
    assert!(init_data.max_pebbles_per_turn > 0, "Max pebbles per turn must be greater than 0");
    
    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };
    
    let game_state = GameState {
        pebbles_count: init_data.pebbles_count,
        max_pebbles_per_turn: init_data.max_pebbles_per_turn,
        pebbles_remaining: init_data.pebbles_count,
        difficulty: init_data.difficulty,
        first_player: first_player.clone(),
        winner: None,
    };
    
    unsafe {
        PEEBLES_GAME = Some(game_state);
    }
    
    if first_player == Player::Program {
        program_turn();
    }
}

#[no_mangle]
pub extern "C" fn handle() {
    let action: PebblesAction = msg::load().expect("Unable to load action");
    
    unsafe {
        if let Some(ref mut game_state) = PEEBLES_GAME {
            match action {
                PebblesAction::Turn(pebbles) => {
                    assert!(pebbles > 0 && pebbles <= game_state.max_pebbles_per_turn, "Invalid number of pebbles");
                    assert!(pebbles <= game_state.pebbles_remaining, "Not enough pebbles remaining");
                    
                    game_state.pebbles_remaining -= pebbles;
                    if game_state.pebbles_remaining == 0 {
                        game_state.winner = Some(Player::User);
                        msg::reply(PebblesEvent::Won(Player::User), 0).expect("Unable to send reply");
                        return;
                    }
                    
                    program_turn();
                }
                PebblesAction::GiveUp => {
                    game_state.winner = Some(Player::Program);
                    msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to send reply");
                }
                PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
                    game_state.pebbles_count = pebbles_count;
                    game_state.max_pebbles_per_turn = max_pebbles_per_turn;
                    game_state.pebbles_remaining = pebbles_count;
                    game_state.difficulty = difficulty;
                    game_state.winner = None;
                    
                    game_state.first_player = if get_random_u32() % 2 == 0 {
                        Player::User
                    } else {
                        Player::Program
                    };
                    
                    if game_state.first_player == Player::Program {
                        program_turn();
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn state() {
    unsafe {
        if let Some(ref game_state) = PEEBLES_GAME {
            msg::reply(game_state, 0).expect("Unable to send reply");
        }
    }
}

fn program_turn() {
    unsafe {
        if let Some(ref mut game_state) = PEEBLES_GAME {
            let pebbles_to_remove = match game_state.difficulty {
                DifficultyLevel::Easy => (get_random_u32() % game_state.max_pebbles_per_turn as u32 + 1) as u32,
                DifficultyLevel::Hard => find_best_move(game_state.pebbles_remaining, game_state.max_pebbles_per_turn),
            };
            
            game_state.pebbles_remaining -= pebbles_to_remove;
            msg::reply(PebblesEvent::CounterTurn(pebbles_to_remove), 0).expect("Unable to send reply");
            
            if game_state.pebbles_remaining == 0 {
                game_state.winner = Some(Player::Program);
                msg::reply(PebblesEvent::Won(Player::Program), 0).expect("Unable to send reply");
            }
        }
    }
}

fn find_best_move(pebbles_remaining: u32, max_pebbles_per_turn: u32) -> u32 {
    for i in 1..=max_pebbles_per_turn {
        if pebbles_remaining <= i {
            return pebbles_remaining;
        }
    }
    (get_random_u32() % max_pebbles_per_turn as u32 + 1) as u32
}

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}
