extern crate bfcore;
use bfcore::Input;

use crate::Lang;


#[derive(Default, Debug, Clone)]
pub struct CompilerInput {
    lang: Lang,
    index: usize,
    input_script: String
}


impl CompilerInput {
    pub fn new(lang: Lang, input_script: String) -> Self {
        Self {
            lang, index: 0,
            input_script:lang.script_header().to_string() + &input_script
        }
    }
}


impl Input for &mut CompilerInput {
    fn input(&mut self) -> char {
        match self.input_script.chars().nth(self.index) {
            Some(ch) => { self.index += 1; ch },
            None => '\0'
        }
    }
}
