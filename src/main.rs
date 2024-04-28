use std::fs;
use std::process::Command;
use colored::Colorize;

fn runSection(code: &str){
    let mut gcccmd: Vec<String> = vec!["gcc".to_string()];

    for line in code.split('\n') {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if let Some(token) = tokens.get(0) {
            match *token {
                "ObjectOut" => {
                    gcccmd.push("-c".to_string());
                }
                "Source" => {
                    if let Some(lib) = tokens.get(1) {
                        gcccmd.push(lib.to_string());
                    }
                }
                "AddExe" => {
                    if let Some(lib) = tokens.get(1) {
                        gcccmd.push("-o".to_string());
                        gcccmd.push(lib.to_string());
                    }
                }
                "LinkDir" => {
                    if let Some(lib) = tokens.get(1) {
                        gcccmd.push(format!("{}{}", "-L".to_string(), lib.to_string()));
                    }
                }
                "IncludeDir" => {
                    if let Some(lib) = tokens.get(1) {
                        gcccmd.push(format!("{}{}", "-I".to_string(), lib.to_string()));
                    }
                }
                "Link" => {
                    if let Some(lib) = tokens.get(1) {
                        gcccmd.push(format!("{}{}", "-l".to_string(), lib.to_string()));
                    }
                }
                _ => {}
            }
        }
    }

    // Execute the command
    let output = Command::new("gcc")
        .args(&gcccmd[1..]) // Exclude "gcc" from arguments
        .output()
        .expect("Failed to execute command");
    println!("{}", format!("Executed Command: {:?} After Preferences", gcccmd.join(" ")).yellow().bold());
    println!("status: {}", output.status);
    println!("{}", format!("{} {}", "[~] stdout:", String::from_utf8_lossy(&output.stdout)).yellow().bold());
    println!("{}", format!("{} {}", "[-] stderr:", String::from_utf8_lossy(&output.stderr)).red().bold());
}


fn main() {
    let binding = fs::read_to_string("cbuild").expect("No cbuild-file");
    let sections = binding.as_str().split("==~==");
    for sectionSplitS in sections{
        let section = format!("{}", sectionSplitS);
        runSection(&section.as_str());
    }
}
