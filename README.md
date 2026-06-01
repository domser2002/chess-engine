# Chess Engine

A UCI-compatible chess engine in Rust, built for learning. Written by a C developer exploring idiomatic Rust — no unsafe code, no premature optimization, just clean move generation and a focus on correctness.

## Current Status — v0.2.1

Functional but plays random legal moves. No search or evaluation yet.

### What works
- ✅ Legal move generation for all piece types
- ✅ True legality filtering (king safety check)
- ✅ `make_move` with promotions, en passant, captures
- ✅ Check detection
- ✅ Board display with Unicode chess symbols (♔♕♖♗♘♙ / ♚♛♜♝♞♟)
- ✅ Move notation: algebraic (`e4`, `Nf3`, `exd5`) and UCI (`e2e4`, `g1f3`)
- ✅ UCI protocol compatible — load in Arena, CuteChess, etc.
- ✅ Interactive terminal mode
- ✅ 58 unit tests covering all functionality

### What's next
- Castling logic
- Search (minimax / alpha-beta)
- Evaluation function
- FEN parsing
- Time management

## How to Play

### Interactive mode (terminal)
```bash
cargo run --release
```

Type moves in algebraic notation: `e4`, `Nf3`, `0-0`, `exd5`, `f8=Q`
Type `moves` to see all legal moves, `quit` to exit.

### UCI mode (for GUIs)
```bash
echo -e "uci\nisready\nposition startpos\ngo\nquit" | cargo run
```

Or load the binary at `target/release/chess-engine` in any UCI-compatible GUI.

## Architecture

| Module | File | Purpose |
|--------|------|---------|
| `piece` | `piece.rs` | Piece types, colors, constants, character conversion |
| `square` | `square.rs` | Square indexing (1..=64), rank/file conversion |
| `move` | `move.rs` | Move struct, printing, parsing (UCI + algebraic) |
| `position` | `position.rs` | Board state, move generation, make/unmake, check |
| `uci` | `uci.rs` | UCI protocol command loop |
| `play` | `play.rs` | Interactive terminal play mode |

### Key design decisions

- **2D array board representation**: `[[Option<Piece>; 8]; 8]` — the C-like approach, intentional for learning
- **Position is Copy**: ~72 bytes, fits in two cache lines. Copy-make is used currently; make-unmake can be added later in the search module
- **Direction tables**: Sliding piece moves use direction constants (`ROOK_DIRS`, `BISHOP_DIRS`) and a shared `add_sliding_moves` function instead of N separate functions per direction
- **Pseudo-legal then filter**: `pseudo_legal_moves` generates moves without king safety, then `legal_moves` filters via copy-make-check

## Running Tests

```bash
cargo test
```

All 58 tests should pass.