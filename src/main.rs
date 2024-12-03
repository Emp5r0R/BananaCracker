use std::fs::File; // For file managemnet
use std::env;   //For enviroment & argument
use std::io::{BufRead, BufReader};  //For File reading
use sha2::{Sha256, Digest}; //Reading sha256
use std::process::exit;  //Process

fn main() {
//Arguments management
    let argue: Vec<String> = env::args().collect();

    if argue.len() != 3 {
        println!("Invalid Options Usage!");
        println!("Usage example: cargo run <hash_Sha256> <Path_toThe_wordlist>");
        exit(1);
    }    
    let req_hash = &argue[1];
    let pass_file_loc = &argue[2];
    let mut current_state = 1;
    println!("Process started: Attemting to crack the hash: {}", req_hash);
    println!("Specified wordlist: {}", pass_file_loc);

    if (pass_file_loc == "-h") || (pass_file_loc == "--help") {
        println!("Usage example: cargo run <hash_Sha256> <Path_toThe_wordlist>");
        exit(1);
    }

//File management
    
    let file_reader = file_pass(pass_file_loc);  //Reader for reading the contents of the file
//Main loop
    for contents in file_reader.lines() {
        match contents{                      //Handling the errors
            Ok(contents) => {
                let cont_pass = contents.trim().to_owned().into_bytes();
                let cont_pass_hash = format!("{:x}", Sha256::digest(&cont_pass));
                if &cont_pass_hash == req_hash {
                    println!("Password found[!]----> {} <----[!]Hash Crackerd. Total attempts: {}. Found password's hash: {}", std::str::from_utf8(&cont_pass).unwrap(), current_state, cont_pass_hash);
                    exit(0);
                }
                current_state += 1;
                println!("Current Attempt: [{}], Current password: {} ----> {}", current_state, std::str::from_utf8(&cont_pass).unwrap(), cont_pass_hash);
            }
            Err(e) => {println!("Unable to read the line! {}", e )}

        };
         
    }
    println!("[X]Password hash not found!. Advised to try different wordlist");
}

fn file_pass(path: &String) -> BufReader<File> {
    let pass_file = File::open(path).expect("Failed to open the file");  // Custom error message
    BufReader::new(pass_file)
}

    