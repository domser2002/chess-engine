use crate::eval::{evaluate, piece_value};
use crate::r#move::Move;
use crate::position::{Position, capture_moves, is_in_check, legal_moves, make_move, unmake_move};
use std::io::{self, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Default search depth (half-moves / ply).
const DEFAULT_DEPTH: i32 = 100;

/// Default time per move in seconds.
const DEFAULT_MOVE_TIME_SECS: u64 = 10;

const MIN_SCORE: i32 = -100_000;
const MAX_SCORE: i32 = 100_000;
const MATE_BOUND: i32 = 90_000;

fn store_score(score: i32, ply: usize) -> i32 {
    if score > MATE_BOUND {
        score + ply as i32
    } else if score < -MATE_BOUND {
        score - ply as i32
    } else {
        score
    }
}

fn retrieve_score(score: i32, ply: usize) -> i32 {
    if score > MATE_BOUND {
        score - ply as i32
    } else if score < -MATE_BOUND {
        score + ply as i32
    } else {
        score
    }
}

/// Search configuration.
#[derive(Clone, Copy)]
pub struct SearchConfig {
    /// Maximum time to spend on one move, in milliseconds.
    pub time_per_move_ms: u64,
    /// Maximum search depth (overrides time if reached first).
    pub max_depth: i32,
    /// If true, print UCI `info` lines after each completed depth.
    pub uci_output: bool,
    /// Number of best lines to search (MultiPV).
    pub multi_pv: usize,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            time_per_move_ms: DEFAULT_MOVE_TIME_SECS * 1000,
            max_depth: DEFAULT_DEPTH,
            uci_output: false,
            multi_pv: 1,
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
/// Starts at depth 1, increases depth until time runs out, `max_depth` is reached, or `stop` is set.
pub fn search_with_time(
    position: Position,
    config: SearchConfig,
    stop: Arc<AtomicBool>,
) -> SearchResult {
    let mut tt = crate::tt::TranspositionTable::new(16);
    search_with_time_and_tt(position, config, stop, &mut tt)
}

pub fn search_with_time_and_tt(
    position: Position,
    config: SearchConfig,
    stop: Arc<AtomicBool>,
    tt: &mut crate::tt::TranspositionTable,
) -> SearchResult {
    let mut moves = legal_moves(position);
    let mut killers = [[None; 2]; 128];
    let mut history = [[0; 70]; 70];
    order_moves(&position, &mut moves, 0, &killers, &history, None);
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
    let mut multi_pv_results: Vec<Move> = Vec::new();

    // Iterative deepening: search depth 1, 2, 3, ... until time runs out or stop is signalled
    for depth in 1..=config.max_depth {
        // Check if we still have time to search this depth
        if (Instant::now() >= deadline || stop.load(Ordering::Relaxed)) && depth > 1 {
            break;
        }

        let mut excluded_moves = Vec::new();
        let mut iteration_completed = true;

        for pv_idx in 0..config.multi_pv.min(moves.len()) {
            if (Instant::now() >= deadline || stop.load(Ordering::Relaxed)) && depth > 1 {
                iteration_completed = false;
                break;
            }

            // PV move ordering: swap the best move from the previous iteration to the front
            if let Some(&prev_pv) = multi_pv_results.get(pv_idx) {
                if let Some(idx) = moves.iter().position(|&m| m == prev_pv) {
                    moves.swap(0, idx);
                }
            }

            let mut best = moves
                .iter()
                .find(|m| !excluded_moves.contains(*m))
                .copied()
                .unwrap_or(moves[0]);
            let mut best_score = MIN_SCORE;
            let mut alpha = MIN_SCORE;

            for &m in &moves {
                if excluded_moves.contains(&m) {
                    continue;
                }

                // Check time and stop flag periodically (at each root move)
                if (Instant::now() >= deadline || stop.load(Ordering::Relaxed)) && depth > 1 {
                    iteration_completed = false;
                    break;
                }

                let mut pos = position;
                let undo = make_move(&mut pos, m);
                let score = -negamax(
                    pos,
                    depth - 1,
                    -MAX_SCORE,
                    -alpha,
                    &mut total_nodes,
                    1,
                    &mut killers,
                    &mut history,
                    tt,
                    deadline,
                    &stop,
                );
                unmake_move(&mut pos, m, undo);

                if score > best_score {
                    best_score = score;
                    best = m;
                    if score > alpha {
                        alpha = score;
                    }
                }
            }

            if !iteration_completed {
                break;
            }

            excluded_moves.push(best);

            if pv_idx == 0 {
                best_result = SearchResult {
                    best_move: Some(best),
                    depth_reached: depth,
                    score: best_score,
                    nodes: total_nodes,
                };
            }

            if config.uci_output {
                let elapsed_ms = start.elapsed().as_millis().max(1) as u64;
                let nps = (total_nodes * 1000) / elapsed_ms;

                let score_str = if best_score > MATE_BOUND - 100 {
                    format!("mate {}", (MAX_SCORE - best_score + 1) / 2)
                } else if best_score < -MATE_BOUND + 100 {
                    format!("mate -{}", (MAX_SCORE + best_score) / 2)
                } else {
                    format!("cp {}", best_score)
                };

                let mut pv_str = crate::uci::move_to_uci(best);
                let mut pv_pos = position;
                let mut pv_depth = depth - 1;
                let mut current_move = best;
                let mut visited_hashes = vec![pv_pos.hash];

                while pv_depth > 0 {
                    let _undo = make_move(&mut pv_pos, current_move);
                    if visited_hashes.contains(&pv_pos.hash) {
                        break;
                    }
                    visited_hashes.push(pv_pos.hash);

                    if let Some(entry) = tt.probe(pv_pos.hash) {
                        if let Some(m) = entry.best_move {
                            pv_str.push_str(&format!(" {}", crate::uci::move_to_uci(m)));
                            current_move = m;
                            pv_depth -= 1;
                            continue;
                        }
                    }
                    break;
                }

                let info_msg = format!(
                    "info depth {depth} multipv {} score {score_str} nodes {total_nodes} nps {nps} hashfull {} time {elapsed_ms} pv {pv_str}",
                    pv_idx + 1,
                    tt.hashfull()
                );
                println!("{info_msg}");
                io::stdout().flush().unwrap();
                let mut log_file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("/tmp/chess_uci.log")
                    .unwrap_or_else(|_| std::fs::File::create("/dev/null").unwrap());
                std::io::Write::write_fmt(&mut log_file, format_args!("< {info_msg}\n")).ok();
            }
        }

        if iteration_completed {
            multi_pv_results = excluded_moves;
        }
    }

    best_result.nodes = total_nodes;
    best_result
}

/// Create a stop flag that is never triggered (for non-UCI use).
pub fn no_stop() -> Arc<AtomicBool> {
    Arc::new(AtomicBool::new(false))
}

/// Search with default time and depth. Used by tests.
#[cfg(test)]
pub fn best_move(position: Position) -> Option<Move> {
    search_with_time(position, SearchConfig::default(), no_stop()).best_move
}

/// Search with a fixed depth (no time limit). Used by tests.
#[cfg(test)]
pub fn best_move_depth(position: Position, depth: i32) -> Option<Move> {
    let config = SearchConfig {
        max_depth: depth,
        time_per_move_ms: 60_000,
        uci_output: false,
        multi_pv: 1,
    };
    search_with_time(position, config, no_stop()).best_move
}

/// Negamax with alpha-beta pruning.
/// `nodes` is incremented for every position evaluated.
fn negamax(
    position: Position,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    nodes: &mut u64,
    ply: usize,
    killers: &mut [[Option<Move>; 2]; 128],
    history: &mut [[i32; 70]; 70],
    tt: &mut crate::tt::TranspositionTable,
    deadline: Instant,
    stop: &Arc<AtomicBool>,
) -> i32 {
    *nodes += 1;

    if (*nodes & 2047) == 0 {
        if Instant::now() >= deadline || stop.load(Ordering::Relaxed) {
            stop.store(true, Ordering::Relaxed);
            return 0;
        }
    }

    if depth <= 0 {
        return quiescence(position, alpha, beta, nodes, ply, killers, deadline, stop);
    }

    let mut tt_move = None;
    if let Some(entry) = tt.probe(position.hash) {
        tt_move = entry.best_move;
        if entry.depth >= depth as u8 {
            let score = retrieve_score(entry.score, ply);
            match entry.flag {
                crate::tt::TTFlag::Exact => return score,
                crate::tt::TTFlag::Alpha => {
                    if score <= alpha {
                        return score;
                    }
                }
                crate::tt::TTFlag::Beta => {
                    if score >= beta {
                        return score;
                    }
                }
                crate::tt::TTFlag::Empty => {}
            }
        }
    }

    let in_check = is_in_check(position, position.turn);

    // Null Move Pruning
    if depth >= 3 && !in_check && position.has_non_pawn_material(position.turn) {
        let mut pos = position;
        let undo = pos.make_null_move();

        let r = 2; // Depth reduction for null move
        let val = -negamax(
            pos,
            depth - 1 - r,
            -beta,
            -beta + 1,
            nodes,
            ply + 1,
            killers,
            history,
            tt,
            deadline,
            stop,
        );

        pos.unmake_null_move(undo);

        if val >= beta {
            return beta;
        }
    }

    let mut moves = legal_moves(position);
    order_moves(&position, &mut moves, ply, killers, history, tt_move);
    if moves.is_empty() {
        if in_check {
            // Favor faster mates by subtracting ply
            return MIN_SCORE + ply as i32;
        }
        return 0;
    }

    // Check extension: if we are in check, search 1 ply deeper, up to a maximum depth (e.g. ply < 128)
    let extension = i32::from(in_check && ply < 128);

    let mut score = MIN_SCORE;
    let old_alpha = alpha;
    let mut best_move = None;

    for &m in &moves {
        let mut pos = position;
        let undo = make_move(&mut pos, m);
        let val = -negamax(
            pos,
            depth - 1 + extension,
            -beta,
            -alpha,
            nodes,
            ply + 1,
            killers,
            history,
            tt,
            deadline,
            stop,
        );
        unmake_move(&mut pos, m, undo);

        if val >= beta {
            if !m.capture {
                if ply < 128 && killers[ply][0] != Some(m) {
                    killers[ply][1] = killers[ply][0];
                    killers[ply][0] = Some(m);
                }
                let bonus = depth * depth;
                let v = &mut history[m.from as usize][m.to as usize];
                *v = v.saturating_add(bonus);
            }
            tt.store(
                position.hash,
                depth as u8,
                crate::tt::TTFlag::Beta,
                store_score(beta, ply),
                Some(m),
            );
            return beta;
        }
        if val > score {
            score = val;
            best_move = Some(m);
            if val > alpha {
                alpha = val;
            }
        }
    }

    let flag = if score <= old_alpha {
        crate::tt::TTFlag::Alpha
    } else {
        crate::tt::TTFlag::Exact
    };
    tt.store(
        position.hash,
        depth as u8,
        flag,
        store_score(score, ply),
        best_move,
    );

    score
}

/// Quiescence search: resolves capture sequences beyond the search horizon to avoid the horizon effect.
fn quiescence(
    position: Position,
    mut alpha: i32,
    beta: i32,
    nodes: &mut u64,
    ply: usize,
    killers: &mut [[Option<Move>; 2]; 128],
    deadline: Instant,
    stop: &Arc<AtomicBool>,
) -> i32 {
    *nodes += 1;

    if (*nodes & 2047) == 0 {
        if Instant::now() >= deadline || stop.load(Ordering::Relaxed) {
            stop.store(true, Ordering::Relaxed);
            return 0;
        }
    }

    let stand_pat = evaluate(position);
    if stand_pat >= beta {
        return beta;
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut moves = capture_moves(position);
    static EMPTY_HISTORY: [[i32; 70]; 70] = [[0; 70]; 70];
    order_moves(&position, &mut moves, ply, killers, &EMPTY_HISTORY, None);

    let mut score = stand_pat;

    for &m in &moves {
        let mut pos = position;
        let undo = make_move(&mut pos, m);
        let val = -quiescence(pos, -beta, -alpha, nodes, ply + 1, killers, deadline, stop);
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
fn score_move(
    position: &Position,
    m: &Move,
    ply: usize,
    killers: &[[Option<Move>; 2]; 128],
    history: &[[i32; 70]; 70],
    tt_move: Option<Move>,
) -> i32 {
    if Some(*m) == tt_move {
        return 10_000_000;
    }
    if m.capture {
        let (col, row) = crate::square::get_col_and_row(m.to);
        let victim_val = match position.get_piece(row as usize, col as usize) {
            Some(p) => piece_value(p),
            None => 100, // En passant capture victim is always a pawn
        };
        let attacker_val = piece_value(m.piece);
        // MVV-LVA: Most Valuable Victim - Least Valuable Attacker
        1_000_000 + 10 * victim_val - attacker_val
    } else {
        if ply < 128 {
            if killers[ply][0] == Some(*m) {
                return 900_000;
            } else if killers[ply][1] == Some(*m) {
                return 800_000;
            }
        }
        history[m.from as usize][m.to as usize]
    }
}

/// Order moves to improve alpha-beta pruning efficiency.
fn order_moves(
    position: &Position,
    moves: &mut [Move],
    ply: usize,
    killers: &[[Option<Move>; 2]; 128],
    history: &[[i32; 70]; 70],
    tt_move: Option<Move>,
) {
    moves.sort_unstable_by_key(|m| -score_move(position, m, ply, killers, history, tt_move));
}
