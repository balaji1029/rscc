use santiago::grammar::Associativity;
use santiago::grammar::Grammar;

/**
 * The function that returns the grammar of the language.
 */
pub fn grammar() -> Grammar<()> {
    santiago::grammar!(
        // "program" => rules "global_decl_stmt_list" "func_def_list";
        "program" => rules "func_def_list";

        // "global_decl_stmt_list" => rules "global_decl_stmt_list" "func_decl";
        // "global_decl_stmt_list" => rules "global_decl_stmt_list" "var_decl_stmt";
        // "global_decl_stmt_list" => rules "var_decl_stmt";
        // "global_decl_stmt_list" => rules "func_decl";

        // "func_decl" => rules "func_header" "LEFT_ROUND_BRACKET" "formal_param_list" "RIGHT_ROUND_BRACKET" "SEMICOLON";

        "func_def_list" => rules "func_def_list" "func_def";
        "func_def_list" => rules "func_def";

        "func_def" => rules "func_header" "LEFT_ROUND_BRACKET" "formal_param_list" "RIGHT_ROUND_BRACKET" "LEFT_CURLY_BRACKET" "stmt_list" "RIGHT_CURLY_BRACKET";

        "func_def" => rules "func_header" "LEFT_ROUND_BRACKET" "RIGHT_ROUND_BRACKET" "LEFT_CURLY_BRACKET" "stmt_list" "RIGHT_CURLY_BRACKET";

        "func_header" => rules "named_type" "ID";

        "formal_param_list" => rules "formal_param_list" "COMMA" "formal_param";
        "formal_param_list" => rules "formal_param";

        "formal_param" => rules "param_type" "ID";

        "stmt_list" => rules "stmt_list" "stmt";
        "stmt_list" => rules "stmt";

        "stmt" => rules "assignment_stmt";

        "assignment_stmt" => rules "ID" "ASSIGN" "expr" "SEMICOLON";

        "primary_expr" => rules "ID";
        "primary_expr" => rules "INTEGER_CONSTANT";
        "primary_expr" => rules "FLOAT_CONSTANT";
        "primary_expr" => rules "STRING_CONSTANT";
        "primary_expr" => rules "LEFT_ROUND_BRACKET" "expr" "RIGHT_ROUND_BRACKET";

        "unary_expr" => rules "MINUS" "unary_expr";
        "unary_expr" => rules "primary_expr";

        "mul_expr" => rules "mul_expr" "MULT" "unary_expr";
        "mul_expr" => rules "mul_expr" "DIV"  "unary_expr";
        "mul_expr" => rules "unary_expr";

        "add_expr" => rules "add_expr" "PLUS"  "mul_expr";
        "add_expr" => rules "add_expr" "MINUS" "mul_expr";
        "add_expr" => rules "mul_expr";

        "expr" => rules "add_expr";

        "param_type" => rules "INT";
        "param_type" => rules "FLOAT";
        "param_type" => rules "STRING";
        "param_type" => rules "BOOL";

        "named_type" => rules "INT";
        "named_type" => rules "FLOAT";
        "named_type" => rules "STRING";
        "named_type" => rules "BOOL";
        "named_type" => rules "VOID";

        "ID" => lexemes "ID";
        "COMMA" => lexemes "COMMA";
        "SEMICOLON" => lexemes "SEMICOLON";
        "ASSIGN" => lexemes "ASSIGN";
        "PLUS" => lexemes "PLUS";
        "MINUS" => lexemes "MINUS";
        "MULT" => lexemes "MULT";
        "DIV" => lexemes "DIV";
        "LEFT_ROUND_BRACKET" => lexemes "LEFT_ROUND_BRACKET";
        "RIGHT_ROUND_BRACKET" => lexemes "RIGHT_ROUND_BRACKET";
        "LEFT_CURLY_BRACKET" => lexemes "LEFT_CURLY_BRACKET";
        "RIGHT_CURLY_BRACKET" => lexemes "RIGHT_CURLY_BRACKET";
        "INTEGER_CONSTANT" => lexemes "INTEGER_CONSTANT";
        "FLOAT_CONSTANT" => lexemes "FLOAT_CONSTANT";
        "STRING_CONSTANT" => lexemes "STRING_CONSTANT";

        "INT" => lexemes "INT";
        "FLOAT" => lexemes "FLOAT";
        "STRING" => lexemes "STRING";
        "BOOL" => lexemes "BOOL";
        "VOID" => lexemes "VOID";

        Associativity::Left => rules "PLUS" "MINUS";
        Associativity::Left => rules "MULT" "DIV";
    )
}
