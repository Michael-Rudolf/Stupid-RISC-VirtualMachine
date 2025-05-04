# Stupid RISC Virtual Machine (sriscvm)

## What is this?
This is a Virtual Machine (pretty self explanatory) that supports my "Stupid RISC" instruction set which you can find under https://github.com/Michael-Rudolf/Stupid-RISC-instruction-set.

## Table of contents 
- [Installation](Readme.md#Installation)
- [Flags](Readme.md#Flags)

## Installation
### This programm can't actually be installed, but it can be aliased.
To perform this, please build it first.

### Building
Please ensure you have cargo and rustc installed properly.
If you have it installed, enter the following code in your terminal in this projects subfolder:
```sh
cargo build
```
*This will build the project for your computer.*

Now, you can locate the binary, which should usually be in ```target/debug/VirtualMachine```.
You can now put this into your program folder (Application folder in macOS, /etc in Linux, any in Windows).

### Shell function
You can make an alias in your bash/zsh/fish profile.
For example (**change for your own system!**)
```shell
sriscvm(){
    args=()
    for arg in "$@"; do
        if [[ -d "$arg" || -f "$arg" ]]; then
            abs_path="$(realpath "$arg")"
            args+=("$abs_path")
        else
            args+=("$arg")
        fi
    done
    (cd ~/path/to/virtual_machine && ./VirtualMachine "${args[@]}")
}
```
*This is most likely going to be /bin in Linux, /bin or ~/Applications on MacOS and more or less arbitrary on Windows.*

## Flags
### Last Argument
Though technically not a flag, it almost acts as one (and was the -f flag previously). This is the file to be executed.

### -hz
You can set the machine to target a specific speed using the -hz flag. Please note that this will never reach the target speed,
but just wait an additional time.

### -ms
With the memory store flag (*-ms*), you can store the machines DRAM to a binary file at the end of execution. 
The file needs to be created (```touch mem_sto_file.bin```) before that.

### Example
Here is an example usage with all the flags:
```shell
sriscvm -hz 20 -f main.bin -ms mem_sto_file.bin
```

## Commands
Commands are used during execution to quit it, read variables or modify them (upcoming). Using them will pause execution.

### Beahaviour
To enter a command **press any key** (preferably letters, there are still issues especially with newlines),
which will pause execution. A colon will appear to indicate this.
You can now type any command or use r or **return** to continue with execution.

### Quit
The **quit** command or **q** to leave the program.
Note that this will also delete the alternate screen buffer.

### Memory Get
The **memget** command (or **mg** or **mem-get" will get the memory at the given index (starting at 0).

Example:
```
mem-get 100
```
*This would retrieve the memory address 100 and output it in **decimal***

There are more commands upcoming.
