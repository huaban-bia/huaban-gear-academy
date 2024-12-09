use gtest::{Program, System};
use pebbles_game_io::*;

#[test]
fn basic_test() {
    let sys = System::new();
    sys.init_logger();

    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 3,
    };
    let res = program.send(1, init_msg);
    assert!(res.main_failed().is_none(), "Initialization failed");

    let state: pebbles_game_io::GameState = program.read_state(()).expect("Unable to read state");
    assert_eq!(state.pebbles_remaining, 15, "Initial pebbles count mismatch");

    let user_action = PebblesAction::Turn(2);
    let res = program.send(1, user_action);
    assert!(res.main_failed().is_none(), "User action failed");

    let state: pebbles_game_io::GameState = program.read_state().expect("Unable to read state");
    assert!(state.pebbles_remaining < 15, "Pebbles count did not decrease");

    let give_up_action = PebblesAction::GiveUp;
    let res = program.send(1, give_up_action);
    assert!(res.main_failed().is_none(), "Give up action failed");

    let state: pebbles_game_io::GameState = program.read_state().expect("Unable to read state");
    assert_eq!(state.winner, Some(Player::Program), "Program should be the winner");

    let restart_action = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 20,
        max_pebbles_per_turn: 4,
    };
    let res = program.send(1, restart_action);
    assert!(res.main_failed().is_none(), "Restart action failed");

    let state: pebbles_game_io::GameState = program.read_state().expect("Unable to read state");
    assert_eq!(state.pebbles_remaining, 20, "Pebbles count after restart mismatch");
    assert_eq!(state.max_pebbles_per_turn, 4, "Max pebbles per turn after restart mismatch");
    assert_eq!(state.winner, None, "Winner should be None after restart");
}
