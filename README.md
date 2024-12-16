# SHA256_cracker
This is a CLI tool build to crack hashes at lighting speed with minimal effort.

From the version 2.2.1 and above, Modes option are introduced for more versatility. This gave access to Normal(-n) and Verbose(-v) modes, Eventually modes are improved to be more versatile from version 3.3.3 where Normal mode completely replaced with Multifast(-m) which gave it ability to read hashes from a text file and crack those automatically one by one using specified wordlist. If you are using a older versions please be wary.

### Modes
- `-v`, `--verbose`
- `-m`, `--multifast`

### Example Usage for Multi Fast Mode:
1. `./sha256_cracker e2c587ac5155e215da971f07fb1aba71788ef2fa423a7efb251267419e146521 /usr/share/wordlists/rockyou.txt -m`
2. `./sha256_cracker ebf13de02b7406fd14c97ec3d136e6b36a27b53dd327602e7ace6912ccbccfa9 /usr/share/wordlists/rockyou.txt --multifast`
3. `./sha256_cracker path/to/the/hashes.txt /usr/share/wordlists/rockyou.txt -m`
4. `./sha256_cracker path/to/the/hashes.txt /usr/share/wordlists/rockyou.txt`

### Example Usage for Verbose Mode:
1. `./sha256_cracker e2c587ac5155e215da971f07fb1aba71788ef2fa423a7efb251267419e146521 /usr/share/wordlists/rockyou.txt -v`
2. `./sha256_cracker ebf13de02b7406fd14c97ec3d136e6b36a27b53dd327602e7ace6912ccbccfa9 /usr/share/wordlists/rockyou.txt --verbose`
3. `./sha256_cracker path/to/the/hashes.txt /usr/share/wordlists/rockyou.txt -v`

**Note:** Multi Fast mode is significantly faster than Verbose mode. Thus, choosing Multi Fast mode is recommended. If no modes are specified, Multi Fast mode will be selected automatically.

---
## Installation
---
### Linux Binary
- Download the latest version of Linux Executable Binary from here:- https://github.com/apocalypsecracker/SHA256_cracker/releases/latest
- Give executable permissions to the binary:
```
chmod +x sha256_cracker
```
- then, run :- `./sha256_cracker <hash> <Path_To_The_wordlist> <Mode>`

- **Placing into the executable path(Optional)**

* I know always changing into the directory where sha256_cracker lives and executing it, is a pain in the ass. Don't worry I have a solution for you, Do the following steps carefully:
- 'cd' into the directory where sha256_cracker is stored generally in Downloads.
- Now type this in the terminal: 
```
sudo mv sha256_cracker /usr/local/bin

```
* That's all you've made it, Now you can execute sha256_cracker anywhere from the Terminal Emulator like this:
```
sha256_cracker
```
___________________________________________________________________________________________________________________________________________________________________________
### Windows executable
- Download the latest version of Windows-x86 executable from here:- https://github.com/apocalypsecracker/SHA256_cracker/releases/latest
- Open Command Prompt or Powershell:
- Now go to the path where sha256_cracker.exe located at or Open cmd from the path
- then, run :- `./sha256_cracker.exe <hash> <Path_To_The_wordlist> <Mode>` or simply `sha256_cracker.exe <hash> <Path_To_The_wordlist> <Mode>`
**Note:** As of now windows executable are only made for x86 platforms.
___________________________________________________________________________________________________________________________________________________________________________
### From Source
- Get into the directory of the project(eg: `cd sha256_cracker`)
**Note: This require cargo to be installed on the system as a prerequesite. For cargo installation refer here:** https://doc.rust-lang.org/cargo/getting-started/installation.html
- Then run below command with correct options.
**`cargo run <sha256_hash> <Path_to_password_dictionary> <Mode>`**(eg: cargo run 79bb8d29bad9c9534b5b0d154febf0cec5efbdb9d15821bb6675af2636a061d2 /usr/share/wordlists/rockyou.txt -n)
- Successful output might look likes this
- ![image](https://github.com/user-attachments/assets/cf13f716-801e-49aa-bc51-ba1e8bb7997a)
___________________________________________________________________________________________________________________________________________________________________________
### For Nerds
- If you love to read documentation or bunch texts you may read this [Wiki](https://github.com/apocalypsecracker/SHA256_cracker/wiki) as much time spent on making these.
