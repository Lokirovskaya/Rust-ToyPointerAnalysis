#[derive(Debug)]
pub enum Token {
    Var(String),
    Ampersand,
    Star,
    Semicolon,
    Assign,
    LeftBrace,
    RightBrace,
    Sharp,
    If,
    Else,
    While,
}

pub fn get_tokens(source: &str) -> Vec<Token> {
    let mut ans = Vec::<Token>::new();

    let bytes = source.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let c = bytes[i];
        match c {
            b'&' => ans.push(Token::Ampersand),
            b'*' => ans.push(Token::Star),
            b';' => ans.push(Token::Semicolon),
            b'=' => ans.push(Token::Assign),
            b'{' => ans.push(Token::LeftBrace),
            b'}' => ans.push(Token::RightBrace),
            b'#' => ans.push(Token::Sharp),
            _ => {
                if i < bytes.len() && bytes[i].is_ascii_whitespace() {
                    i += 1;
                    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                        i += 1;
                    }
                    i -= 1;
                }

                if i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    let mut ident = String::new();
                    while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_')
                    {
                        ident.push(bytes[i] as char);
                        i += 1;
                    }
                    match ident.as_str() {
                        "while" => ans.push(Token::While),
                        "if" => ans.push(Token::If),
                        "else" => ans.push(Token::Else),
                        _ => ans.push(Token::Var(ident)),
                    }
                    i -= 1;
                }
            }
        }
        i += 1;
    }

    return ans;
}
