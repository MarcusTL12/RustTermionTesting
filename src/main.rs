mod terminalgame;
use terminalgame::TerminalGame;

mod tictactoe;
use tictactoe::TicTacToe;

fn main() {
    let mut game = TicTacToe::new();
    game.start();
}
