/* Imports */
use egg::*;
use argparse::{ArgumentParser, StoreTrue, Store};
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
    
    // Default command line arguments
    let mut tree_depth: u32 = 3;
    let mut simple_expr: bool = false; 
    let mut visualize: bool = false; 
    let mut init_csv: bool = false; 
    let mut csv_path_str:String = "target/performance.csv".to_string();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("minimax-egg's command-line arguments.");
        ap.refer(&mut tree_depth)
            .add_option(&["-d", "--tree_depth"], Store,
            "The depth of the tree");
        ap.refer(&mut simple_expr)
            .add_option(&["-s", "--simple_expr"], StoreTrue,
            "Uses a simple RecExpr when true");
        ap.refer(&mut visualize)
            .add_option(&["-v", "--visualize"], StoreTrue,
            "Visualizes the egraphs when true");
        ap.refer(&mut init_csv)
            .add_option(&["-i", "--init_csv"], StoreTrue,
            "Initialize the csv when when true");
        ap.refer(&mut csv_path_str)
            .add_option(&["-p", "--csv_path"], Store,
            "Path to the performance csv when true");
        ap.parse_args_or_exit();
    }
    let csv_path = Path::new(&csv_path_str);

    // Print tree depth
    println!("tree_depth = {}", tree_depth);

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