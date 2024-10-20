mod asm;
mod assemble;
pub mod ast;
mod compile;
pub mod grammar;
mod utils;
use asm::string_of_directive;
use ast::*;
use compile::compile;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;
use std::ptr::hash;

fn read_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_and_compile(contents: &str) -> Result<String, String> {
    let parsed = grammar::ExprParser::new().parse(contents);
    let mut output = String::new();
    if let Ok(expr_vec) = parsed {
        let directives = compile(*expr_vec);
        for directive in directives {
            output.push_str(&format!("{}\n", string_of_directive(&directive)));
        }
        Ok(output)
    } else {
        Err("Failed to parse input".to_string())
    }
}

fn parse_file(path: &Path) -> Result<(), String> {
    let contents = read_file(path).map_err(|e| format!("Error reading file: {}", e))?;
    let cleaned = contents.replace('\n', "");
    let output = parse_and_compile(&cleaned)?;
    println!("{}", output);
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // If an argument is provided, compile the specified file
        let input_file = Path::new(&args[1]);
        return parse_file(input_file);
    }

    // If no argument is provided, process all .lisp files in the samples directory
    let samples_dir = Path::new("samples");
    if !samples_dir.exists() {
        return Err("Samples directory does not exist".to_string());
    }

    let entries =
        fs::read_dir(samples_dir).map_err(|e| format!("Error reading directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("lisp") {
            match parse_file(&path) {
                Ok(_) => (),
                Err(e) => eprintln!("Error parsing file {:?}: {}", path, e),
            }
        }
    }

    Ok(())
}
