use std::collections::HashMap;

mod cli;

pub struct HelpText {
    pub tagline:     &'static str, // used in <cmd usage>
    pub short_desc:  &'static str, // used in DESCRIPTION
    pub synopsis:    &'static str, // showcasing the cmd
    /*
	pub usage:       Option<&'static str>, // overrides USAGE section
	pub long_desc:   Option<&'static str>, // overrides DESCRIPTION section
	pub options:     Option<&'static str>, // overrides OPTIONS section
	pub arguments:   Option<&'static str>, // overrides ARGUMENTS section
	pub subcommands: Option<&'static str> // overrides SUBCOMMANDS section
    */
}

pub struct Command {
	options:    Vec<Opt>,
	arguments:  Vec<Argument>,
	pre_run:    (),
	run:        (),
	post_run:   (),
	help_text:   HelpText,
	subcommands: HashMap<String, Command>,
}

impl Command {
    pub fn new(options: Vec<Opt>,
               arguments: Vec<Argument>,
               help_text: HelpText)
               -> Self {
        Command {
            options: options,
            arguments: arguments,
            pre_run: (),
            run: (),
            post_run: (),
            help_text: help_text,
            subcommands: HashMap::new()
        }
    }

    pub fn options(&self) -> Vec<&Opt> {
        let mut v = Vec::new();
        for opt in self.options.iter() {
            v.push(opt);
        }
        v
    }

    pub fn subcommand(&self, subcmd: &str) -> Option<&Command> {
        self.subcommands.get(subcmd)
    }
}

pub enum OptType {
    Bool,
    String,
    Int,
}

// represents an option for a command
pub struct Opt {
    names: Vec<&'static str>,
    opt_type: OptType,
    description: &'static str,
}

impl Opt {
    pub fn new_bool(names: Vec<&'static str>, desc: &'static str) -> Self {
        Self::new(names, OptType::Bool, desc)
    }

    fn new(names: Vec<&'static str>, opt_type: OptType, desc: &'static str) -> Self {
        Opt { names: names, opt_type: opt_type, description: desc }
    }
}

enum ArgumentType {
    ArgString,
    ArgFile,
}

pub struct Argument {
    name: String,
    ty: ArgumentType,
    required: bool,
    description: String,
}

pub struct Request {
    x: (),
}
