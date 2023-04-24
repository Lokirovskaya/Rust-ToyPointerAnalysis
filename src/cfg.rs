use std::ptr::null_mut;

use crate::parser::Stmt;

pub struct CFG {
    root: BasicBlock,
}

type Link = *mut BasicBlock;

struct BasicBlock {
    stmts: Vec<Stmt>,
    next: [Link; 2],
}
