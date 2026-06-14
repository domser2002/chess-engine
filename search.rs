use std::io::{self, Write};
use std::time::{Duration, Instant};
use crate::eval::{evaluate, piece_value};
use crate::position::{is_in_check, legal_moves, capture_moves, make_move, unmake_move, Position};
use crate::r#move::Move;

/// Default search depth (half-moves / ply).
const DEFAULT_DEPTH: i32 = 100;

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
    /// If true, print UCI `info` lines after each completed depth.
    pub uci_output: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            time_per_move_ms: DEFAULT_MOVE_TIME_SECS * 1000,
            max_depth: DEFAULT_DEPTH,
            uci_output: false,
        }
    }
}

/// Result of a timed search.
#[cfg_attr(test, allow(dead_code))]
pub struct SearchResult {
    pub best_move: Option<Move>,
    pub depth_reached: i32,
    pub score: i32,
    pub nodes: u64,
}

/// Find the best move using iterative deepening with time management.
/// Starts at depth 1, increases depth until time runs out or max_depth is reached.
pub fn search_with_time(position: Position, config: SearchConfig) -> SearchResult {
    let mut moves = legal_moves(position);
    let mut killers = [[None; 2]; 128];
    order_moves(&position, &mut moves, 0, &killers);
    if moves.is_empty() {
        return SearchResult {
            best_move: None,
            depth_reached: 0,
            score: 0,
            nodes: 0,
        };
    }

    let start = Instant::now();
    let deadline = start + Duration::from_millis(config.time_per_move_ms);
    let mut best_result = SearchResult {
        best_move: Some(moves[0]),
        depth_reached: 0,
        score: MIN_SCORE,
        nodes: 0,
    };
    let mut total_nodes: u64 = 0;
    let mut best_from_prev: Option<Move> = None;

    // Iterative deepening: search depth 1, 2, 3, ... until time runs out
    for depth in 1..=config.max_depth {
        // Check if we still have time to search this depth
        if Instant::now() >= deadline && depth > 1 {
            break;
        }

        // PV move ordering: swap the best move from the previous iteration to the front
        if let Some(pv_m) = best_from_prev {
            if let Some(idx) = moves.iter().position(|&m| m == pv_m) {
                moves.swap(0, idx);
            }
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
            // Root moves are ply 0
            let score = -negamax(pos, depth - 1, MIN_SCORE, MAX_SCORE, &mut total_nodes, 1, &mut killers);
            unmake_move(&mut pos, m, undo);

            if score > best_score {
                best_score = score;
                best = m;
            }
        }

        // Only store the result if we completed the full depth search.
        // A partial result (timeout mid-depth) could be worse than a previous depth.
        if completed {
            best_from_prev = Some(best);
            best_result = SearchResult {
                best_move: Some(best),
                depth_reached: depth,
                score: best_score,
                nodes: total_nodes,
            };

            if config.uci_output {
                let elapsed_ms = start.elapsed().as_millis();
                println!("info depth {depth} score cp {best_score} nodes {total_nodes} time {elapsed_ms}");
                io::stdout().flush().unwrap();
            }
        }
    }

    best_result.nodes = total_nodes;
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
        uci_output: false,
    };
    search_with_time(position, config).best_move
}

/// Negamax with alpha-beta pruning.
/// `nodes` is incremented for every position evaluated.
fn negamax(position: Position, depth: i32, mut alpha: i32, beta: i32, nodes: &mut u64, ply: usize, killers: &mut [[Option<Move>; 2]; 128]) -> i32 {
    *nodes += 1;

    if depth == 0 {
        return quiescence(position, alpha, beta, nodes, ply, killers);
    }

    let mut moves = legal_moves(position);
    order_moves(&position, &mut moves, ply, killers);
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
        let val = -negamax(pos, depth - 1, -beta, -alpha, nodes, ply + 1, killers);
        unmake_move(&mut pos, m, undo);

        if val >= beta {
            if !m.capture && ply < 128 {
                if killers[ply][0] != Some(m) {
                    killers[ply][1] = killers[ply][0];
                    killers[ply][0] = Some(m);
                }
            }
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

/// Quiescence search: resolves capture sequences beyond the search horizon to avoid the horizon effect.
fn quiescence(position: Position, mut alpha: i32, beta: i32, nodes: &mut u64, ply: usize, killers: &mut [[Option<Move>; 2]; 128]) -> i32 {
    *nodes += 1;

    let stand_pat = evaluate(position);
    if stand_pat >= beta {
        return beta;
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut moves = capture_moves(position);
    order_moves(&position, &mut moves, ply, killers);

    let mut score = stand_pat;

    for &m in &moves {
        let mut pos = position;
        let undo = make_move(&mut pos, m);
        let val = -quiescence(pos, -beta, -alpha, nodes, ply + 1, killers);
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

/// Score a move for move ordering (MVV-LVA and Killers).
fn score_move(position: &Position, m: &Move, ply: usize, killers: &[[Option<Move>; 2]; 128]) -> i32 {
    if m.capture {
        let (col, row) = crate::square::get_col_and_row(m.to);
        let victim_val = match position.pieces[row as usize][col as usize] {
            Some(p) => piece_value(p),
            None => 100, // En passant capture victim is always a pawn
        };
        let attacker_val = piece_value(m.piece);
        // MVV-LVA: Most Valuable Victim - Least Valuable Attacker
        10000 + 10 * victim_val - attacker_val
    } else {
        if ply < 128 {
            if killers[ply][0] == Some(*m) {
                return 90;
            } else if killers[ply][1] == Some(*m) {
                return 80;
            }
        }
        0
    }
}

/// Order moves to improve alpha-beta pruning efficiency.
fn order_moves(position: &Position, moves: &mut [Move], ply: usize, killers: &[[Option<Move>; 2]; 128]) {
    moves.sort_unstable_by_key(|m| -score_move(position, m, ply, killers));
}