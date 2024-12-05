# SHA256_cracker
This Project takes input of hash for first argument and File location as second argument.
**Options Usage:**
Usage --> For Program source: cargo run <hash> <Path_to_the_wordlist> <Mode> 
Modes:
 -v,--verbose
 -n,--normal
### Usage for Source
- Get into the directory of the project(eg: `cd sha256_cracker`)
**Note: This require cargo installed on the system as a prerequesite. For cargo installation refer here:** https://doc.rust-lang.org/cargo/getting-started/installation.html
- Then run below command with correct options.
**`cargo run <sha256_hash> <Path_to_password_dictionary> <Mode>`**(eg: cargo run 79bb8d29bad9c9534b5b0d154febf0cec5efbdb9d15821bb6675af2636a061d2 /usr/share/wordlists/rockyou.txt -n)
- Successful output might look likes this
- ![image](https://github.com/user-attachments/assets/cf13f716-801e-49aa-bc51-ba1e8bb7997a)
### Usage Linux Binary
- Download the latest version  Binary from here:- https://github.com/apocalypsecracker/SHA256_cracker/releases/latest
- Give executable permissions to the binary:
```
chmod +x sha256_cracker
```
- After Downloading file then, run it:- `./sha256_cracker <hash> <Path_To_The_wordlist> <Mode>`

 
