use rs_clp::frontend::lexer::lexer_rules;

fn main() {
    // let input = "10. + 20.0 + .30\n30 - 20";

    let input = std::fs::read_to_string("examples/test1.lc").expect("File error");

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).expect("Error in lexing");

    for lexeme in lexemes {
        println!("Lexeme: {}, Value: {}, Line: {}", lexeme, lexeme.raw, lexeme.position.line);
        // println!(" {}", lexeme.raw);
    }

    // println!("Hello, world!");
}
