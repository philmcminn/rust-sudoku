mod dlx;
mod matrix;
mod solver;
mod sudoku;

use std::fs;
use std::env;
use std::path::Path;
use std::process;
use std::time::Instant;

use crate::sudoku::Sudoku;

const ALL_STR: &str = "--all";
const USAGE_STR: &str = "Usage: sudoku filename|starting_configuration [-all]";
const FILENAME_PARAM: usize = 1;
const ALL_PARAM: usize = 2;

fn read_sudoku_str(args: &Vec<String>) -> String {
    // check for correct number of parameters
    if args.len() < FILENAME_PARAM + 1 {
        println!("{}", USAGE_STR);
        process::exit(1);
    }

    // check if the param is a file and read it in
    let param = &args[FILENAME_PARAM];
    if Path::new(param).is_file() {
        if let Ok(sudoku_str) = fs::read_to_string(param) {
            sudoku_str
        } else {
            println!("Could not read file {}", param);
            process::exit(1);
        }
    } else {
        String::from(param)
    }
}

fn terminate_on_first(args: &Vec<String>) -> bool {
    if args.len() >= ALL_PARAM + 1 {
        args[ALL_PARAM] != ALL_STR
    } else {
        true
    }
}

fn main() {
    let args = env::args().collect();
    let sudoku_str = &read_sudoku_str(&args);
    let sudoku = Sudoku::from(sudoku_str);

    println!("Initial Sudoku ({}/{}) is:\n{}",
             sudoku.num_completed_cells(),
             sudoku.num_cells(),
             sudoku);

    if sudoku.is_consistent() {
        // let start_time = Instant::now();
        // let solutions = solver::solve(&sudoku, terminate_on_first(&args));
        // let elapsed_time = start_time.elapsed();
        //
        // let num_solutions = solutions.len();
        // if num_solutions > 0 {
        //     let mut plural = "";
        //     if num_solutions > 1 {
        //         plural = "s";
        //     }
        //
        //     println!("\nFound {} solution{} in {:.1?}:", num_solutions, plural, elapsed_time);
        //     let mut count = 1;
        //     for solution in solutions {
        //         if num_solutions > 1 {
        //             println!("{}:", count);
        //             count += 1;
        //         }
        //         println!("{}", solution);
        //     }
        // } else {
        //     println!("This Sudoku is unsolvable!");
        // }
    } else {
        println!("Sudoku contains repeated numbers in rows, columns or blocks.");
    }
}