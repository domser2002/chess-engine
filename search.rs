use crate::eval::evaluate;
use crate::position::{legal_moves, make_move, unmake_move, Position};
use crate::r#move::Move;

/// Find the best move by evaluating the position 1 ply ahead (greedy).
/// Picks the move that gives the highest immediate material score.
pub fn best_move(position: Position) -> Option<Move> {
    let moves = legal_moves(position);
    if moves.is_empty() {
        return None;
    }

    let mut best = moves[0];
    let mut best_score = i32::MIN;

    for &m in &moves {
        let mut pos = position;
        let undo = make_move(&mut pos, m);
        let score = -evaluate(pos);
        unmake_move(&mut pos, m, undo);

        if score > best_score {
            best_score = score;
            best = m;
        }
    }

    Some(best)
}