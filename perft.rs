use std::time::Instant;
use crate::position::{Position, legal_moves, make_move, unmake_move, start_position};

pub fn perft(pos: &mut Position, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = legal_moves(*pos);
    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes = 0;
    for m in moves {
        let undo = make_move(pos, m);
        nodes += perft(pos, depth - 1);
        unmake_move(pos, m, undo);
    }
    nodes
}

#[allow(dead_code)]
pub fn divide(pos: &mut Position, depth: u8) {
    println!("--- Perft Depth {depth} ---");
    let start_time = Instant::now();
    let moves = legal_moves(*pos);
    let mut total_nodes = 0;
    
    for m in moves {
        let undo = make_move(pos, m);
        let nodes = perft(pos, depth - 1);
        unmake_move(pos, m, undo);
        
        println!("{}: {}", crate::uci::move_to_uci(m), nodes);
        total_nodes += nodes;
    }
    
    let duration = start_time.elapsed();
    let nps = if duration.as_secs_f64() > 0.0 {
        #[allow(clippy::cast_precision_loss)]
        let nodes_f64 = total_nodes as f64;
        nodes_f64 / duration.as_secs_f64()
    } else {
        0.0
    };
    
    println!("Total nodes: {total_nodes}");
    println!("Time: {:.2}s", duration.as_secs_f64());
    println!("NPS: {nps:.0}");
}

#[allow(dead_code)]
pub fn run_perft_command(depth: u8) {
    let mut pos = start_position();
    divide(&mut pos, depth);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::parse_fen;

    #[test]
    fn perft_start_position() {
        let mut pos = start_position();
        assert_eq!(perft(&mut pos, 1), 20);
        assert_eq!(perft(&mut pos, 2), 400);
        assert_eq!(perft(&mut pos, 3), 8902);
        assert_eq!(perft(&mut pos, 4), 197_281);
    }

    #[test]
    fn perft_kiwipete() {
        // Kiwipete is a famous perft position with a lot of tactical complexity early on
        let mut pos = parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
        assert_eq!(perft(&mut pos, 1), 48);
        assert_eq!(perft(&mut pos, 2), 2039);
        assert_eq!(perft(&mut pos, 3), 97862);
    }

    #[test]
    fn perft_pos3() {
        let mut pos = parse_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
        assert_eq!(perft(&mut pos, 1), 14);
        assert_eq!(perft(&mut pos, 2), 191);
        assert_eq!(perft(&mut pos, 3), 2812);
        assert_eq!(perft(&mut pos, 4), 43238);
    }
}
