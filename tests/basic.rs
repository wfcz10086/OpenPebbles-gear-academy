use gstd::prelude::*;
use gtest::{Log, Program, System};
use pebbles_game_io::*;

fn create_system_and_user() -> (System, u64) {
    let sys = System::new();
    sys.init_logger();
    let user_id = 1; // 用户ID
    sys.mint_to(user_id, 10_000_000_000_000); // 初始化用户余额，确保有足够的资金
    (sys, user_id)
}

#[test]
fn test_initialization_success() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };

    let res = program.send_bytes(user_id, init_msg.encode());
    println!("{:?}", res);

    let state: GameState = program.read_state(()).expect("Failed to read state");
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 3);
    // 如果程序是第一个玩家，它会进行一个回合操作
    let expected_remaining = if state.first_player == Player::Program {
        10 - 3 // 因为 get_random_u32() 在测试环境中返回 1，程序会取 (1 % 3) + 1 = 2 个石子
    } else {
        10
    };
    assert_eq!(state.pebbles_remaining, expected_remaining);
    assert!(state.first_player == Player::User || state.first_player == Player::Program);
}

#[test]
fn test_turn_order() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(user_id, init_msg.encode());
    let turn_action = PebblesAction::Turn(3);
    let res = program.send_bytes(user_id, turn_action.encode());
    println!("{:?}", res);

    let state: GameState = program.read_state(()).expect("Failed to read state");
    println!("State: {:?}", state);
    assert_eq!(state.first_player, Player::Program);
}

#[test]
fn test_program_win() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 1,
        max_pebbles_per_turn: 1,
    };

    let res = program.send_bytes(user_id, init_msg.encode());
    let state: GameState = program.read_state(()).expect("Failed to read state");
    println!("State: {:?}", state);
    println!("{:?}", res);
    assert_eq!(state.winner, Some(Player::Program));
}

#[test]
fn test_game_restart() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(user_id, init_msg.encode());

    let restart_action = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 20,
        max_pebbles_per_turn: 5,
    };

    program.send_bytes(user_id, restart_action.encode());
    let state: GameState = program.read_state(()).expect("Failed to read state");
    println!("{:?}", state);
    // 如果程序是第一个玩家，它会进行一个回合操作
    let expected_remaining = if state.first_player == Player::Program {
        20 - 2 // 因为 get_random_u32() 在测试环境中返回 1，程序会取 (1 % 5) + 1 = 2 个石子
    } else {
        20
    };
    assert_eq!(state.pebbles_count, 20);
    assert_eq!(state.max_pebbles_per_turn, 5);
    assert_eq!(state.pebbles_remaining, expected_remaining);
}

#[test]
fn test_user_give_up() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(user_id, init_msg.encode());

    let give_up_action = PebblesAction::GiveUp;
    let res = program.send_bytes(user_id, give_up_action.encode());
    let state: GameState = program.read_state(()).expect("Failed to read state");
    println!("{:?}", state);
    println!("{:?}", res);
    assert_eq!(state.winner, Some(Player::Program));
}

#[test]
fn test_user_win() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 2,
        max_pebbles_per_turn: 1,
    };

    program.send_bytes(user_id, init_msg.encode());
    let turn_action = PebblesAction::Turn(1);
    program.send_bytes(user_id, turn_action.encode());
    program.send_bytes(user_id, turn_action.encode());

    let state: GameState = program.read_state(()).expect("Failed to read state");
    println!("{:?}", state);
    assert_eq!(state.winner, Some(Player::User));
}

#[test]
fn test_program_turn_difficulty_easy() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(user_id, init_msg.encode());
    let turn_action = PebblesAction::Turn(3);
    program.send_bytes(user_id, turn_action.encode());

    let state: GameState = program.read_state(()).expect("Failed to read state");
    println!("{:?}", state);
    assert!(state.pebbles_remaining <= 7);
}

#[test]
fn test_program_turn_difficulty_hard() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(user_id, init_msg.encode());
    let turn_action = PebblesAction::Turn(3);
    program.send_bytes(user_id, turn_action.encode());

    let state: GameState = program.read_state(()).expect("Failed to read state");
    println!("{:?}", state);
    assert!(state.pebbles_remaining % 4 == 0);
}

#[test]
fn test_invalid_init_pebbles_count() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 0, // 无效的石子数量
        max_pebbles_per_turn: 3,
    };

    let res = program.send_bytes(user_id, init_msg.encode());
    assert!(res.main_failed());
}

#[test]
fn test_invalid_init_max_pebbles_per_turn() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 0, // 无效的每回合最大石子数量
    };

    let res = program.send_bytes(user_id, init_msg.encode());
    assert!(res.main_failed());
}

#[test]
fn test_invalid_turn_pebbles_count() {
    let (sys, user_id) = create_system_and_user();
    let program = Program::current(&sys);

    let init_msg = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 3,
    };

    program.send_bytes(user_id, init_msg.encode());

    let turn_action = PebblesAction::Turn(0); // 无效的回合
    let res = program.send_bytes(user_id, turn_action.encode());
    assert!(res.main_failed());
}
