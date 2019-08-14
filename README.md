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

bfcomp x.x.x
Adam McDaniel <adam.mcdanie17@gmail.com>
Compiles brainfuck by interpreting the AWIB brainfuck compiler with BFCORE

USAGE:
    bfcomp [OPTIONS] <INPUT_FILE> --target <TARGET>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --out <OUTPUT_FILE>    Path of output file
    -t, --target <TARGET>      The target language to output source code for (c, ruby, go, tcl, java) [possible values:
                               C, Ruby, Go, Tcl, Java]

ARGS:
    <INPUT_FILE>    Path to brainfuck file to compile
```