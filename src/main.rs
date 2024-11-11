mod asm;
mod assemble;
pub mod ast;
mod compile;
pub mod grammar;
mod utils;
// TODO: Add interpreter module
mod interpreter;

use asm::string_of_directive;
use ast::*;
use compile::compile;
use interpreter::interpret;
// TODO: Import interpreter when implemented
// use interpret::interpret;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug, Clone)]
enum Mode {
    Compile,
    Interpret,
}

fn read_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse(contents: &str) -> Result<Program, String> {
    grammar::ProgramParser::new()
        .parse(contents)
        .map_err(|e| e.to_string())
}

fn compile_ast(program: Program) -> Result<String, String> {
    let mut output = String::new();
    let directives = compile(program);
    for directive in directives {
        output.push_str(&format!("{}\n", string_of_directive(&directive)));
    }
    Ok(output)
}

fn interpret_ast(_program: Program) -> Result<String, String> {
    let output = interpret(_program);
    match output {
        Ok(v) => Ok(format!("{:?}", v)),
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn process_file(path: &Path, mode: Mode) -> Result<(), String> {
    let contents = read_file(path).map_err(|e| format!("Error reading file: {}", e))?;
    let cleaned = contents.replace('\n', "");

    // Parse into AST first
    let ast = parse(&cleaned)?;

    // Then either compile or interpret based on mode
    let output = match mode {
        Mode::Compile => compile_ast(ast),
        Mode::Interpret => interpret_ast(ast),
    }?;

    println!("{}", output);
    Ok(())
}

fn process_directory(dir: &Path, mode: Mode) -> Result<(), String> {
    if !dir.exists() {
        return Err("Directory does not exist".to_string());
    }

    let entries = fs::read_dir(dir).map_err(|e| format!("Error reading directory: {}", e))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("lisp") {
            match process_file(&path, mode.clone()) {
                Ok(_) => (),
                Err(e) => eprintln!("Error processing file {:?}: {}", path, e),
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    // Default to compile mode if no flag is provided
    let mut mode = Mode::Compile;
    let mut skip_next = false;
    let mut input_file: Option<String> = None;

    // Parse command line arguments
    let mut i = 1;
    while i < args.len() {
        if skip_next {
            skip_next = false;
            i += 1;
            continue;
        }

        match args[i].as_str() {
            "-c" => mode = Mode::Compile,
            "-i" => mode = Mode::Interpret,
            arg if !arg.starts_with("-") => {
                input_file = Some(arg.to_string());
                break;
            }
            _ => return Err(format!("Unknown option: {}", args[i])),
        }
        i += 1;
    }

    // Process either specific file or samples directory
    match input_file {
        Some(file) => process_file(Path::new(&file), mode),
        None => process_directory(Path::new("samples"), mode),
    }
}
