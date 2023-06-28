// imports
use std::io;
use std::time::Duration;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Module 'PawsForLove'
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Handle events when the user interacts with the application
fn handle_events(event: Event, game: &mut GameState) {
    match event {
        Event::Run => {
            println!("Paws for Love is running!");
        }
        Event::Quit => {
            println!("Goodbye!");
            game.quit = true;
        }
        Event::Input(input) => {
            if input == "exit" {
                game.quit = true;
            } else {
                println!("You said: {}", input);
            }
        }
        _ => (),
    }
}

// Main game loop
fn main_loop(game: &mut GameState) {
    while !game.quit {
        // Get user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Handle user input
        match input.trim() {
            "run" => handle_events(Event::Run, game),
            "exit" => handle_events(Event::Quit, game),
            _ => handle_events(Event::Input(input.trim().to_string()), game),
        }

        // Sleep for 30 milliseconds
        std::thread::sleep(Duration::from_millis(30));
    }
}

// Initialize game state
fn init_game() -> GameState {
    GameState { quit: false }
}

// Main entry point of the application
fn main() {
    // Initialize game state
    let mut game = init_game();

    // Start game loop
    main_loop(&mut game);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Enums
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Event type
enum Event {
    Run,
    Quit,
    Input(String),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Structs
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// GameState
#[derive(PartialEq)]
struct GameState {
    quit: bool,
}