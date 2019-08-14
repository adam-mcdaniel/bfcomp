extern crate bfcore;
use bfcore::Interpreter;

use crate::{COMPILER, CompilerInput, CompilerOutput, Lang};


#[derive(Default, Debug, Clone)]
pub struct Compiler {
    lang: Lang,
    compiler_input: CompilerInput,
    compiler_output: CompilerOutput
}


impl Compiler {
    pub fn new(lang: Lang, input_script: String) -> Self {
        Self {
            lang,
            compiler_input: CompilerInput::new(lang, input_script),
            compiler_output: CompilerOutput::new(lang)
        }
    }

    pub fn compile(mut self) -> String {
        Interpreter::new(
            COMPILER,
            &mut self.compiler_input,
            &mut self.compiler_output
        ).run();

        format!("{}", self.compiler_output)
    }
}