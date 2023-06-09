use std::fs::File;
use std::io::Read;

mod lexer;
mod parser;
mod solver;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let tokens = lexer::get_tokens(&content);
    let ir_list = parser::parse(tokens);

    // parser::_print_ir(&ir_list);

    solver::solve(ir_list);
}
