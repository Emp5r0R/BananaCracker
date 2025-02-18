# BananaCracker
This is a CLI tool build to crack hashes at lighting speed with minimal effort.

> [!IMPORTANT]
> Options have been changed from the latest versions

### Modes  
- `-v`, `--verbose`

### Modules
- `-s2`, `--sha256`
- `-m`, `--md5`
- `-s3-256`, `--sha3-256`
- `-s3-244`, `--sha3-244`
- `-s3-5`, `--sha3-512`
- `-s3`, `--sha3-384`
- `-ad`, `--automatic-detection`
### Example Usage for Modules:
1. `./BananaCracker e2c587ac5155e215da971f07fb1aba71788ef2fa423a7efb251267419e146521 /usr/share/wordlists/rockyou.txt -s2`
2. `./BananaCracker ebf13de02b7406fd14c97ec3d136e6b36a27b53dd327602e7ace6912ccbccfa9 /usr/share/wordlists/rockyou.txt --sha256`
3. `./BananaCracker path/to/the/hashes.txt /usr/share/wordlists/rockyou.txt -m`
4. `./BananaCracker path/to/the/hashes.txt /usr/share/wordlists/rockyou.txt --md5`

### Example Usage for Modes:
1. `./BananaCracker e2c587ac5155e215da971f07fb1aba71788ef2fa423a7efb251267419e146521 /usr/share/wordlists/rockyou.txt -s2 -v`
2. `./BananaCracker ebf13de02b7406fd14c97ec3d136e6b36a27b53dd327602e7ace6912ccbccfa9 /usr/share/wordlists/rockyou.txt -s2 --verbose`
3. `./BananaCracker path/to/the/hashes.txt /usr/share/wordlists/rockyou.txt -ad`
4. `./BananaCracker path/to/the/hashes.txt /usr/share/wordlists/rockyou.txt --automatic-detection`

> [!WARNING]
> Verbose mode can greatly slow down the process of cracking and this tool uses the CPU to the fullest.

---
## Installation
---
### Linux Binary
- Download the latest version of Linux Executable Binary from here:- https://github.com/Emp5r0R/BananaCracker/releases/latest
- Give executable permissions to the binary:
```
chmod +x BananaCracker
```
- then, run :- `./BananaCracker <hash> <Path_To_The_wordlist> <Mode>`

- **Placing into the executable path(Optional)**

* I know always changing into the directory where BananaCracker lives and executing it, is a pain in the ass. Don't worry I have a solution for you, Do the following steps carefully:
- 'cd' into the directory where BananaCracker is stored generally in Downloads.
- Now type this in the terminal: 
```
sudo mv BananaCracker /usr/local/bin

```
* That's all you've made it, Now you can execute BananaCracker anywhere from the Terminal Emulator like this:
```
BananaCracker
```
___________________________________________________________________________________________________________________________________________________________________________
### Windows executable
- Download the latest version of Windows-x86 executable from here:- https://github.com/Emp5r0R/BananaCracker/releases/latest
- Open Command Prompt or Powershell:
- Now go to the path where BananaCracker.exe located at or Open cmd from the path
- then, run :- `./BananaCracker.exe <hash> <Path_To_The_wordlist> <Module> <Mode>` or simply `BananaCracker.exe <hash> <Path_To_The_wordlist> <Mode>`
**Note:** As of now windows executable are only made for x86 platforms.
___________________________________________________________________________________________________________________________________________________________________________
### From Source
- Get into the directory of the project(eg: `cd BananaCracker`)
**Note: This require cargo to be installed on the system as a prerequesite. For cargo installation refer here:** https://doc.rust-lang.org/cargo/getting-started/installation.html
- Then run below command with correct options.
**`cargo run <sha256_hash> <Path_to_password_dictionary> <Modules>`**(eg: cargo run 79bb8d29bad9c9534b5b0d154febf0cec5efbdb9d15821bb6675af2636a061d2 /usr/share/wordlists/rockyou.txt -s2)
- Successful output might look likes this:

