//! # Tic Tac Toe Game
//!
//! A professional implementation of the classic Tic Tac Toe game for two players.
//! This game demonstrates fundamental Rust concepts including ownership, borrowing,
//! pattern matching, and error handling.
//!
//! ## Features
//! - Two-player gameplay (X and O)
//! - Input validation and error handling
//! - Win condition detection
//! - Draw detection
//! - Clean, formatted console output
//!
//! ## Usage
//! ```bash
//! cargo run
//! ```

use std::io::{self, Write};
use std::fmt;

// ============================================================================
// CONSTANTS
// ============================================================================
// In Rust, constants are immutable values that are inlined at compile time.
// They must have explicit types and can only be set to constant expressions.
// Convention: Use SCREAMING_SNAKE_CASE for constant names.

/// Represents Player X's marker on the board
const PLAYER_X: char = 'X';

/// Represents Player O's marker on the board
const PLAYER_O: char = 'O';

/// Defines the dimensions of the game board (3x3)
const BOARD_SIZE: usize = 3;

/// Represents an empty cell on the board
const EMPTY_CELL: char = ' ';

// ============================================================================
// TYPE ALIASES
// ============================================================================
// Type aliases create a new name for an existing type, improving code readability.
// They don't create a new type, just an alternative name.

/// Type alias for the game board structure
/// 
/// Represents a 3x3 grid as a 2D array: `[[char; 3]; 3]`
/// - Outer array: rows
/// - Inner array: columns
/// Arrays in Rust are stack-allocated with fixed size known at compile time.
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

// ============================================================================
// ENUMS
// ============================================================================
// Enums in Rust are algebraic data types that can hold different variants.
// They are memory-efficient and enable exhaustive pattern matching.

/// Represents the possible outcomes of the game
/// 
/// Rust enums can carry data in their variants, making them more powerful
/// than C-style enums. This is known as an "algebraic data type".
#[derive(Debug, PartialEq, Clone, Copy)]
enum GameState {
    /// The game is still in progress
    InProgress,
    /// A player has won (stores the winning player's character)
    Won(char),
    /// The game ended in a draw
    Draw,
}

// ============================================================================
// IMPLEMENTATIONS
// ============================================================================

/// Implements the Display trait for GameState
/// 
/// The Display trait is part of Rust's formatting infrastructure.
/// Implementing it allows custom types to be formatted with `{}` in print macros.
/// This is similar to implementing toString() in other languages.
impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Pattern matching with match expressions is exhaustive in Rust,
        // meaning all possible variants must be handled.
        match self {
            GameState::InProgress => write!(f, "Game in progress"),
            GameState::Won(player) => write!(f, "Player {} wins!", player),
            GameState::Draw => write!(f, "It's a draw!"),
        }
    }
}

// ============================================================================
// BOARD MANAGEMENT FUNCTIONS
// ============================================================================

/// Initializes and returns a new empty game board
/// 
/// # Returns
/// A 3x3 board filled with empty cells
/// 
/// # Rust Concepts
/// - Array initialization: `[value; size]` creates an array with repeated values
/// - Stack allocation: Arrays are allocated on the stack, making them very fast
/// - Copy semantics: `char` implements Copy trait, so it can be duplicated cheaply
fn initialize_board() -> Board {
    [[EMPTY_CELL; BOARD_SIZE]; BOARD_SIZE]
}

/// Prints the current state of the game board with formatting
/// 
/// # Arguments
/// * `board` - Immutable reference to the game board
/// 
/// # Rust Concepts
/// - Borrowing: `&Board` borrows the board immutably (read-only access)
/// - References allow functions to use data without taking ownership
/// - This prevents unnecessary copies and allows the caller to keep using the board
fn print_board(board: &Board) {
    println!("\n  0   1   2");
    for (row_idx, row) in board.iter().enumerate() {
        // `enumerate()` is an iterator adapter that yields (index, value) pairs
        print!("{} ", row_idx);
        for cell in row {
            print!("{}│", cell);
        }
        println!();
        if row_idx < BOARD_SIZE - 1 {
            println!("  ─────────");
        }
    }
    println!();
}

