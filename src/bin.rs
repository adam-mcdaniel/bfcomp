extern crate bfcomp;
use bfcomp::{Compiler, Lang};

extern crate clap;
use clap::{clap_app, crate_version, Arg, value_t, AppSettings};

use std::fs::{read_to_string, write};
use std::process::exit;


fn error(s: impl std::fmt::Display) {
    println!("==[ERROR]===> {}", s);
    exit(1);
}


fn main() {
    let matches = clap_app!(bfcomp =>
        (version: crate_version!())
        (author: "Adam McDaniel <adam.mcdanie17@gmail.com>")
        (about: "Compiles brainfuck by interpreting the AWIB brainfuck compiler with BFCORE")
        (@arg INPUT_FILE: +required "Path to brainfuck file to compile")
        (@arg OUTPUT_FILE: -o --out +takes_value "Path of output file")
    )
    .arg(
        Arg::with_name("TARGET")
            .short("t")
            .long("target")
            .required(true)
            .takes_value(true)
            .possible_values(&Lang::variants())
            .case_insensitive(true)
            .allow_hyphen_values(true)
            .help("The target language to output source code for (c, ruby, go, tcl, java)")
    )
    .setting(AppSettings::ArgRequiredElseHelp)
    .get_matches();


    // The Lang variant for the compiler
    let lang;
    // The string of brainfuck code
    let mut input_script = String::default();

    if let Some(file) = matches.value_of("INPUT_FILE") {
        if let Ok(contents) = read_to_string(file) {
            input_script = contents;
        } else {
            error(format!("Could not read file '{}'", file));
        }
    }

    if let Ok(target) = value_t!(matches, "TARGET", Lang) {
        lang = target;
    } else {
        lang = Lang::default();
    }


    let compiled_output = Compiler::new(lang, input_script).compile();
    let output_file = match matches.value_of("OUTPUT_FILE") {
        Some(f) => f.to_string(),
        None => "output.".to_string() + &lang.extension()
    };


    
    if let Err(_) = write(&output_file, compiled_output) {
        error(format!("Could not write to file '{}'", output_file));
    }
}
