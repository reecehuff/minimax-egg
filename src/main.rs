/* Imports */
use egg::*;
use std::time::Instant;

/* Modules */
mod utils;
mod tree;

/* Use's */
use crate::utils::*;
// use crate::utils::{simplify, SimpleLanguage};
use crate::tree::generate_tree;

/* main */
fn main() {
    let tree_depth = 12; 
    let simple_expr = false; 
    let visualize = false; 

    let now = Instant::now();
    let expr: RecExpr<SimpleLanguage> = generate_tree(tree_depth, simple_expr); 
    let elapsed = now.elapsed();
    println!("Time to generate tree: {:.2?}", elapsed);


    let now = Instant::now();
    let best_expr: RecExpr<SimpleLanguage> = simplify(expr.clone(), visualize);
    let elapsed = now.elapsed();
    println!("Time to simplify tree: {:.2?}", elapsed);

    // println!("input  expression: {}", expr);
    println!("output expression: {}", best_expr);
}