```
 
▗▄▄▖ ▗▞▀▜▌▄▄▄▄  ▗▞▀▜▌▄▄▄▄  ▗▞▀▜▌┏┓      ┓              
▐▌ ▐▌▝▚▄▟▌█   █ ▝▚▄▟▌█   █ ▝▚▄▟▌┃ ┏┓┏┓┏ ┃┏ ┏┓┏┓           
▐▛▀▚▖     █   █      █   █      ┗┛┛ ┗┻┗ ┛┗ ┗ ┛          
▐▙▄▞▘  

 4.4.3 https://github.com/Emp5r0R/BananaCracker

[+] Performing in Automatic Detection mode
[-] Verbose Mode: False
[+] Specified wordlist: /usr/share/wordlists/rockyou.txt

[+] Hash type identified as SHA256
1. Attempting to crack hash: 15e2b0d3c33891ebb0f1ef609ec419420c20e320ce94c65fbc8c3312448eb225
[✔] Cracking successful! ----> 123456789 <---- [Hash: 15e2b0d3c33891ebb0f1ef609ec419420c20e320ce94c65fbc8c3312448eb225] | Total Attempts: N/A

[+] Hash type identified as SHA256
2. Attempting to crack hash: 8bb0cf6eb9b17d0f7d22b456f121257dc1254e1f01665370476383ea776df414
[✔] Cracking successful! ----> 1234567 <---- [Hash: 8bb0cf6eb9b17d0f7d22b456f121257dc1254e1f01665370476383ea776df414] | Total Attempts: N/A

[+] Hash type identified as MD5
3. Attempting to crack hash: 25f9e794323b453885f5181f1b624d0b
[✔] Cracking successful! ----> 123456789 <---- [Hash: 25f9e794323b453885f5181f1b624d0b] | Total Attempts: N/A

[+] Hash type identified as Unknown
[x] Found Unknown hash type: 8afa847f50a716e64932d995c8e7435af, skipping

[+] Hash type identified as MD5
4. Attempting to crack hash: 8afa847f50a716e64932d995c8e7435a
[✔] Cracking successful! ----> princess <---- [Hash: 8afa847f50a716e64932d995c8e7435a] | Total Attempts: N/A

[+] Hash type identified as SHA3-224
5. Attempting to crack hash: 9e02be56d8e88fbb38d597907d0278873075169a95d63351396968dc
[✔] Cracking successful! ----> rockyou <---- [Hash: 9e02be56d8e88fbb38d597907d0278873075169a95d63351396968dc] | Total Attempts: N/A

[+] Hash type identified as SHA3-384
6. Attempting to crack hash: b1b31e51e8336eea25d27ca9e979a0a570fd7a0124cc99ff7385e5325c450c536cb31ebdb422a1e27d570017f676f716
[✔] Cracking successful! ----> rockyou <---- [Hash: b1b31e51e8336eea25d27ca9e979a0a570fd7a0124cc99ff7385e5325c450c536cb31ebdb422a1e27d570017f676f716] | Total Attempts: N/A

[+] Hash type identified as SHA3-512
7. Attempting to crack hash: 9f03084ba788aa2fe368354b69be8619f2dff65f74394fa0511cce7672eb3a8365c79abc5adc4c8959837d3c987adb7acd7bfb4dcef04ff8155bc0b7c777858d
[✔] Cracking successful! ----> rockyou <---- [Hash: 9f03084ba788aa2fe368354b69be8619f2dff65f74394fa0511cce7672eb3a8365c79abc5adc4c8959837d3c987adb7acd7bfb4dcef04ff8155bc0b7c777858d] | Total Attempts: N/A

[+] Hash type identified as SHA256
8. Attempting to crack hash: 25d9ea918146a426bca4d660103464545e506f91598a6907937eff35494a443c
[X] Failed to crack hash: 25d9ea918146a426bca4d660103464545e506f91598a6907937eff35494a443c | Total Failed attempts: N/A


[✔] Completed! Total Hashes Cracked: 7, Ignored Words: 0, Total hashes skipped: 1


```
___________________________________________________________________________________________________________________________________________________________________________

### For Nerds
- If you love to read documentations or bunch texts you may read this [Wiki](https://github.com/Emp5r0R/BananaCracker/wiki) as much time spent on making these.
