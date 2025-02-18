# BananaCracker
This is a CLI tool build to crack hashes at lighting speed with minimal effort.

> [!IMPORTANT]
> From last commit modes option have been chanegd.

###  
- `-ad`, `--automatic-detection`
- `-v`, `--verbose`
- `-s2`, `--sha256`
- `-m`, `--md5`

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
> Verbose mode can greatly slow down the process of cracking.

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
- Successful output might look likes this
- ![image](https://github.com/user-attachments/assets/cf13f716-801e-49aa-bc51-ba1e8bb7997a)
___________________________________________________________________________________________________________________________________________________________________________
### For Nerds
- If you love to read documentations or bunch texts you may read this [Wiki](https://github.com/Emp5r0R/BananaCracker/wiki) as much time spent on making these.
