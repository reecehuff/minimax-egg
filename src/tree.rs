/* Imports */
use egg::*;

/* Use's */
use crate::utils::*;
use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn generate_tree(depth: u32) -> RecExpr<SimpleLanguage> {
    // Define a simple expression for testing 
    // let expr: RecExpr<SimpleLanguage> = "(white (black -3 -1) (black 6 -4) )".parse().unwrap();
    let expr: RecExpr<SimpleLanguage> = "(white (black (white -4 2) (white -10 10)) (black (white 6 -2) (white 3 -5)) )".parse().unwrap();
    
    // Create a very large expression
    let num_leaves = i32::pow(2, depth); 
    let mut evaluations: Vec<i32> = (0..num_leaves).collect();
    evaluations.shuffle(&mut thread_rng());
    println!("{:?}", evaluations);

    let mut str_expr = format!("( white {} {} )", evaluations[0], evaluations[1]);
    println!("old str_expr: {:?}", str_expr);
    println!("new str_expr: {:?}", str_expr);

    let mut black_or_white = "black";
    for chunk in evaluations.chunks(4) {
        println!("running with {}", black_or_white);
        str_expr = add_leaves(&mut str_expr, black_or_white, chunk); 
        // panic!();
        if let [a, b, c, d] = &chunk[..] {
            // println!("a: {}", a);
            // println!("b: {}", b);
            // println!("c: {}", c);
            // println!("d: {}", d);
            let slice = [evaluations[*a as usize], evaluations[*b as usize], evaluations[*c as usize], evaluations[*d as usize]];
            println!("slice : {:?}", slice);
        }
        if black_or_white == "white" {
            black_or_white = "black";
        } else {
            black_or_white = "white";
        }
    }
    println!("new str_expr: {:?}", str_expr);
    panic!();


    // let indices = str_expr.char_indices();
    // for (index, character) in indices {
    //     println!("index    : {}", index);
    //     println!("character: {}", character);
    //     println!("is_digit : {}", character.is_digit(10));
    // }

    for spl in str_expr.split_whitespace() {
        println!("spl      : {}", spl);
        println!("is_digit : {}", spl.parse::<i32>().is_ok());
    }

    let mut parts: Vec<String> = Vec::new();
    let mut new_integer = format!("( black {} {} )", evaluations[2], evaluations[3]);
    

    for spl in str_expr.split_whitespace() {
        println!("{:?}", spl);
        if spl.parse::<i32>().is_ok() {
            parts.push(new_integer.to_string());
            new_integer = format!("( black {} {} )", evaluations[4], evaluations[5]);
        } else {
            parts.push(spl.to_string());
        }
    }

    str_expr = parts.join(" ");

    println!("{}", str_expr);

    // panic!();

    let expr: RecExpr<SimpleLanguage> = str_expr.parse().unwrap();

    // Return the expression 
    expr
}

fn add_leaves(str_expr: &mut String, bw: &'static str, indices: &[i32]) -> String {

    for spl in str_expr.split_whitespace() {
        println!("spl      : {}", spl);
        println!("is_digit : {}", spl.parse::<i32>().is_ok());
    }

    if let [a,b,c,d] = &indices[..] {
        
        let mut parts: Vec<String> = Vec::new();
        let mut new_integer = format!("( {} {} {} )", bw, a, b);
        

        for spl in str_expr.split_whitespace() {
            println!("{:?}", spl);
            if spl.parse::<i32>().is_ok() {
                parts.push(new_integer.to_string());
                new_integer = format!("( {} {} {} )", bw, c, d);
            } else {
                parts.push(spl.to_string());
            }
        }
        return parts.join(" ");
    }
    str_expr.to_string()
}