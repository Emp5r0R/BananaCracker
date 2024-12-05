use colored::Colorize;
use sha2::{Digest, Sha256}; // For SHA256 hashing
use std::env;
use std::fs::File; // For file management
use std::io::{BufRead, BufReader}; // For file reading
use std::process::exit;

fn main() {
    // Arguments management
    let argue: Vec<String> = env::args().collect();

    if argue.len() == 4 {
        let req_hash = &argue[1];
        let pass_file_loc = &argue[2];
        let cust_option = &argue[3];
        let mut current_state = 1;
        let mut read_error_count = 0;

        println!(
            "{} Process started: Attempting to crack the hash: {}",
            "[+]".blue(),
            req_hash
        );
        println!(
            "{} Specified wordlist path: {}",
            "[+]".blue(),
            pass_file_loc
        );

        // File management
        let file_reader = match file_pass(pass_file_loc) {
            Ok(reader) => reader,
            Err(err) => {
                eprintln!("{} Failed to open file: {}", "[X]".red(), err);
                exit(1);
            }
        };

        // Main loop
        for line in file_reader.lines() {
            match line {
                // Handling the errors
                Ok(contents) => {
                    let cont_pass = contents.trim().to_owned().into_bytes();
                    let cont_pass_hash = format!("{:x}", Sha256::digest(&cont_pass));
                    if &cont_pass_hash == req_hash {
                        println!(
                            "{} Password found [!] ----> {} <---- [!] Hash Cracked. Total attempts: {}. Found password's hash: {}",
                            "[✔]".green(),
                            std::str::from_utf8(&cont_pass).unwrap().green(),
                            current_state,
                            cont_pass_hash
                        );

                        exit(0);
                    }
                    current_state += 1;
                    if (cust_option == "-v") || (cust_option == "--verbose") {
                        println!(
                            "{} Current Attempt: [{}], Current password: {} ----> {}",
                            "[✖]".red(),
                            current_state,
                            std::str::from_utf8(&cont_pass).unwrap().red(),
                            cont_pass_hash
                        );
                    }
                }
                Err(_) => {
                    read_error_count += 1;
                }
            }
        }
        println!(
            "{} Password hash not found! Total read errors: {}. Advised to try a different wordlist!",
            "[X]".red(),
            read_error_count
        );
    } else {
        println!("{}", "Missing Options!".red());
        println!(
            "{} {} {} {} {}",
            "Usage --> For Raw Program:",
            "cargo run <hash> <Path_to_the_wordlist> <Mode>".blue(),
            "| --> For Binary:",
            "./sha256_cracker <hash> <Path_to_the_wordlist> <Mode>".blue(),
            "\nModes:\n -v,--verbose\n -n,--normal".blue()
        );
        exit(1);
    }
}

// Custom error message for file reading
fn file_pass(path: &String) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let pass_file = File::open(path)?;
    Ok(BufReader::new(pass_file))
}
