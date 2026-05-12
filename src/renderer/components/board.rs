use tuirealm::component::{Component,AppComponent};


use crate::game::{Board, GameResult, Position};

pub struct BoardComponent {
    board: Board,
    selected_cell: Option<Position>,
}
