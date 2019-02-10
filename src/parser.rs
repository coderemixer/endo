lalrpop_mod!(pub syntax);

pub fn parse_expr(code: &str) -> bool {
    return syntax::ExprParser::new().parse(code).is_ok();
}
