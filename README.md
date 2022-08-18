# Go-To command line utility
**Goto** its an utility made for changing your current directory in your shell in less key strokes.

## How to use

### 1. Declaring the directories you usually 'cd' into.
Goto reads the file `~/.gotopaths` to fetch what directories to display with what menu option related with.<br>
This file has its own format, and in the case of not following the specified format, that **directory listed with the invalid format won't be displayed**. An example of a valid gotopaths file its shown below:
```
/a/directory/you/want/to/access/quickly;itsoption
/home/user/Desktop;desk
```
***NOTE***: Keep in mind that the path you write in the file **can be displayed without existing**, and this will make your shell error when trying to '*cd*' into it. (*In the future I'll make it so paths that doesn't exist get displayed with a different color.*)

The paths and its options from above will be displayed like this in goto's menu:
```
[itsoption] /a/directory/you/want/to/access/quickly
[desk] /home/user/Desktop
```

### 2. Install the goto utility
To install the goto utility you'll have to first clone the repository, enter in it, and then install it with `cargo`.
```bash
git clone git@github.com:Folgue02/goto
cd goto

# This will create the `goto` executable in the folder `~/.cargo/bin/`, remember to put
# this path in your environment $PATH variable
cargo install --path .
```

### 3. Implement the `goto` function in your shell's autorun file.
For now, there are three scripts for three different shells (*powershell, fish and bash*). To make the goto function available from the the prompt copy the function and paste it in your shell's autorun file (*`~/.config/powershell/Microsoft.Powershell_profile.ps1`, `~/.config/fish/config.fish`, `~/.bashrc`*). <br>
*Each script has its implementation instructions written in comments*.