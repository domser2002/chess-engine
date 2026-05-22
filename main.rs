mod r#move;
mod position;
mod piece;
mod square;
mod uci;
#[cfg(test)]
mod public_test;

fn main() {
    uci::uci_loop();
}