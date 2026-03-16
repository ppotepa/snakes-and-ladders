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

fn validate_player_name(input: &str) -> Result<String, &'static str> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err("Name cannot be empty.");
    }
    if trimmed.eq_ignore_ascii_case("snake") {
        return Err("Name 'snake' is not allowed.");
    }
    Ok(trimmed.to_string())
}

fn prompt_player_name() -> String {
    loop {
        print!("Enter player name: ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read player name");

        match validate_player_name(&input) {
            Ok(name) => return name,
            Err(message) => println!("{message} Please try again.\n"),
        }
    }
}

fn main() {
    let mut game = Game::new(Board::standard());
    let player_name = prompt_player_name();

    println!("Snakes and Ladders (Rust boilerplate)");
    println!("Welcome, {player_name}!");
    println!("Press ENTER to roll, or type 'q' then ENTER to quit.\n");

    loop {
        println!(
            "{player_name} position: {} / {}",
            game.position(),
            game.board_size()
        );
        println!(
            "Progress: {}",
            progress_bar(game.position(), game.board_size(), 40)
        );

        if game.is_won() {
            println!("\n{player_name} wins! Reached {}.", game.position());
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
        println!("{player_name} is now at: {}\n", turn.to);
    }
}

#[cfg(test)]
mod tests {
    use super::validate_player_name;

    #[test]
    fn accepts_regular_name() {
        let name = validate_player_name("Alice").expect("valid name should pass");
        assert_eq!(name, "Alice");
    }

    #[test]
    fn trims_whitespace() {
        let name = validate_player_name("  Bob  ").expect("valid name should pass");
        assert_eq!(name, "Bob");
    }

    #[test]
    fn rejects_empty_name() {
        let error = validate_player_name("   ").expect_err("empty name should fail");
        assert_eq!(error, "Name cannot be empty.");
    }

    #[test]
    fn rejects_snake_name_case_insensitive() {
        let error = validate_player_name("SnAkE").expect_err("snake should fail");
        assert_eq!(error, "Name 'snake' is not allowed.");
    }
}
