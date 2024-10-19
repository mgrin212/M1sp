use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

pub fn assemble(name: &str, instructions: &[String]) -> Result<(), String> {
    let asm_file = format!("{}.s", name);
    let object_file = format!("{}.o", name);
    let binary_file = format!("{}.exe", name);

    // Write assembly to file
    let asm_content = instructions.join("\n");
    File::create(&asm_file)
        .and_then(|mut file| file.write_all(asm_content.as_bytes()))
        .map_err(|e| format!("Failed to write assembly file: {}", e))?;

    // Assemble
    let output = Command::new("nasm")
        .args(&["-felf64", &asm_file, "-o", &object_file])
        .output()
        .map_err(|e| format!("Failed to execute nasm: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "nasm failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Link
    let output = Command::new("ld")
        .args(&[&object_file, "-o", &binary_file])
        .output()
        .map_err(|e| format!("Failed to execute ld: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "ld failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Clean up temporary files
    fs::remove_file(&asm_file).map_err(|e| format!("Failed to remove assembly file: {}", e))?;
    fs::remove_file(&object_file).map_err(|e| format!("Failed to remove object file: {}", e))?;

    Ok(())
}
