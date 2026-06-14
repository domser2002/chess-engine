# Chess Engine

A UCI-compatible chess engine in Rust, built for learning. Written by a C developer exploring idiomatic Rust — no unsafe code, no premature optimization, just clean move generation and a focus on correctness.

## v0.2.1

Functional but plays random legal moves. No search or evaluation yet.

- ✅ Legal move generation for all piece types
- ✅ True legality filtering (king safety check)
- ✅ `make_move` with promotions, en passant, captures and castling
- ✅ Check detection
- ✅ Board display with Unicode chess symbols (♔♕♖♗♘♙ / ♚♛♜♝♞♟)
- ✅ Move notation: algebraic (`e4`, `Nf3`, `exd5`) and UCI (`e2e4`, `g1f3`)
- ✅ UCI protocol compatible — load in Arena, CuteChess, etc.
- ✅ Interactive terminal mode
- ✅ 65 unit tests covering all functionality

## v0.3.1

Plays real chess with alpha-beta search, iterative deepening, and time management.

- ✅ `unmake_move` — proper undo via `UndoInfo` struct, restoring castle rights, captures, and en passant
- ✅ Material evaluation function — standard piece values (P=100, N=320, B=330, R=500, Q=900), side-to-move perspective
- ✅ Negamax search with alpha-beta pruning
- ✅ Iterative deepening (depth 1→5), keeping only fully completed depth results
- ✅ Time management — configurable per-move time, UCI `wtime`/`btime`/`depth` support (~1/40th of remaining clock)
- ✅ FEN parsing — `parse_fen` with full support for piece placement, active color, castling rights, en passant target
- ✅ UCI `position fen <fen> [moves ...]` now works (previously fell back to startpos)
- ✅ Move disambiguation for algebraic notation (`Rfc1`, `R1c3`, `Ngf3`)
- ✅ Interactive mode now displays search score: `Engine move: Nf3 (depth 5, score 42)`
- 🐛 Fixed partial-depth blunder — incomplete iterative deepening results no longer overwrite completed results
- 🐛 Fixed Black short-castling UCI — `move_to_uci` was always returning `e1g1`; now correctly returns `e8g8` for Black
- 🐛 Fixed algebraic move parsing disambiguation
- ✅ Zero compilation warnings
- ✅ 114 unit tests (up from 65)

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
| `position` | `position.rs` | Board state, move generation, make/unmake, check, FEN parsing |
| `eval` | `eval.rs` | Static position evaluation (material) |
| `search` | `search.rs` | Negamax, alpha-beta, iterative deepening, time management |
| `uci` | `uci.rs` | UCI protocol command loop |
| `play` | `play.rs` | Interactive terminal play mode |

### Key design decisions

- **2D array board representation**: `[[Option<Piece>; 8]; 8]` — the C-like approach, intentional for learning
- **Position is Copy**: ~72 bytes, fits in two cache lines. Both copy-make (in `legal_moves`) and make-unmake (in `search`) are used
- **Direction tables**: Sliding piece moves use direction constants (`ROOK_DIRS`, `BISHOP_DIRS`) and a shared `add_sliding_moves` function instead of N separate functions per direction
- **Pseudo-legal then filter**: `pseudo_legal_moves` generates moves without king safety, then `legal_moves` filters via copy-make-check
- **Iterative deepening with time guard**: Search checks the clock at each root move; only fully completed depth results are kept

## Running Tests

```bash
cargo test
```

All 114 tests should pass.