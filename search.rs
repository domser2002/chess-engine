use crate::eval::evaluate;
use crate::position::{is_in_check, legal_moves, make_move, unmake_move, Position};
use crate::r#move::Move;

/// Default search depth (half-moves / ply).
const DEFAULT_DEPTH: i32 = 5;

/// Safe sentinel values for alpha-beta bounds.
/// Max material in any position is ~39,000 centipawns, so +/-100,000 is safe
/// and won't overflow when negated.
const MIN_SCORE: i32 = -100_000;
const MAX_SCORE: i32 = 100_000;

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
    let mut best_score = MIN_SCORE;

    for &m in &moves {
        let mut pos = position;
        let undo = make_move(&mut pos, m);
        let score = -negamax(pos, depth - 1, MIN_SCORE, MAX_SCORE);
        unmake_move(&mut pos, m, undo);

        if score > best_score {
            best_score = score;
            best = m;
        }
    }

    Some(best)
}

/// Negamax with alpha-beta pruning.
/// alpha = best score the maximizer can guarantee, beta = best score the minimizer can guarantee.
/// Returns the score from the perspective of the side to move.
fn negamax(position: Position, depth: i32, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return evaluate(position);
    }

    let moves = legal_moves(position);
    if moves.is_empty() {
        if is_in_check(position, position.turn) {
            return MIN_SCORE + 1;
        }
        return 0;
    }

    let mut score = MIN_SCORE;

    for &m in &moves {
        let mut pos = position;
        let undo = make_move(&mut pos, m);
        let val = -negamax(pos, depth - 1, -beta, -alpha);
        unmake_move(&mut pos, m, undo);

        if val >= beta {
            return beta; // Beta cutoff — opponent can force at least this
        }
        if val > score {
            score = val;
            if val > alpha {
                alpha = val;
            }
        }
    }

    score
}
