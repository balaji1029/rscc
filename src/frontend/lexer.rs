use santiago::lexer::LexerRules;

pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        "DEFAULT" | "INTEGER_CONSTANT" = pattern r"[0-9]+";
        "DEFAULT" | "FLOAT_CONSTANT" = pattern r"([0-9]+\.)|([0-9]*\.[0-9]+)";
        "DEFAULT" | "STRING_CONSTANT" = pattern r#""([^"\\]|\\.)*""#;

        "DEFAULT" | "INT" = string "int";
        "DEFAULT" | "FLOAT" = string "float";
        "DEFAULT" | "STRING" = string "string";
        "DEFAULT" | "VOID" = string "void";

        "DEFAULT" | "NAME" = pattern r"[a-zA-Z_]*";

        "DEFAULT" | "PLUS" = string "+";
        "DEFAULT" | "MINUS" = string "-";
        "DEFAULT" | "MULT" = string "*";
        "DEFAULT" | "DIV" = string "/";

        "DEFAULT" | "SEMICOLON" = string ";";

        "DEFAULT" | "LEFT_CURLY_BRACKET" = string "{";
        "DEFAULT" | "RIGHT_CURLY_BRACKET" = string "}";
        "DEFAULT" | "LEFT_ROUND_BRACKET" = string "(";
        "DEFAULT" | "RIGHT_ROUND_BRACKET" = string ")";

        "DEFAULT" | "COMMENT" = pattern "//.*" => |lexer| lexer.skip();
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();

        "DEFAULT" | "ERROR" = pattern r"." => |lexer| {
            panic!("\trs-clp: Unexpected character '{}'", lexer.matched());
            // lexer.error("Unexpected character: " + {})
        };
    )
}
