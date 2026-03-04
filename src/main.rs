use rs_clp::frontend::lexer::lexer_rules;

fn main() {
    let input = "10 + 20 + 30";

    let lexer_rules = lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules, &input).unwrap();

    for lexeme in lexemes {
        println!("{}", *lexeme);
    }

    println!("Hello, world!");
}
