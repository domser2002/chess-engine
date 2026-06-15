use crate::r#move::Move;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TTFlag {
    Exact,
    Alpha, // Upperbound
    Beta,  // Lowerbound
    Empty, // To avoid Option
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub hash: u64,
    pub depth: u8,
    pub flag: TTFlag,
    pub score: i32,
    pub best_move: Option<Move>,
}

impl Default for TTEntry {
    fn default() -> Self {
        Self {
            hash: 0,
            depth: 0,
            flag: TTFlag::Empty,
            score: 0,
            best_move: None,
        }
    }
}

pub struct TranspositionTable {
    entries: Vec<TTEntry>,
    size_mask: usize,
}

impl TranspositionTable {
    pub fn new(size_mb: usize) -> Self {
        // Size in MB. Each entry is 24-32 bytes depending on padding.
        let entry_size = std::mem::size_of::<TTEntry>();
        let target_bytes = size_mb * 1024 * 1024;
        let mut size = 1;
        while size * entry_size <= target_bytes {
            size <<= 1;
        }
        size >>= 1; // Power of two size
        if size == 0 {
            size = 1;
        }
        
        Self {
            entries: vec![TTEntry::default(); size],
            size_mask: size - 1,
        }
    }

    pub fn probe(&self, hash: u64) -> Option<TTEntry> {
        let index = (hash as usize) & self.size_mask;
        let entry = self.entries[index];
        if entry.flag != TTFlag::Empty && entry.hash == hash {
            return Some(entry);
        }
        None
    }

    pub fn store(&mut self, hash: u64, depth: u8, flag: TTFlag, score: i32, best_move: Option<Move>) {
        let index = (hash as usize) & self.size_mask;
        // Always replace strategy (or replace if depth >= current)
        // For simplicity and good performance, always replace is fine.
        self.entries[index] = TTEntry {
            hash,
            depth,
            flag,
            score,
            best_move,
        };
    }
}
