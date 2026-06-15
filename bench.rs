use std::time::Instant;
use crate::position::{parse_fen, start_position};
use crate::search::{search_with_time, SearchConfig, no_stop};

#[allow(dead_code)]
pub fn run_bench_command(depth: u8) {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", // Start position
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", // Kiwipete
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", // Position 3
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", // Position 4
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", // Position 5
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", // Position 6
    ];

    println!("Running bench at depth {depth}...");
    let mut total_nodes = 0;
    let start_time = Instant::now();

    for (i, fen) in fens.iter().enumerate() {
        let pos = if *fen == "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" {
            start_position()
        } else {
            parse_fen(fen).unwrap()
        };

        let config = SearchConfig {
            max_depth: i32::from(depth),
            uci_output: false,
            ..Default::default()
        };
        
        let result = search_with_time(pos, config, no_stop());
        total_nodes += result.nodes;
        
        println!("Position {}: score {}, nodes {}", i + 1, result.score, result.nodes);
    }

    let duration = start_time.elapsed();
    let nps = if duration.as_secs_f64() > 0.0 {
        #[allow(clippy::cast_precision_loss)]
        let nodes_f64 = total_nodes as f64;
        nodes_f64 / duration.as_secs_f64()
    } else {
        0.0
    };

    println!("===========================");
    println!("Total nodes: {total_nodes}");
    println!("Total time : {:.2}s", duration.as_secs_f64());
    println!("NPS        : {nps:.0}");
}
