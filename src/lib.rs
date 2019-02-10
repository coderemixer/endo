#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub syntax);

pub mod ast;
pub mod parser;

#[no_mangle]
pub extern "C" fn print_hello_from_rust() {
    println!("Hello from Rust");
}
