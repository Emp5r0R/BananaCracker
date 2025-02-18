use colored::Colorize;
use lazy_static::lazy_static;
use md5::Md5;
use rayon::prelude::*;
use regex::Regex;
use sha2::{Digest, Sha256};
use sha3::{Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::sync::atomic::{AtomicUsize, Ordering}; // Import AtomicUsize
use std::sync::Arc;

lazy_static! {
    static ref MD5_REGEX: Regex = Regex::new(r"^[a-fA-F0-9]{32}$").unwrap();
    static ref SHA256_REGEX: Regex = Regex::new(r"^[a-fA-F0-9]{64}$").unwrap();
    static ref SHA3_256_REGEX: Regex = Regex::new(r"^[a-fA-F0-9]{64}$").unwrap();
    static ref SHA3_384_REGEX: Regex = Regex::new(r"^[a-fA-F0-9]{96}$").unwrap();
    static ref SHA3_224_REGEX: Regex = Regex::new(r"^[a-fA-F0-9]{56}$").unwrap();
    static ref SHA3_512_REGEX: Regex = Regex::new(r"^[a-fA-F0-9]{128}$").unwrap();
}

fn hash_checker(hash: &str) -> String {
    match hash.len() {
        32 if MD5_REGEX.is_match(hash) => "MD5".to_string(),
        64 if SHA256_REGEX.is_match(hash) => "SHA256".to_string(),
        64 if SHA3_256_REGEX.is_match(hash) => "SHA3-256".to_string(),
        96 if SHA3_384_REGEX.is_match(hash) => "SHA3-384".to_string(),
        56 if SHA3_224_REGEX.is_match(hash) => "SHA3-224".to_string(),
        128 if SHA3_512_REGEX.is_match(hash) => "SHA3-512".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn file_pass(path: &str) -> Result<BufReader<File>, std::io::Error> {
    let pass_file = File::open(path)?;
    Ok(BufReader::new(pass_file))
}

fn print_usage() {
    println!("{}", "Invalid Options!".red());
    println!(
        "Usage --> For Source : {} | --> For Binary: {} {}",
        "cargo run <hash> <Path_to_the_wordlist> <Mode>".blue(),
        "./BananaCracker <hash> <Path_to_the_wordlist> <Mode>".blue(),
        "\nModules:\n -s2, --sha256\n -m,  --md5\n Modes:\n -ad, --automatic-detection\n -v,  --verbose"
            .blue()
    );
    println!("{}","Example Usage: \nType 1 --> ./BananaCracker f25a2fc72690b780b2a14e140ef6a9e0 /usr/share/wordlists/rockyou.txt -m \nType 2 --> ./BananaCracker ebf13de02b7406fd14c97ec3d136e6b36a27b53dd327602e7ace6912ccbccfa9 /usr/share/wordlists/rockyou.txt -s2 \nType 3 --> ./BananaCracker path/to/the/hashes.txt /usr/share/wordlists/rockyou.txt -ad \nLikewise for Verbose mode use option -v or --verbose\n".blue());
    println!(
        "{} Fact: According to some estimates, more than 100 billion bananas are consumed globally each year",
        "[!]".yellow()
    );
}

fn main() {
    let argue: Vec<String> = env::args().collect();
    let version = env!("CARGO_PKG_VERSION");
    println!(
        " {} {} {}\n",
        "
 
▗▄▄▖ ▗▞▀▜▌▄▄▄▄  ▗▞▀▜▌▄▄▄▄  ▗▞▀▜▌┏┓      ┓              
▐▌ ▐▌▝▚▄▟▌█   █ ▝▚▄▟▌█   █ ▝▚▄▟▌┃ ┏┓┏┓┏ ┃┏ ┏┓┏┓           
▐▛▀▚▖     █   █      █   █      ┗┛┛ ┗┻┗ ┛┗ ┗ ┛          
▐▙▄▞▘  

"
        .magenta(),
        version.magenta(),
        "https://github.com/Emp5r0R/BananaCracker".magenta()
    );

    let mode_arg = if argue.len() >= 4 { &argue[3] } else { "" };
    let mode_opt = if argue.len() >= 5 { &argue[4] } else { "" };

    let verbose_mode = matches!(mode_opt, "-v" | "--verbose");

    if argue.len() < 4 {
        print_usage();
        exit(1);
    }

    match mode_arg {
        "--md5" | "-m" => println!("{} Selected Module:{}", "[+]".blue(), "MD5".blue()),
        "--sha256" | "-s2" => println!("{} Selected Module:{}", "[+]".blue(), "SHA256".blue()),
        "--sha3-256" | "-s3-256" => {
            println!("{} Selected Module:{}", "[+]".blue(), "SHA3-256".blue())
        }
        "--sha3-384" | "-s3" => println!("{} Selected Module:{}", "[+]".blue(), "SHA3-384".blue()),
        "--sha3-224" | "-s3-224" => {
            println!("{} Selected Module:{}", "[+]".blue(), "SHA3-224".blue())
        }
        "--sha3-512" | "-s3-5" => {
            println!("{} Selected Module:{}", "[+]".blue(), "SHA3-512".blue())
        }
        "--automatic-detection" | "-ad" => println!(
            "{} Performing in {} mode",
            "[+]".blue(),
            "Automatic Detection".blue()
        ),
        _ => {
            println!(
                "{} Invalid Mode or Module were specified, Exiting!",
                "[!]".yellow()
            );
            exit(3);
        }
    }

    if verbose_mode {
        println!("{} Verbose Mode: {}", "[+]".blue(), "True".blue());
    } else {
        println!("{} Verbose Mode: {}", "[-]".yellow(), "False".yellow());
    }

    let req_hash = argue[1].trim();

    if req_hash.contains(".txt") && verbose_mode {
        println!(
            "{} Failed: Verbose cannot be performed when declaring text file.",
            "[x]".red()
        );
        print_usage();
        exit(4);
    }

    let pass_file_loc = &argue[2];

    let hashes_to_crack = if req_hash.contains(".txt") {
        let file_reader = match file_pass(req_hash) {
            Ok(reader) => reader,
            Err(err) => {
                eprintln!(
                    "{} Failed to open hash file '{}': {}",
                    "[x]".red(),
                    req_hash,
                    err
                );
                exit(1);
            }
        };
        file_reader
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<_>>()
    } else {
        vec![req_hash.to_string()]
    };

    if hashes_to_crack.is_empty() {
        eprintln!("{} No hashes found to crack.", "[x]".red());
        exit(5);
    }

    // --- Pre-load wordlist ---
    let wordlist_file = match file_pass(pass_file_loc) {
        Ok(reader) => reader,
        Err(err) => {
            eprintln!(
                "{} Failed to open wordlist file '{}': {}",
                "[X]".red(),
                pass_file_loc,
                err
            );
            exit(1);
        }
    };
    let wordlist: Vec<String> = wordlist_file.lines().filter_map(Result::ok).collect();

    let total_hashes_cracked = Arc::new(AtomicUsize::new(0));
    let read_error_count = Arc::new(AtomicUsize::new(0)); // Not used when pre-loading
    let skipped_hashes = Arc::new(AtomicUsize::new(0));

    println!(
        "{} Specified wordlist: {}\n",
        "[+]".blue(),
        pass_file_loc.blue()
    );

    for hash in hashes_to_crack {
        let hash_type = hash_checker(&hash);
        let mut sha256_bool = false;
        let mut md5_bool = false;
        let mut sha3256_bool = false;
        let mut sha3384_bool = false;
        let mut sha3224_bool = false;
        let mut sha3512_bool = false;

        if mode_arg == "-ad" || mode_arg == "--automatic-detection" {
            println!(
                "{} Hash type identified as {}",
                "[+]".blue(),
                hash_type.blue()
            );
            match hash_type.as_str() {
                "SHA256" => sha256_bool = true,
                "MD5" => md5_bool = true,
                "SHA3-256" => sha3256_bool = true,
                "SHA3-384" => sha3384_bool = true,
                "SHA3-224" => sha3224_bool = true,
                "SHA3-512" => sha3512_bool = true,
                _ => {}
            }
        }

        if hash_type == "Unknown" {
            println!(
                "{} Found {} hash type: {}, skipping\n",
                "[x]".red(),
                hash_type.red(),
                hash.red()
            );
            skipped_hashes.fetch_add(1, Ordering::Relaxed);
            continue;
        }

        if !hash.contains(".txt") && !hash.contains("/") {
            println!(
                "{}. Attempting to crack hash: {}",
                total_hashes_cracked.load(Ordering::Relaxed) + 1, //not safe sum
                hash.blue()
            );
        } else {
            println!(
                "{} Hash contains Invalid characters or unknown file location, check your input!",
                "[x]".red()
            );
            exit(0);
        }

        // --- Parallel Processing ---
        let cracked_password: Option<String> = wordlist.par_iter().find_map_any(|line| {
            let trimmed_password = line.trim().as_bytes();
            let hashed_password = match mode_arg {
                "-s2" | "--sha256" => format!("{:x}", Sha256::digest(trimmed_password)),
                "-m" | "--md5" => format!("{:x}", Md5::digest(trimmed_password)),
                "-s3" | "--sha3-384" => format!("{:x}", Sha3_384::digest(trimmed_password)),
                "-s3-256" | "--sha3-256" => format!("{:x}", Sha3_256::digest(trimmed_password)),
                "-s3-5" | "--sha3-512" => format!("{:x}", Sha3_512::digest(trimmed_password)),
                "-s3-224" | "--sha3-224" => format!("{:x}", Sha3_224::digest(trimmed_password)),
                _ => {
                    if sha256_bool {
                        format!("{:x}", Sha256::digest(trimmed_password))
                    } else if md5_bool {
                        format!("{:x}", Md5::digest(trimmed_password))
                    } else if sha3384_bool {
                        format!("{:x}", Sha3_384::digest(trimmed_password))
                    } else if sha3256_bool {
                        format!("{:x}", Sha3_256::digest(trimmed_password))
                    } else if sha3512_bool {
                        format!("{:x}", Sha3_512::digest(trimmed_password))
                    } else if sha3224_bool {
                        format!("{:x}", Sha3_224::digest(trimmed_password))
                    } else {
                        return None;
                    }
                }
            };

            if !hashed_password.is_empty() && hashed_password == hash {
                Some(line.clone()) // Return the cracked password
            } else {
                if verbose_mode {
                    //Not safe print
                    println!(
                        "{} Attempt N/A | Password: {} | Hash: {}",
                        "[x]".red(),
                        line, // Print the line
                        hashed_password
                    );
                }
                None // Return None if not cracked
            }
        });
        // --- End Parallel Processing ---

        if let Some(password) = cracked_password {
            println!(
                "{} Cracking successful! ----> {} <---- [Hash: {}] | Total Attempts: N/A\n",
                "[✔]".green(),
                password.green(),
                hash
            );
            total_hashes_cracked.fetch_add(1, Ordering::Relaxed);
        } else {
            println!(
                "{} Failed to crack hash: {} | Total Failed attempts: N/A\n",
                "[X]".red(),
                hash.red()
            );
        }
    }

    println!(
        "\n{} Completed! Total Hashes Cracked: {}, Ignored Words: {}, Total hashes skipped: {}",
        "[✔]".yellow(),
        total_hashes_cracked.load(Ordering::Relaxed),
        read_error_count.load(Ordering::Relaxed), // Not used, but kept for consistency
        skipped_hashes.load(Ordering::Relaxed)
    );
}
