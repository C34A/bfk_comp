#[derive(Debug)]
pub struct Inst {
    pub idx: usize,     // Index of instruction
    pub kind: InstKind, // Kind of instruction
    pub times: usize,   // run-length encoding of instruction
}

#[derive(Debug, PartialEq)]
pub enum InstKind {
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    WriteByte,
    ReadByte,
    // end_idx = index of instruction after matching LoopEnd
    LoopStart { end_idx: usize },
    // start_idx = index of instruction after matching LoopStart
    LoopEnd { start_idx: usize },
}