use crate::board::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardEvent {
    Snake { from: u32, to: u32 },
    Ladder { from: u32, to: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TurnResult {
    pub roll: u32,
    pub from: u32,
    pub to: u32,
    pub event: Option<BoardEvent>,
}

pub struct Game {
    board: Board,
    position: u32,
}

impl Game {
    pub fn new(board: Board) -> Self {
        Self { board, position: 1 }
    }

    pub fn position(&self) -> u32 {
        self.position
    }

    pub fn board_size(&self) -> u32 {
        self.board.size()
    }

    pub fn is_won(&self) -> bool {
        self.position == self.board.size()
    }

    pub fn take_turn(&mut self, roll: u32) -> Result<TurnResult, String> {
        if !(1..=6).contains(&roll) {
            return Err(format!("invalid roll value: {roll}; expected 1..=6"));
        }

        let start = self.position;
        let mut next = start + roll;
        let board_size = self.board.size();

        if next > board_size {
            next = board_size - (next - board_size);
        }

        let event = match self.board.transition_at(next) {
            Some(destination) if destination < next => {
                let from = next;
                next = destination;
                Some(BoardEvent::Snake {
                    from,
                    to: destination,
                })
            }
            Some(destination) if destination > next => {
                let from = next;
                next = destination;
                Some(BoardEvent::Ladder {
                    from,
                    to: destination,
                })
            }
            _ => None,
        };

        self.position = next;

        Ok(TurnResult {
            roll,
            from: start,
            to: self.position,
            event,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{BoardEvent, Game};
    use crate::board::Board;

    #[test]
    fn ladder_transition_moves_player_up() {
        let board = Board::with_transitions(100, &[(3, 22)]);
        let mut game = Game::new(board);

        let turn = game.take_turn(2).expect("roll should be valid");

        assert_eq!(turn.from, 1);
        assert_eq!(turn.to, 22);
        assert_eq!(turn.event, Some(BoardEvent::Ladder { from: 3, to: 22 }));
        assert_eq!(game.position(), 22);
    }

    #[test]
    fn snake_transition_moves_player_down() {
        let board = Board::with_transitions(100, &[(7, 2)]);
        let mut game = Game::new(board);

        let turn = game.take_turn(6).expect("roll should be valid");

        assert_eq!(turn.from, 1);
        assert_eq!(turn.to, 2);
        assert_eq!(turn.event, Some(BoardEvent::Snake { from: 7, to: 2 }));
        assert_eq!(game.position(), 2);
    }

    #[test]
    fn player_bounces_back_when_overshooting() {
        let board = Board::with_transitions(10, &[]);
        let mut game = Game::new(board);

        game.take_turn(6).expect("roll should be valid");
        let turn = game.take_turn(5).expect("roll should be valid");

        assert_eq!(turn.from, 7);
        assert_eq!(turn.to, 8);
        assert_eq!(game.position(), 8);
    }
}
