use std::time::{Duration, Instant};
use crate::eval::evaluate;
use crate::position::{is_in_check, legal_moves, make_move, unmake_move, Position};
use crate::r#move::Move;

/// Default search depth (half-moves / ply).
const DEFAULT_DEPTH: i32 = 5;

/// Default time per move in seconds.
const DEFAULT_MOVE_TIME_SECS: u64 = 10;

/// Safe sentinel values for alpha-beta bounds.
const MIN_SCORE: i32 = -100_000;
const MAX_SCORE: i32 = 100_000;

/// Search configuration.
#[derive(Clone, Copy)]
pub struct SearchConfig {
    /// Maximum time to spend on one move, in milliseconds.
    pub time_per_move_ms: u64,
    /// Maximum search depth (overrides time if reached first).
    pub max_depth: i32,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            time_per_move_ms: DEFAULT_MOVE_TIME_SECS * 1000,
            max_depth: DEFAULT_DEPTH,
        }
    }
}

/// Result of a timed search.
pub struct SearchResult {
    pub best_move: Option<Move>,
    pub depth_reached: i32,
    pub score: i32,
}

/// Find the best move using iterative deepening with time management.
/// Starts at depth 1, increases depth until time runs out or max_depth is reached.
pub fn search_with_time(position: Position, config: SearchConfig) -> SearchResult {
    let moves = legal_moves(position);
    if moves.is_empty() {
        return SearchResult {
            best_move: None,
            depth_reached: 0,
            score: 0,
        };
    }

    let deadline = Instant::now() + Duration::from_millis(config.time_per_move_ms);
    let mut best_result = SearchResult {
        best_move: Some(moves[0]),
        depth_reached: 0,
        score: MIN_SCORE,
    };

    // Iterative deepening: search depth 1, 2, 3, ... until time runs out
    for depth in 1..=config.max_depth {
        // Check if we still have time to search this depth
        if Instant::now() >= deadline && depth > 1 {
            break;
        }

        let mut best = moves[0];
        let mut best_score = MIN_SCORE;
        let mut completed = true;

        for &m in &moves {
            // Check time periodically (at each root move)
            if Instant::now() >= deadline && depth > 1 {
                completed = false;
                break;
            }

            let mut pos = position;
            let undo = make_move(&mut pos, m);
            let score = -negamax(pos, depth - 1, MIN_SCORE, MAX_SCORE);
            unmake_move(&mut pos, m, undo);

            if score > best_score {
                best_score = score;
                best = m;
            }
        }

        // Only store the result if we completed the full depth search.
        // A partial result (timeout mid-depth) could be worse than a previous depth.
        if completed {
            best_result = SearchResult {
                best_move: Some(best),
                depth_reached: depth,
                score: best_score,
            };
        }
    }

    best_result
}

/// Search with default time and depth. Used by tests.
#[cfg(test)]
pub fn best_move(position: Position) -> Option<Move> {
    search_with_time(position, SearchConfig::default()).best_move
}

/// Search with a fixed depth (no time limit). Used by tests.
#[cfg(test)]
pub fn best_move_depth(position: Position, depth: i32) -> Option<Move> {
    let config = SearchConfig {
        max_depth: depth,
        time_per_move_ms: 60_000,
    };
    search_with_time(position, config).best_move
}

/// Negamax with alpha-beta pruning.
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
            return beta;
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