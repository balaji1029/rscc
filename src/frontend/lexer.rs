use santiago::lexer::LexerRules;

/**
 * This function defines the lexer rules for the rscc language.
 */
pub fn lexer_rules() -> LexerRules {
    santiago::lexer_rules!(
        // Integer constants, float constants, and string constants
        "DEFAULT" | "INTEGER_CONSTANT" = pattern r"[0-9]+";
        "DEFAULT" | "FLOAT_CONSTANT" = pattern r"([0-9]+\.)|([0-9]*\.[0-9]+)";
        "DEFAULT" | "STRING_CONSTANT" = pattern r#""([^"\\]|\\.)*""#;

        // Keywords

        // Types
        "DEFAULT" | "INT" = string "int";
        "DEFAULT" | "FLOAT" = string "float";
        "DEFAULT" | "STRING" = string "string";
        "DEFAULT" | "BOOL" = string "bool";
        "DEFAULT" | "VOID" = string "void";

        // Identifiers
        "DEFAULT" | "ID" = pattern r"[a-zA-Z_]*";

        // Assignment
        "DEFAULT" | "ASSIGN" = string "=";

        // Operators
        "DEFAULT" | "PLUS" = string "+";
        "DEFAULT" | "MINUS" = string "-";
        "DEFAULT" | "MULT" = string "*";
        "DEFAULT" | "DIV" = string "/";

        // Semicolon
        "DEFAULT" | "SEMICOLON" = string ";";

        // Parentheses and curly brackets
        "DEFAULT" | "LEFT_CURLY_BRACKET" = string "{";
        "DEFAULT" | "RIGHT_CURLY_BRACKET" = string "}";
        "DEFAULT" | "LEFT_ROUND_BRACKET" = string "(";
        "DEFAULT" | "RIGHT_ROUND_BRACKET" = string ")";

        // Comments and whitespace
        "DEFAULT" | "COMMENT" = pattern "//.*" => |lexer| lexer.skip();
        "DEFAULT" | "WS" = pattern r"\s" => |lexer| lexer.skip();

        // Catch-all rule for unexpected characters
        "DEFAULT" | "ERROR" = pattern r"." => |lexer| {
            panic!("\trscc: Unexpected character '{}'", lexer.matched());
        };
    )
}
