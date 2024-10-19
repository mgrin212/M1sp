mod asm;
mod assemble;
mod ast;
mod compile;
use asm::string_of_directive;
use ast::parse;
use compile::compile;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

fn read_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_file(path: &Path) -> Result<(), String> {
    let contents = read_file(path).map_err(|e| format!("Error reading file: {}", e))?;
    let parsed = parse(&contents);
    if let Ok(expr_vec) = parsed {
        for expr in expr_vec {
            let directives = compile(expr);
            for directive in directives {
                println!("{}", string_of_directive(&directive));
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), String> {
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
