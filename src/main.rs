/* Imports */
use egg::*;

/* Modules & scripts */
mod utils;
// use crate::utils::{generate_tree, simplify, SimpleLanguage};
use crate::utils::*;


/* main */
fn main() {
    let expr: RecExpr<SimpleLanguage> = generate_tree(); 
    println!("input  expression: {}", expr);
    let best_expr: RecExpr<SimpleLanguage> = simplify(expr);
    println!("output expression: {}", best_expr);
}
