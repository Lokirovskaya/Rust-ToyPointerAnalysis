use crate::lexer::Token;

#[derive(Debug)]
pub enum StmtKind {
    Ref,
    Alias,
    DerefRead,
    DerefWrite,
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub lhs: String,
    pub rhs: String,
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

pub fn parse(tokens: Vec<Token>) {
    let mut reader = TokenReader { tokens, i: 0 };
    parse_stmts(&mut reader)
}

fn parse_stmts(reader: &mut TokenReader) {

}

struct TokenReader {
    tokens: Vec<Token>,
    i: usize,
}

impl TokenReader {
    fn peek(&self) -> Token {
        return self.tokens[self.i];
    }

    fn next(&mut self) {
        self.i += 1;
    }
}

pub fn get_stmts(source: &str) -> Vec<Stmt> {
    let mut ans: Vec<Stmt> = Vec::new();
    let split = source.split('\n');

    for s in split {
        if s.trim().is_empty() {
            continue;
        }

        let mut is_deref_write = false;
        let mut it = s.as_bytes().iter().peekable();

        if let Some(symbol) = read_symbol(&mut it) {
            if symbol == b'*' {
                is_deref_write = true;
            }
        }

        let lhs = read_ident(&mut it).unwrap();

        assert_eq!(read_symbol(&mut it), Some(b'='));

        let kind = if is_deref_write {
            StmtKind::DerefWrite
        } else {
            match read_symbol(&mut it) {
                Some(b'*') => StmtKind::DerefRead,
                Some(b'&') => StmtKind::Ref,
                Some(c) => panic!("unexpected symbol {}", c),
                None => StmtKind::Alias,
            }
        };

        let rhs = read_ident(&mut it).unwrap();

        ans.push(Stmt { kind, lhs, rhs });
    }

    return ans;
}

fn read_symbol(it: &mut Peekable<Iter<u8>>) -> Option<u8> {
    skip_blank(it);
    let c = **it.peek()?;
    if c == b'*' || c == b'&' || c == b'=' {
        if let Some(c) = it.next() {
            return Some(*c);
        }
    }
    return None;
}

fn read_ident(it: &mut Peekable<Iter<u8>>) -> Option<String> {
    skip_blank(it);
    if it.peek()?.is_ascii_alphabetic() {
        let mut s = String::new();
        while let Some(c) = it.peek() {
            if !(**c).is_ascii_alphabetic() {
                break;
            }
            s.push(**c as char);
            it.next();
        }
        return Some(s);
    }
    return None;
}

fn skip_blank(it: &mut Peekable<Iter<u8>>) {
    while let Some(c) = it.peek() {
        if (**c).is_ascii_whitespace() {
            it.next();
        } else {
            return;
        }
    }
}
