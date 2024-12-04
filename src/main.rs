use std::fs::File;                  // For file managemnet
use std::env;                       //For enviroment & argument
use std::io::{BufRead, BufReader};  //For File reading
use sha2::{Sha256, Digest};         //Reading sha256
use std::process::exit;             //Process
use colored::Colorize;              //Colours


fn main() {
//Arguments management
    let argue: Vec<String> = env::args().collect();

    

    if argue.len() == 4 {
        let req_hash = &argue[1];
        let pass_file_loc = &argue[2];
        let cust_option = &argue[3];
        let mut current_state = 1;
        println!("Process started: Attemting to crack the hash: {}", req_hash);
        println!("Specified wordlist path: {}", pass_file_loc);
//File management
        
        let file_reader = file_pass(pass_file_loc);  //Reader for reading the contents of the file
//Main loop
    for contents in file_reader.lines() {
        match contents{                      //Handling the errors
            Ok(contents) => {
                let cont_pass = contents.trim().to_owned().into_bytes();
                let cont_pass_hash = format!("{:x}", Sha256::digest(&cont_pass));
                if &cont_pass_hash == req_hash {
                    println!("{} Password found[!]----> {} <----[!]Hash Crackerd. Total attempts: {}. Found password's hash: {}","[✔]".green(), std::str::from_utf8(&cont_pass).unwrap().green(), current_state, cont_pass_hash);
                    exit(0);
                }
                current_state += 1;
                if (cust_option == "-v") || (cust_option == "--verbose"){
                    println!("{} Current Attempt: [{}], Current password: {} ----> {}","[✖]".red(), current_state, std::str::from_utf8(&cont_pass).unwrap().red(), cont_pass_hash);
                }
                
                
            }
            Err(e) => {println!("Unable to read the line! {}", e )}

        };
         
    }
    println!("{}","[X] Password hash not found!. Advised to try different wordlist!".red());

    } else {
        println!("{}", "Missing Options!".red());
        println!("{} {} {} {} {}","Usage --> For Raw Program:", "cargo run <hash> <Path_to_the_wordlist> <Mode>".blue(), "| --> For Binary:", "./sha256_cracker <hash> <Path_to_the_wordlist> <Mode>".blue(), "\nModes:\n -v,--verbose\n -n,--normal".blue());
        exit(1);
    }
}
// Custom error message
fn file_pass(path: &String) -> BufReader<File> {
    let pass_file = File::open(path).expect("Failed to open the file");  // Custom error message
    BufReader::new(pass_file)
}
    
