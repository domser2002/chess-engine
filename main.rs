mod r#move;
mod position;
mod piece;
mod square;
mod uci;
mod play;
mod eval;
#[cfg(test)]
mod public_test;

fn main() {
    #[cfg(not(test))]
    {
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