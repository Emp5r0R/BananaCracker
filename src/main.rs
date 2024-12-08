use colored::Colorize; // For output color highlights
use sha2::{Digest, Sha256}; // For SHA256 hashing
use std::env; // For taking Arguments
use std::fs::File; // For file management
use std::io::{BufRead, BufReader}; // For file reading
use std::process::exit; // For closing the program

fn main() {
    // Argument management
    let argue: Vec<String> = env::args().collect();
    let version = env!("CARGO_PKG_VERSION");
    println!(
        " {} {} {}",
        "

 ┓   ┏┓┏━┏┓        ┓     
┏┣┓┏┓┏┛┗┓┣┓ ┏┏┓┏┓┏ ┃┏ ┏┓ ┏┓
┛┛┗┗┻┗━┗┛┗┛━┗┛ ┗┻┗ ┛┗ ┗  ┛ 
"
        .magenta(),
        version.magenta(),
        "https://github.com/apocalypsecracker/SHA256_cracker".magenta()
    );

    // Determine mode argument
    let mode_arg = if argue.len() == 4 {
        &argue[3]
    } else {
        "--multifast"
    };

    if argue.len() == 4 || argue.len() == 3 {
        // Display mode selected
        match (argue.len(), mode_arg) {
            (3, _) => println!(
                "{} No options selected, so Defaulting to {} mode",
                "[!]".yellow(),
                "Multi Fast".blue()
            ),

            (_, "--verbose") | (_, "-v") => {
                println!("{} Performing in {} mode", "[+]".blue(), "Verbose".blue())
            }
            (_, "--multifast") | (_, "-m") => {
                println!("{} Performing in {} mode", "[+]".blue(), "Muti Fast".blue())
            }
            _ => println!(
                "{} Invalid Option found, so Defaulting to {} mode",
                "[!]".yellow(),
                "Multi Fast".blue()
            ),
        };
        let req_hash = argue[1].trim().to_string(); // Input hash or path to hash file
        let pass_file_loc = &argue[2]; // Path to wordlist file

        // Determine if input is a hash file or a single hash
        let hashes_to_crack = if req_hash.contains(".txt") {
            // Read hashes from file
            let file_reader = file_pass(&req_hash).expect("Failed to read hash file");
            file_reader
                .lines()
                .filter_map(|line| line.ok()) // Collect valid lines
                .collect::<Vec<_>>() // Store all hashes
        } else {
            vec![req_hash] // Treat as a single hash
        };

        let mut total_hashes_cracked = 0; // Counter for successfully cracked hashes
        let mut read_error_count = 0; // Counter for ignored lines due to errors

        println!(
            "{} Specified wordlist path: {}\n",
            "[+]".blue(),
            pass_file_loc.blue()
        );
        // Loop over each hash to crack
        for hash in hashes_to_crack {
            if !hash.contains(".txt") && !hash.contains("/") {
                println!(
                    "{}. Attempting to crack hash: {}",
                    total_hashes_cracked + 1,
                    hash.blue()
                );
            } else {
                println!(
                    "{} Hash contains Invalid characters or unknown file location, check your inputs!",
                    "[x]".red()
                );
                exit(0);
            }

            // Open the wordlist file
            let file_reader = match file_pass(pass_file_loc) {
                Ok(reader) => reader,
                Err(err) => {
                    eprintln!("{} Failed to open wordlist file: {}", "[X]".red(), err);
                    continue; // If there's an error opening the wordlist, skip to the next hash
                }
            };

            let mut hash_cracked = false; // Flag to track if hash is cracked
            let mut attempt_count = 0;

            // Attempt to crack the hash using the wordlist
            for line in file_reader.lines() {
                match line {
                    Ok(password) => {
                        attempt_count += 1;
                        let trimmed_password = password.trim().to_owned().into_bytes();
                        let hashed_password = format!("{:x}", Sha256::digest(&trimmed_password));

                        // Check if the password hash matches
                        if hashed_password == hash {
                            println!(
                                "{} Password found! ----> {} <---- [Hash: {}] | Total Attempts: {}\n",
                                "[✔]".green(),
                                std::str::from_utf8(&trimmed_password).unwrap().green(),
                                hash.green(),
                                attempt_count - 1,
                            );
                            total_hashes_cracked += 1;
                            hash_cracked = true;

                            break; // If the hash is cracked, move to the next hash
                        }

                        // Verbose mode output
                        if mode_arg == "-v" || mode_arg == "--verbose" {
                            println!(
                                "{} Attempt {} | Password: {} | Hash: {}",
                                "[x]".red(),
                                attempt_count,
                                std::str::from_utf8(&trimmed_password).unwrap(),
                                hashed_password
                            );
                        }
                    }
                    Err(_) => {
                        read_error_count += 1; // Increase the error count if reading the line fails
                    }
                }
            }

            // If the hash was not cracked, print a failure message
            if !hash_cracked {
                println!(
                    "{} Failed to crack hash: {} | Total Failed attempts {}\n",
                    "[✖]".red(),
                    hash.red(),
                    attempt_count
                );
            }
        }

        // Final report on cracked hashes
        println!(
            "\n{} Completed! Total Hashes Cracked: {}, Ignored Words: {}",
            "[✔]".yellow(),
            total_hashes_cracked,
            read_error_count
        );
    } else {
        println!("{}", "Invalid Options!".red());
        println!(
            "Usage --> For Source : {} | --> For Binary: {} {}",
            "cargo run <hash> <Path_to_the_wordlist> <Mode>".blue(),
            "./sha256_cracker <hash> <Path_to_the_wordlist> <Mode>".blue(),
            "\nModes:\n -v,--verbose\n -m,--multifast".blue()
        );
        println!("{}","Example Usage: \nType 1 --> ./sha256_cracker e2c587ac5155e215da971f07fb1aba71788ef2fa423a7efb251267419e146521 /usr/share/wordlists/rockyou.txt -m \nType 2 --> ./sha256_cracker ebf13de02b7406fd14c97ec3d136e6b36a27b53dd327602e7ace6912ccbccfa9 /usr/share/wordlists/rockyou.txt --multifast \nType 3 --> ./sha256_cracker path/to/the/hashes.txt /usr/share/wordlists/rockyou.txt -m \nLikewise for Verbose mode use option -v or --verbose\n".blue());
        println!(
            "{} Tip: Multi Fast mode is significantly faster than Verbose mode",
            "[!]".yellow()
        );
        exit(1);
    }
}

// Custom error message for file reading
fn file_pass(path: &String) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let pass_file = File::open(path)?;
    Ok(BufReader::new(pass_file))
}
