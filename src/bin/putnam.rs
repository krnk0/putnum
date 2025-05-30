use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;

use putnam::solve;
use putnam::parser::parse_and_convert;
use putnam::solver::dpll::SolveResult;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file.cnf> [--model]", args[0]);
        process::exit(1);
    }
    
    let filename = &args[1];
    let show_model = args.get(2).map_or(false, |arg| arg == "--model");
    
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            process::exit(1);
        }
    };
    
    let reader = BufReader::new(file);
    
    let (formula, num_vars) = match parse_and_convert(reader) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error parsing DIMACS file: {}", e);
            process::exit(1);
        }
    };
    
    match solve(&formula, num_vars) {
        SolveResult::Sat(model) => {
            println!("SAT");
            if show_model {
                print!("v ");
                for var in 0..num_vars {
                    match model.value(var) {
                        putnam::types::Val::True => print!("{} ", var + 1),
                        putnam::types::Val::False => print!("-{} ", var + 1),
                        putnam::types::Val::Undef => print!("{} ", var + 1), // default to true
                    }
                }
                println!("0");
            }
            process::exit(10);
        }
        SolveResult::Unsat => {
            println!("UNSAT");
            process::exit(20);
        }
    }
}