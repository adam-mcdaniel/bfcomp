# bfcomp
A compiler for BrainF*ck implemented with AWIB and bfcore


## Installation

Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install `bfcomp`

```bash
cargo install -f bfcomp
```

All done!

## Usage 

```
$ bfcomp --help

bfcomp 0.1.0
Adam McDaniel <adam.mcdanie17@gmail.com>
Compiles brainfuck by interpreting the AWIB brainfuck compiler with BFCORE

USAGE:
    bfcomp [FLAGS] [OPTIONS] <INPUT_FILE>

FLAGS:
    -h, --help       Prints help information
    -t, --target     The target language to output source code for (c, ruby, go, tcl, java)
    -V, --version    Prints version information

OPTIONS:
    -o, --out <OUTPUT_FILE>    Path of output file

ARGS:
    <INPUT_FILE>    Path to brainfuck file to compile
```