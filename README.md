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

## v0.4.1

Massive performance leap and full GUI compatibility. Engine can now confidently hit depths 7-9 within a few seconds and works seamlessly in professional chess interfaces.

- ✅ Move sorting — evaluates captures first, vastly improving alpha-beta pruning efficiency
- ✅ Killer moves heuristic — stores the last two moves that caused a beta cutoff at each ply and evaluates them early
- ✅ Check extensions — extends search depth by 1 ply when in check to avoid horizon effect blunders
- ✅ Node counting & progress logging — accurate profiling metric to measure search efficiency
- ✅ Full UCI compatibility — threaded communication loop, `stop` support, and outputting the principal variation (`pv`) token required by modern GUIs
- ✅ Dynamic time management — sophisticated game clock calculation from `wtime`, `btime`, `winc`, `binc`, and `movestogo` parameters
- ✅ Test suite FEN refactoring — entirely replaced verbose 15-line manual board setups with 1-line FEN strings
- ✅ Tactical test suite — new tests covering forks, pins, skewers, and forced mates to guarantee search correctness
- ✅ 124 unit tests (up from 114)

## v0.5.1

Algorithmic overhaul achieving a massive speedup in search performance. Advanced heuristics allow the engine to evaluate 5x fewer nodes and reach depths 10+ with tactical accuracy.

- ✅ CI/CD pipeline — automated `cargo test` and `cargo clippy` execution via GitHub Actions
- ✅ Perft command — `perft <depth>` command for strict move generation validation against known node counts
- ✅ Bench command — automated `bench <depth>` command to accurately measure Nodes Per Second (NPS) improvements
- ✅ Zobrist Hashing — fully incremental 64-bit XOR position hashing for fast state tracking
- ✅ Transposition Table (TT) — caches `ZobristHash`, depth, score, and best move to completely eliminate redundant tree searches
- ✅ TT Move Ordering — probes the Transposition Table and evaluates its `best_move` first, providing instantaneous alpha-beta cutoffs
- ✅ Null Move Pruning (NMP) — artificially passes the turn to safely prune huge branches in non-zugzwang positions
- ✅ History Heuristic — tracks quiet moves that successfully cause cutoffs in a `[70][70]` table and ranks them higher in sibling nodes
- ✅ Dynamic Hash Allocation — supports UCI `setoption name Hash value X` to dynamically resize the TT in megabytes
- ✅ UCI `ucinewgame` support — correctly clears the Transposition Table between games to prevent cross-game evaluation pollution
- ✅ MultiPV Search — supports `setoption name MultiPV value X` to search and output the top `X` lines sequentially
- ✅ 131 unit tests (up from 124)

## v0.6.1

Architectural overhaul completing the transition to a pure bitboard engine with advanced positional evaluation. The legacy 2D array structures have been eliminated, and performance is now strictly O(1) for board state evaluation.

- ✅ **Bitboard Architecture** — replaced the legacy `[[Option<Piece>; 8]; 8]` 2D array with 64-bit integer bitboards (`u64`), natively supporting fast bitwise operations.
- ✅ **Positional Evaluation (PeSTO Tables)** — integrated PeSTO's evaluation tables, transitioning from a basic material-only evaluation to a robust, phase-dependent positional evaluation system (distinguishing between middlegame and endgame square values).
- ✅ **Incremental Evaluation** — replaced the O(64) leaf-node evaluation loop with an O(1) state tracker. The `mg_score`, `eg_score`, and `phase` from the PeSTO tables are updated dynamically during `make_move`.
- ✅ **Optimized Move Execution** — rewrote `make_move` to apply bitboard XOR/AND operations natively, completely bypassing heavy generic piece lookup overhead.
- ✅ **MVV-LVA Bitwise Refactor** — improved move sorting by using native bitwise `AND` on bitboards to identify captured pieces, removing legacy array getter dependencies.
- 🐛 **Fixed Time Forfeits** — resolved an issue where the engine would lose on time by implementing a recursive search unwinding mechanism (`TimeOut` error) to immediately break out of the search tree and return the best move when the UCI clock expires.
- 🐛 **Fixed Castling / Rook Move Collision** — resolved a critical bitboard shifting bug where castling moves would trigger a "shift left overflow" due to incorrect square indices being evaluated as rook captures.
- ✅ 145 unit tests (up from 131), guaranteeing search stability across purely bitboard-based regression tests.

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

- **Bitboard representation**: `u64` bitmasks — replaced the legacy 2D array approach to allow lightning-fast bitwise operations.
- **Position is Copy**: Fits in a few cache lines. Both copy-make (in `legal_moves`) and make-unmake (in `search`) are used
- **Direction tables**: Sliding piece moves use direction constants (`ROOK_DIRS`, `BISHOP_DIRS`) and a shared `add_sliding_moves` function instead of N separate functions per direction
- **Pseudo-legal then filter**: `pseudo_legal_moves` generates moves without king safety, then `legal_moves` filters via copy-make-check
- **Iterative deepening with time guard**: Search checks the clock at each root move; only fully completed depth results are kept

## Running Tests

```bash
cargo test
```

All 145 tests should pass.