/// Checks if a specific cell on the board is empty
/// 
/// # Arguments
/// * `board` - Reference to the game board
/// * `row` - Row index
/// * `col` - Column index
/// 
/// # Returns
/// `true` if the cell is empty, `false` otherwise
/// 
/// # Rust Concepts
/// - Boolean return type with explicit true/false values
/// - Direct array indexing with bounds checking at runtime
fn is_cell_empty(board: &Board, row: usize, col: usize) -> bool {
    board[row][col] == EMPTY_CELL
}

/// Checks if all cells on the board are filled
/// 
/// # Arguments
/// * `board` - Reference to the game board
/// 
/// # Returns
/// `true` if the board is full, `false` otherwise
/// 
/// # Rust Concepts
/// - Iterator methods: `iter()` creates an iterator over the board
/// - `flatten()` converts nested iterators into a single iterator
/// - `all()` is a higher-order function that checks if all elements satisfy a predicate
/// - Closures: `|&cell|` is an anonymous function capturing the cell reference
fn is_board_full(board: &Board) -> bool {
    board.iter().flatten().all(|&cell| cell != EMPTY_CELL)
}

// ============================================================================
// GAME LOGIC FUNCTIONS
// ============================================================================

/// Checks if the specified player has won the game
/// 
/// # Arguments
/// * `board` - Reference to the game board
/// * `player` - The player character to check for ('X' or 'O')
/// 
/// # Returns
/// `true` if the player has three in a row, `false` otherwise
/// 
/// # Rust Concepts
/// - Range syntax: `0..BOARD_SIZE` creates a range from 0 to BOARD_SIZE-1
/// - Iterator methods: `any()` returns true if any element satisfies the condition
/// - Short-circuit evaluation: Logical OR (||) stops at the first true value
fn check_winner(board: &Board, player: char) -> bool {
    // Check rows: All cells in any row match the player
    let row_win = (0..BOARD_SIZE).any(|i| {
        (0..BOARD_SIZE).all(|j| board[i][j] == player)
    });

    // Check columns: All cells in any column match the player
    let col_win = (0..BOARD_SIZE).any(|j| {
        (0..BOARD_SIZE).all(|i| board[i][j] == player)
    });

    // Check main diagonal (top-left to bottom-right)
    let diag1_win = (0..BOARD_SIZE).all(|i| board[i][i] == player);

    // Check anti-diagonal (top-right to bottom-left)
    let diag2_win = (0..BOARD_SIZE).all(|i| board[i][BOARD_SIZE - 1 - i] == player);

    // Return true if any winning condition is met
    row_win || col_win || diag1_win || diag2_win
}

/// Determines the current state of the game
/// 
/// # Arguments
/// * `board` - Reference to the game board
/// * `last_player` - The player who made the most recent move
/// 
/// # Returns
/// `GameState` enum representing the current game status
/// 
/// # Rust Concepts
/// - Early returns: Using `return` to exit the function immediately
/// - Enum variants: Constructing GameState variants with associated data
fn evaluate_game_state(board: &Board, last_player: char) -> GameState {
    if check_winner(board, last_player) {
        return GameState::Won(last_player);
    }
    
    if is_board_full(board) {
        return GameState::Draw;
    }
    
    GameState::InProgress
}

// ============================================================================
// INPUT/OUTPUT FUNCTIONS
// ============================================================================

