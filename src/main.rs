mod board;
mod game;

use std::io::{self, Write};

use rand::Rng;

use crate::board::Board;
use crate::game::Game;

fn roll_dice() -> u32 {
    rand::thread_rng().gen_range(1..=6)
}

fn progress_bar(position: u32, size: u32, width: usize) -> String {
    let filled = ((position as f64 / size as f64) * width as f64).round() as usize;
    let filled = filled.min(width);
    let empty = width.saturating_sub(filled);
    format!(
        "[{}{}] {}%",
        "=".repeat(filled),
        " ".repeat(empty),
        (position * 100) / size
    )
}

fn main() {
    let mut game = Game::new(Board::standard());

    println!("Snakes and Ladders (Rust boilerplate)");
    println!("Press ENTER to roll, or type 'q' then ENTER to quit.\n");

    loop {
        println!("Position: {} / {}", game.position(), game.board_size());
        println!(
            "Progress: {}",
            progress_bar(game.position(), game.board_size(), 40)
        );

        if game.is_won() {
            println!("\nYou win! Reached {}.", game.position());
            break;
        }

        print!("\nRoll dice? ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read user input");

        if input.trim().eq_ignore_ascii_case("q") {
            println!("Quitting game.");
            break;
        }

        let roll = roll_dice();
        let turn = game
            .take_turn(roll)
            .expect("dice roll should always be between 1 and 6");

        println!("\nRolled: {}", turn.roll);
        if let Some(event) = turn.event {
            match event {
                game::BoardEvent::Snake { from, to } => {
                    println!("Snake at {}! Sliding down to {}.", from, to);
                }
                game::BoardEvent::Ladder { from, to } => {
                    println!("Ladder at {}! Climbing up to {}.", from, to);
                }
            }
        }
        println!("Now at: {}\n", turn.to);
    }
}
