use std::fmt::Display;

use crate::lexer::Token;

#[derive(Debug)]
pub enum Stmt {
    Ref { lhs: String, rhs: String },
    Alias { lhs: String, rhs: String },
    DerefRead { lhs: String, rhs: String },
    DerefWrite { lhs: String, rhs: String },
}

pub enum IR {
    Stmt(Stmt),
    Branch(usize),
    Nop,
}

/// CompileStart -> Stmts
/// Stmts -> Stmt+
/// Stmt -> RefStmt | AliasStmt | DerefReadStmt | DerefWriteStmt | IfStmt | WhileStmt | ;
/// RefStmt -> VAR = & VAR ;
/// AliasStmt -> VAR = VAR ;
/// DerefReadStmt -> VAR = * VAR ;
/// DerefWriteStmt -> * VAR = VAR ;
/// IfStmt -> IF '{' Stmts '}'
/// WhileStmt -> WHILE '{' Stmts '}'
/// note: Stmt starts with one of [VAR, *, IF, WHILE]

pub fn parse(tokens: Vec<Token>) -> Vec<IR> {
    let mut reader = TokenReader { tokens, i: 0 };
    let mut ir = Vec::<IR>::new();
    parse_stmts(&mut reader, &mut ir);
    return ir;
}

fn parse_stmts(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    while !reader.is_eof()
        && matches!(
            reader.peek(),
            Token::Var(_) | Token::Star | Token::If | Token::While
        )
    {
        parse_stmt(reader, ir);
    }
}

fn parse_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    match reader.peek() {
        Token::If => parse_if_stmt(reader, ir),
        Token::While => parse_while_stmt(reader, ir),
        Token::Star => parse_deref_write_stmt(reader, ir),
        Token::Semicolon => reader.next(),
        Token::Var(_) => {
            if let Some(forward) = reader.peek_forward(2) {
                match forward {
                    Token::Ampersand => parse_ref_stmt(reader, ir),
                    Token::Star => parse_deref_read_stmt(reader, ir),
                    Token::Var(_) => parse_alias_stmt(reader, ir),
                    _ => unreachable!(),
                }
            }
        }
        _ => unreachable!(),
    }
}

fn parse_ref_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    let lhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    }
    .clone();
    reader.next(); // Var
    reader.next(); // &
    reader.next(); // =
    let rhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    }
    .clone();
    reader.next(); // Var
    reader.next(); // ;

    ir.push(IR::Stmt(Stmt::Ref { lhs, rhs }));
}

fn parse_alias_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    let lhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    }
    .clone();
    reader.next(); // Var
    reader.next(); // =
    let rhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    }
    .clone();
    reader.next(); // Var
    reader.next(); // ;

    ir.push(IR::Stmt(Stmt::Alias { lhs, rhs }));
}

fn parse_deref_read_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    let lhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    }
    .clone();
    reader.next(); // Var
    reader.next(); // =
    reader.next(); // *
    let rhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    }
    .clone();
    reader.next(); // Var
    reader.next(); // ;

    ir.push(IR::Stmt(Stmt::DerefRead { lhs, rhs }));
}

fn parse_deref_write_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    reader.next(); // *
    let lhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    }
    .clone();
    reader.next(); // Var
    reader.next(); // =
    let rhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    }
    .clone();
    reader.next(); // Var
    reader.next(); // ;

    ir.push(IR::Stmt(Stmt::DerefWrite { lhs, rhs }));
}

/// branch label
/// {
///    ...
/// }
/// label
/// Nop
fn parse_if_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    reader.next(); // If
    reader.next(); // {

    let idx_br = ir.len();
    ir.push(IR::Branch(0));

    parse_stmts(reader, ir);
    let label = ir.len();

    reader.next(); // }
    ir.push(IR::Nop);

    // fill back
    match ir.get_mut(idx_br).unwrap() {
        IR::Branch(x) => *x = label,
        _ => unreachable!(),
    }
}

/// branch_1 label_2
/// {
///    label_1
///    ...
///    branch_2 label_1
/// }
/// label_2
/// Nop
fn parse_while_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    reader.next(); // While
    reader.next(); // {

    let idx_br_1 = ir.len();
    ir.push(IR::Branch(0));

    let label_1 = ir.len();
    parse_stmts(reader, ir);

    let idx_br_2 = ir.len();
    ir.push(IR::Branch(0));

    let label_2 = ir.len();

    reader.next(); // }
    ir.push(IR::Nop);

    // fill back
    match ir.get_mut(idx_br_1).unwrap() {
        IR::Branch(x) => *x = label_2,
        _ => unreachable!(),
    }
    match ir.get_mut(idx_br_2).unwrap() {
        IR::Branch(x) => *x = label_1,
        _ => unreachable!(),
    }
}

struct TokenReader {
    tokens: Vec<Token>,
    i: usize,
}

impl TokenReader {
    fn peek(&self) -> &Token {
        return &self.tokens[self.i]; // panic if out of range
    }

    fn peek_forward(&self, offset: usize) -> Option<&Token> {
        return self.tokens.get(self.i + offset);
    }

    fn next(&mut self) {
        if !self.is_eof() {
            self.i += 1;
        }
    }

    fn is_eof(&self) -> bool {
        return self.tokens.len() == self.i;
    }
}

pub fn print_ir(ir_list: &Vec<IR>) {
    let mut i = 0;
    for ir in ir_list.iter() {
        print!("{} ", i);
        match ir {
            IR::Stmt(stmt) => println!("{}", stmt),
            IR::Branch(x) => println!("branch({})", x),
            IR::Nop => println!("nop"),
        }
        i += 1;
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Stmt::Ref { lhs, rhs } => write!(f, "{} = &{}", lhs, rhs),
            Stmt::Alias { lhs, rhs } => write!(f, "{} = {}", lhs, rhs),
            Stmt::DerefRead { lhs, rhs } => write!(f, "{} = *{}", lhs, rhs),
            Stmt::DerefWrite { lhs, rhs } => write!(f, "*{} = {}", lhs, rhs),
        };
    }
}