/// Prompts the current player for their move and validates input
/// 
/// # Arguments
/// * `current_player` - The character representing the current player
/// * `board` - Reference to the game board for validation
/// 
/// # Returns
/// A tuple `(row, col)` representing valid board coordinates
/// 
/// # Rust Concepts
/// - Infinite loops: `loop` creates an unconditional loop (broken explicitly)
/// - Mutable variables: `mut` keyword allows reassignment
/// - String type: Heap-allocated, growable UTF-8 string
/// - Error handling: `expect()` unwraps Result, panicking with a message on error
/// - Method chaining: Calling multiple methods in sequence
/// - Tuples: Fixed-size collection of values of different types
fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        // Create a mutable String buffer for user input
        // Strings in Rust are UTF-8 encoded and heap-allocated
        let mut input = String::new();
        
        // Print prompt and flush stdout to ensure it displays before input
        // `print!` doesn't auto-flush like `println!` does
        print!("Player {}, enter your move (row column): ", current_player);
        io::stdout().flush().expect("Failed to flush stdout");
        
        // Read a line of input from stdin
        // `read_line()` returns a Result<usize, Error> which must be handled
        // `expect()` unwraps the Result, panicking if it's an Err variant
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        // Parse input using iterator adapters and functional programming
        let coordinates: Vec<usize> = input
            .split_whitespace()              // Split by any whitespace
            .filter_map(|s| s.parse().ok())  // Parse each token, filtering out errors
            .collect();                       // Collect into a Vec<usize>
        
        // Validate that we have exactly two coordinates
        if coordinates.len() != 2 {
            println!("❌ Invalid input. Please enter two numbers separated by a space.");
            continue; // Skip to next iteration of the loop
        }
        
        // Destructure the vector into individual variables
        let (row, col) = (coordinates[0], coordinates[1]);
        
        // Validate coordinates are within bounds
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            println!("❌ Coordinates out of range. Use 0-{} for both row and column.", BOARD_SIZE - 1);
            continue;
        }
        
        // Validate the cell is empty
        if !is_cell_empty(board, row, col) {
            println!("❌ That cell is already occupied. Choose another.");
            continue;
        }
        
        // All validations passed, return the valid coordinates
        return (row, col);
    }
}

// ============================================================================
// GAME FLOW FUNCTIONS
// ============================================================================

/// Main game loop that manages the flow of the game
/// 
/// # Rust Concepts
/// - Mutable state: `mut` allows variables to be modified
/// - Ownership: The function owns the board and can modify it
/// - Control flow: Using loop, if, and match for game logic
/// - Breaking loops: Using `break` to exit the loop explicitly
fn play_game() {
    // Initialize a mutable board (ownership transfers to this function)
    let mut board = initialize_board();
    
    // Start with Player X
    let mut current_player = PLAYER_X;
    
    println!("\n🎮 Game started! Player X goes first.");
    println!("📝 Enter moves as: row column (e.g., '1 2' for middle-right)");
    
    // Main game loop
    loop {
        // Display the current board state
        print_board(&board);
        
        // Get and validate the player's move
        let (row, col) = get_player_move(current_player, &board);
        
        // Place the player's marker on the board
        // This is a mutable operation, modifying the board in place
        board[row][col] = current_player;
        
        // Evaluate the game state after the move
        let game_state = evaluate_game_state(&board, current_player);
        
        // Pattern matching on the game state
        // This is exhaustive: all possible GameState variants must be handled
        match game_state {
            GameState::Won(player) => {
                print_board(&board);
                println!("🎉 {}", GameState::Won(player));
                break; // Exit the game loop
            }
            GameState::Draw => {
                print_board(&board);
                println!("🤝 {}", GameState::Draw);
                break; // Exit the game loop
            }
            GameState::InProgress => {
                // Switch to the other player using a ternary-like expression
                // This demonstrates Rust's expression-based nature
                current_player = if current_player == PLAYER_X {
                    PLAYER_O
                } else {
                    PLAYER_X
                };
            }
        }
    }
    
    println!("\n👋 Thanks for playing!");
}

// ============================================================================
// ENTRY POINT
// ============================================================================

/// Program entry point
/// 
/// # Rust Concepts
/// - `main()` is the entry point for all Rust executables
/// - It has no parameters and returns `()` (unit type, similar to void)
/// - The function signature `fn main()` is special and recognized by the compiler
fn main() {
    println!("╔════════════════════════════════╗");
    println!("║   Welcome to Tic Tac Toe!    ║");
    println!("╔════════════════════════════════╗");
    
    play_game();
}