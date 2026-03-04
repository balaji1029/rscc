use santiago::grammar::Grammar;

/**
 * The function that returns the grammar of the language.
 */
pub fn _grammar() -> Grammar<()> {
    santiago::grammar!(
        "program" => rules "global_decl_stmt_list" "func_def_list";
        "program" => rules "func_def_list";

        "global_decl_stmt_list" => rules "global_decl_stmt_list" "func_decl";
        "global_decl_stmt_list" => rules "global_decl_stmt_list" "var_decl_stmt";
        "global_decl_stmt_list" => rules "var_decl_stmt";
        "global_decl_stmt_list" => rules "func_decl";

        "func_decl" => rules "func_header" "LEFT_ROUND_BRACKET" "formal_param_list" "RIGHT_ROUND_BRACKET" "SEMICOLON";

        "func_def_list" => rules "func_def_list" "func_def";
        "func_def_list" => rules "func_def";

        "named_type" => lexemes "INT" "FLOAT" "STRING" "BOOL";
    )
}