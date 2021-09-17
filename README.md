# Difr
A command-line tool to discern the differences between two text files.

## Installation
Clone this repository
```sh
git clone "https://github.com/0xba1/difr.git"
cd difr
```
Ensure that you have [rust](https://www.rust-lang.org/) installed, if not follow [these instructions](https://www.rust-lang.org/tools/install) to install rust.

Build the binary by running
```sh
cargo build --release
```

Copy the binary file at `./target/release/difr` or `./target/release/difr.exe`(for Windows) to the bin folder of your choice and ensure it's included in $PATH

## Example
Input:
```sh
difr sample/baz.txt sample/buz.txt
```
Output:
```sh
File 1:	sample/baz.txt	29bytes	1line(s)
File 2:	sample/buz.txt	19bytes	1line(s)

Computing SHA3-256 hashes of files...
File 1:	7623b9d516a1d92ce0795076f2a5081b8070fab79eb5cffb845e9d525be7111d
File 2:	c52fa205f47daf2b17c84ca57b50cffb2e9e08ccffcadffb77cde9477a1c1634

Contents of files are different


Line 1:
>	Look at what you made me do.
>	What is your hash?

```

 **It uses the `coloured` crate to provide better looking output.**
