extern crate endo;
use endo::parser;

#[test]
fn basic_syntax() {
    // Calculation
    assert_eq!(parser::parse_expr("(2 + 2)"), true);
    assert_eq!(parser::parse_expr("2 + 2)"), false);
    // Id and Members
    assert_eq!(parser::parse_expr("self.hello"), true);
    assert_eq!(parser::parse_expr("self.1"), false);
}
