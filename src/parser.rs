use crate::lexer::Token;

pub enum Stmt {
    Ref { lhs: String, rhs: String },
    Alias { lhs: String, rhs: String },
    DerefRead { lhs: String, rhs: String },
    DerefWrite { lhs: String, rhs: String },
}

pub enum IR {
    Stmt(Stmt),
    Goto(Vec<u32>),
    Label(u32),
}

/// CompileStart -> Stmts
/// Stmts -> Stmt+
/// Stmt -> RefStmt | AliasStmt | DerefReadStmt | DerefWriteStmt | IfStmt | WhileStmt
/// RefStmt -> VAR = & VAR ;
/// AliasStmt -> VAR = VAR ;
/// DerefReadStmt -> VAR = * VAR ;
/// DerefWriteStmt -> * VAR = VAR ;
/// IfStmt -> IF '{' Stmts '}'
/// WhileStmt -> WHILE '{' Stmts '}'
/// note: Stmt starts with one of [VAR, *, IF, WHILE]

pub fn parse(tokens: Vec<Token>) {
    let mut reader = TokenReader { tokens, i: 0 };
    let mut ir = Vec::<IR>::new();
    parse_stmts(&mut reader, &mut ir)
}

fn parse_stmts(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    while !reader.is_eof() {
        let is_stmt_start = matches!(
            reader.peek(),
            Token::Var(_) | Token::Star | Token::If | Token::While
        );
        if is_stmt_start {
            parse_stmt(reader, ir);
        }
    }
}

fn parse_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    match reader.peek() {
        Token::If => parse_if_stmt(reader, ir),
        Token::While => parse_while_stmt(reader, ir),
        Token::Star => parse_deref_write_stmt(reader, ir),
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
    };
    reader.next(); // Var
    reader.next(); // &
    reader.next(); // =
    let rhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    };
    reader.next(); // Var
    reader.next(); // ;

    ir.push(IR::Stmt(Stmt::Ref {
        lhs: lhs.clone(),
        rhs: rhs.clone(),
    }));
}

fn parse_alias_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    let lhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    };
    reader.next(); // Var
    reader.next(); // =
    let rhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    };
    reader.next(); // Var
    reader.next(); // ;

    ir.push(IR::Stmt(Stmt::Alias {
        lhs: lhs.clone(),
        rhs: rhs.clone(),
    }));
}

fn parse_deref_read_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    let lhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    };
    reader.next(); // Var
    reader.next(); // =
    reader.next(); // *
    let rhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    };
    reader.next(); // Var
    reader.next(); // ;

    ir.push(IR::Stmt(Stmt::DerefRead {
        lhs: lhs.clone(),
        rhs: rhs.clone(),
    }));
}

fn parse_deref_write_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {
    reader.next(); // *
    let lhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    };
    reader.next(); // Var
    reader.next(); // =
    let rhs = match reader.peek() {
        Token::Var(ident) => ident,
        _ => unreachable!(),
    };
    reader.next(); // Var
    reader.next(); // ;

    ir.push(IR::Stmt(Stmt::DerefWrite {
        lhs: lhs.clone(),
        rhs: rhs.clone(),
    }));
}

fn parse_if_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {}

fn parse_while_stmt(reader: &mut TokenReader, ir: &mut Vec<IR>) {}

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
