mod r#move;
mod position;
mod piece;
mod square;
mod uci;
mod play;
mod eval;
mod search;
pub mod zobrist;
pub mod tt;
mod perft;
mod bench;
#[cfg(test)]
mod public_test;

#[cfg(not(test))]
use std::env;

fn main() {
    #[cfg(not(test))]
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            match args[1].as_str() {
                "perft" => {
                    let depth = args.get(2).and_then(|d| d.parse::<u8>().ok()).unwrap_or(5);
                    perft::run_perft_command(depth);
                    return;
                }
                "bench" => {
                    let depth = args.get(2).and_then(|d| d.parse::<u8>().ok()).unwrap_or(7);
                    bench::run_bench_command(depth);
                    return;
                }
                _ => {}
            }
        }

        // If stdin is a terminal, run interactive mode. Otherwise UCI mode.
        if atty::is(atty::Stream::Stdin) {
            play::interactive_loop();
        } else {
            uci::uci_loop();
        }
    }
    #[cfg(test)]
    {
        uci::uci_loop();
    }
}