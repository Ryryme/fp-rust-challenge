
// A struct implementing TicTacToe represents a game of TicTacToe.
// This game may be ongoing, or it may have been finished.
trait TicTacToe: std::fmt::Display + Clone {
  type Player;  // Some type indicating something like either the X player, O player, or empty/tie
  type Error;   // Some type indicating potential errors that may be returned from functions

  // Static method creating a new instance of a struct implementing TicTacToe
  // At the start of the game, it should be the X player's turn.
  fn new() -> Self;

  // This method makes a move in the TicTacToe game.
  fn make_move(&mut self, row: u32, column: u32) -> Result<(), Self::Error>;

  // This method returns the player whose turn it currently is.
  fn current_player(&self) -> Self::Player;

  // This method returns the winner of the TicTacToe game.
  fn winner(&self) -> Self::Player;

  // This method returns the player with a mark at the given (row, col) of the TicTacToe board.
  fn get_board_position(&self, row: u32, column: u32) -> Result<Self::Player, Self::Error>;

  // This method should reset the game as if it were just constructed.
  fn reset(&mut self);
}

// Write a struct to store game state, and
struct Game {
  ...
}

// Implement the TicTacToe trait for your Game struct
impl TicTacToe for Game {
  ...
}


// Write some unit tests for the TicTacToe game to exhibit basic game functionality.
#[cfg(test)]
mod tests {
  #[test]
  fn can_create_game() {
    let _game = Game::new();
  }
}
