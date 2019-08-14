use clap::arg_enum;

arg_enum!{
    /// lang_c    - C code
    /// lang_ruby - Ruby code
    /// lang_go   - Go code
    /// lang_tcl  - Tcl code
    /// lang_java - Java code
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Lang {
        C, Ruby, Go, Tcl, Java
    }
}

impl Lang {
    pub fn extension(&self) -> &str {
        match self {
            Lang::C => "c",
            Lang::Ruby => "rb",
            Lang::Go => "go",
            Lang::Tcl => "tcl",
            Lang::Java => "java",
        }
    }

    pub fn script_header(&self) -> &str {
        match self {
            Lang::C => "@lang_c\n",
            Lang::Ruby => "@lang_ruby\n",
            Lang::Go => "@lang_go\n",
            Lang::Tcl => "@lang_tcl\n",
            Lang::Java => "@lang_java\n",
        }
    }
}

impl Default for Lang {
    fn default() -> Self {
        Lang::C
    }
}