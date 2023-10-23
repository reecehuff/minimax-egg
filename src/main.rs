/* Imports */
use egg::*;

/* Modules */
mod utils;
mod tree;

/* Use's */
use crate::utils::*;
// use crate::utils::{simplify, SimpleLanguage};
use crate::tree::generate_tree;

/* main */
fn main() {
    let tree_depth = 3; 
    let expr: RecExpr<SimpleLanguage> = generate_tree(tree_depth); 
    println!("input  expression: {}", expr);
    let best_expr: RecExpr<SimpleLanguage> = simplify(expr);
    println!("output expression: {}", best_expr);
}
