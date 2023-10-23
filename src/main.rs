/* Imports */
use egg::*;
use std::env;
use std::path::Path;
use std::time::Instant;
use std::error::Error;
use csv::Writer;
use std::fs::OpenOptions;

/* Modules */
mod utils;
mod tree;

/* Use's */
use crate::utils::*;
// use crate::utils::{simplify, SimpleLanguage};
use crate::tree::generate_tree;

/* main */
fn main() {
    
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();

    let tree_depth = args[1].parse::<u32>().expect("Failed to convert tree_depth to u32");
    let simple_expr = args[2].parse::<bool>().unwrap_or_else(|_| panic!("simple_expr must be 'true' or 'false'"));
    let visualize = args[3].parse::<bool>().unwrap_or_else(|_| panic!("visualize must be 'true' or 'false'"));
    let init_csv = args[4].parse::<bool>().unwrap_or_else(|_| panic!("init_csv must be 'true' or 'false'"));
    let csv_path = Path::new(&args[5]); //.parse::<Path>().unwrap_or_else(|_| panic!("init_csv must be 'true' or 'false'"));

    // Generating a massive tree 
    let now_tree = Instant::now();
    let expr: RecExpr<SimpleLanguage> = generate_tree(tree_depth, simple_expr); 
    let elapsed_tree = now_tree.elapsed();
    println!("Time to generate tree: {:.2?}", elapsed_tree);

    // Simplify the tree using egg
    let now_egg = Instant::now();
    let best_expr: RecExpr<SimpleLanguage> = simplify(expr.clone(), visualize);
    let elapsed_egg = now_egg.elapsed();
    println!("Time to simplify tree: {:.2?}", elapsed_egg);

    // Convert the durations to a float
    let elapsed_tree_f32 = elapsed_tree.as_secs_f32();
    let elapsed_egg_f32 = elapsed_egg.as_secs_f32();

    // Write the elasped times to a csv file
    if init_csv { initialize_csv(csv_path).unwrap(); }
    // initialize_csv();
    append_csv(csv_path, tree_depth, elapsed_tree_f32, elapsed_egg_f32).unwrap();

    // println!("input  expression: {}", expr);
    println!("output expression: {}", best_expr);
}

fn initialize_csv(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;
    wtr.write_record(&["tree_depth", "generate_tree_secs", "egg_time_secs"])?;
    wtr.flush()?;
    Ok(())
}

fn append_csv(path: &Path, tree_depth: u32, tree_time: f32, egg_time: f32) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .open(path)
                                    .unwrap();
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&[tree_depth.to_string(), tree_time.to_string(), egg_time.to_string()])?;
    wtr.flush()?;
    Ok(())
}