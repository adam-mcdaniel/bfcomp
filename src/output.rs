extern crate bfcore;
use bfcore::Output;

use crate::Lang;
use std::fmt::{Display, Formatter, Error};

#[derive(Default, Debug, Clone)]
pub struct CompilerOutput {
    lang: Lang,
    output_script: String
}

impl CompilerOutput {
    pub fn new(lang: Lang) -> Self {
        Self {
            lang, output_script: String::from("")
        }
    }

    pub fn extension(self, name: &str) -> String {
        name.to_owned() + self.lang.extension()
    }
}

impl Output for &mut CompilerOutput {
    fn output(&mut self, ch: char) {
        self.output_script.push(ch)
    }
}


impl Display for CompilerOutput {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.output_script)
    }
}
