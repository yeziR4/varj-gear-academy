use gstd::{Encode, Decode, msg, exec}; // Import necessary functions
use scale_info::TypeInfo; // Import TypeInfo derive macro (assuming you're using scale-info)

// Define the GameState struct
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GameState {
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
    pub pebbles_remaining: u32,
    pub difficulty: DifficultyLevel,
    pub first_player: Player,
    pub winner: Option<Player>,
}

// Define the PebblesInit struct
#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
    pub difficulty: DifficultyLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode, TypeInfo)]
pub enum DifficultyLevel {
    Easy,
    Hard,
}

// Implementing Default for DifficultyLevel
impl Default for DifficultyLevel {
    fn default() -> Self {
        DifficultyLevel::Easy // Set Easy as the default difficulty level
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Encode, Decode, TypeInfo)]
pub enum Player {
    User,
    Program,
}

impl Default for Player {
    fn default() -> Self {
        Player::User // Specify the default variant
    }
}

// Function to get a random 32-bit number (provided in the assignment)
fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

// Function to choose the first player (replace with your preferred method)
fn init_first_player() -> Player {
    match get_random_u32() % 2 {
        0 => Player::User,
        _ => Player::Program,
    }
}

// Implementation of init() function
fn init() -> GameState {
    let pebbles_init: PebblesInit = msg::load().expect("Failed to load PebblesInit");

    // Validate input data (add checks as needed)
    if pebbles_init.pebbles_count <= 0 || pebbles_init.max_pebbles_per_turn <= 0 {
        panic!("Invalid input data: pebbles_count and max_pebbles_per_turn must be positive");
    }

    let first_player = init_first_player();

    let mut game_state = GameState {
        pebbles_count: pebbles_init.pebbles_count,
        max_pebbles_per_turn: pebbles_init.max_pebbles_per_turn,
        pebbles_remaining: pebbles_init.pebbles_count,
        difficulty: pebbles_init.difficulty,
        first_player,
        winner: None,
    };

    if first_player == Player::Program {
        let program_turn = get_pebbles_to_remove(&mut game_state);
        game_state.pebbles_remaining -= program_turn;
        // Implement your event handling logic here
    }

    game_state
}

// Function to get pebbles to remove for program's turn (replace with your strategy)
fn get_pebbles_to_remove(game_state: &mut GameState) -> u32 {
    match game_state.difficulty {
        DifficultyLevel::Easy => get_random_u32() % (game_state.max_pebbles_per_turn + 1),
        DifficultyLevel::Hard => {
            // Implement your winning strategy logic here
            // This is a placeholder, replace with your algorithm to win the game
            panic!("Hard difficulty strategy not implemented yet");
        }
    }
}

fn main() {
    // Call init() function to initialize the game state
    let _ = init();
}
