# SHA256_cracker
This Project takes input of hash for first argument and File location as second argument.

**Options Usage:**
Usage --> cargo run <hash> <Path_to_the_wordlist> <mode> 
Modes:
 -v,--verbose
 -n,--normal

### Usage for Linux Binary
- Download the latest version Binary from here:- https://github.com/apocalypsecracker/SHA256_cracker/releases/latest
- Give executable permissions to the binary:
```
chmod +x sha256_cracker
```
- then, run :- `./sha256_cracker <hash> <Path_To_The_wordlist> <Mode>`

### Usage for Source
- Get into the directory of the project(eg: `cd sha256_cracker`)
**Note: This require cargo to be installed on the system as a prerequesite. For cargo installation refer here:** https://doc.rust-lang.org/cargo/getting-started/installation.html
- Then run below command with correct options.
**`cargo run <sha256_hash> <Path_to_password_dictionary> <Mode>`**(eg: cargo run 79bb8d29bad9c9534b5b0d154febf0cec5efbdb9d15821bb6675af2636a061d2 /usr/share/wordlists/rockyou.txt -n)
- Successful output might look likes this
- ![image](https://github.com/user-attachments/assets/cf13f716-801e-49aa-bc51-ba1e8bb7997a)

### Creator Answered Questions

**Which should I choose, Binary or Source?**
- In my opinion Choosing binary would be better because it is much easier to use and less likely to get in to issue. Building from source is different and much more complicated, There are also prerequesite for building from source like cargo need to be installed. However if can manage technical things in building and your confident that you can troubleshoot any issues and also if you want new updated version with new features and fixes (Note:might also have bugs too) then this should be your choice  

**What is the versions of executable binaries, which tagged as Pre-release?**
- Pre-release versions are generally not optimized binaries which can work fine but are slow and takes more disk space than Optimized binaries.
 
