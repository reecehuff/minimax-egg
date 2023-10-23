/* Imports */
use egg::*;

/* Use's */
use crate::utils::*;
// use rand::thread_rng;
// use rand::rngs::StdRng;
// use rand::seq::SliceRandom;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;


pub fn generate_tree(depth: u32, simple: bool) -> RecExpr<SimpleLanguage> {

    /* Define a simple expression for testing */  

    // let expr: RecExpr<SimpleLanguage> = "(white (black -3 -1) (black 6 -4) )".parse().unwrap();
    if simple == true {
        // let expr: RecExpr<SimpleLanguage> = "(white (black -3 -1) (black 6 -4) )".parse().unwrap();
        let expr: RecExpr<SimpleLanguage> = "(white (black (white -4 2) (white -10 10)) (black (white 6 -2) (white 3 -5)) )".parse().unwrap();
        return expr;
    }
    
    /* Define a very large expression for evaluation */  

    // Begin by defining the end leaves of the tree
    let num_leaves = i32::pow(2, depth); 
    let mut rng = ChaCha8Rng::seed_from_u64(2);
    let mut evaluations: Vec<i32> = (-(num_leaves/2)..(num_leaves/2)).collect();
    evaluations.shuffle(&mut rng);
    println!("num_leaves = {}", evaluations.len());

    // Define our starting string expr
    // We will add new branches to this string
    // NOTE: The starting evals don't matter because we will replace them 
    let mut str_expr = format!("( white {} {} )", 51, 23);

    // Start by adding black branches to the tree
    let mut black_or_white = "black";

    // Grow the tree
    for _ in 1..depth {
        
        // Add branches to the tree
        str_expr = add_branches(&mut str_expr, black_or_white, &evaluations); 
        
        // Alternate between white and black branches
        if black_or_white == "white" {
            black_or_white = "black";
        } else {
            black_or_white = "white";
        }
    }

    // Convert the string to a RecExpr
    let expr: RecExpr<SimpleLanguage> = str_expr.parse().unwrap();

    // Return the expression 
    expr

}

fn add_branches(str_expr: &mut String, bw: &'static str, indices: &Vec<i32>) -> String {

    // Unravel the indices of the evaluations vector
    let mut c1 = 0;
    let mut c2 = 1; 
    
    // Define parts as a vector of strings
    // We will concatenate to this string to create our new str_expr
    let mut parts: Vec<String> = Vec::new();

    // Define the new branch of the tree as mutable 
    let mut new_branch: String;
    
    // Split the input str_expr by the whitespace and loop through 
    for spl in str_expr.split_whitespace() {
        // If the particular split is an integer, then replace with new branch 
        if spl.parse::<i32>().is_ok() {

            // Add indices to the new branch of the tree and push it to parts
            new_branch = format!("( {} {} {} )", bw, indices[c1], indices[c2]);
            parts.push(new_branch.to_string());

            // Update the indices
            c1 += 2; c2 += 2; 

        // If not an integer, then add to the parts Vec
        } else {
            parts.push(spl.to_string());
        }
    }
    // Join the Vec of String's into one string and return it 
    parts.join(" ")

}