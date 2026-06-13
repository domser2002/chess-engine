use crate::eval::evaluate;
use crate::position::{is_in_check, legal_moves, make_move, unmake_move, Position};
use crate::r#move::Move;

/// Default search depth (half-moves / ply). Depth 3 = engine looks 3 half-moves ahead.
const DEFAULT_DEPTH: i32 = 4;

/// Find the best move using depth-limited minimax at `DEFAULT_DEPTH`.
pub fn best_move(position: Position) -> Option<Move> {
    best_move_depth(position, DEFAULT_DEPTH)
}

/// Find the best move using minimax with the given search depth.
/// Depth counts half-moves (plies). Depth 1 = greedy (1-ply).
pub fn best_move_depth(position: Position, depth: i32) -> Option<Move> {
    let moves = legal_moves(position);
    if moves.is_empty() {
        return None;
    }

    let mut best = moves[0];
    let mut best_score = i32::MIN;

    for &m in &moves {
        let mut pos = position;
        let undo = make_move(&mut pos, m);
        let score = -negamax(pos, depth - 1);
        unmake_move(&mut pos, m, undo);

        if score > best_score {
            best_score = score;
            best = m;
        }
    }

    Some(best)
}

/// Minimax (negamax-style): returns the score from the perspective of the side to move.
/// At depth 0, evaluates the position. Otherwise tries all legal moves and picks the best.
fn negamax(position: Position, depth: i32) -> i32 {
    if depth == 0 {
        return evaluate(position);
    }

    let moves = legal_moves(position);
    if moves.is_empty() {
        // No legal moves = checkmate or stalemate
        if is_in_check(position, position.turn) {
            return i32::MIN + 1; // Checkmate (worst possible for this side)
        }
        return 0; // Stalemate
    }

    let mut best_score = i32::MIN;

    for &m in &moves {
        let mut pos = position;
        let undo = make_move(&mut pos, m);
        let score = -negamax(pos, depth - 1);
        unmake_move(&mut pos, m, undo);

        if score > best_score {
            best_score = score;
        }
    }

    best_score
}
