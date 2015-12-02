use std::collections::HashMap;

pub mod cli;
pub mod request;

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
	run:        fn(request::Request) -> Result<(), String>,
	post_run:   (),
	help_text:   HelpText,
	subcommands: HashMap<String, Command>,
}

impl Command {
    pub fn new(options: Vec<Opt>,
               arguments: Vec<Argument>,
               help_text: HelpText,
               subcommands: Vec<(String, Command)>)
               -> Self
    {
        fn run(req: request::Request) -> Result<(), String> {
            unimplemented!()
        }
        Command {
            options: options,
            arguments: arguments,
            pre_run: (),
            run: run,
            post_run: (),
            help_text: help_text,
            subcommands: subcommands.into_iter().collect(),
        }
    }

    pub fn options(&self) -> Vec<&Opt> {
        let mut v = Vec::new();
        for opt in self.options.iter() {
            v.push(opt);
        }
        v
    }

    pub fn get_option<'a>(&'a self, name: &str) -> Option<&'a Opt> {
        for opt in self.options.iter() {
            for &opt_name in opt.names.iter() {
                if name == opt_name {
                    return Some(opt);
                }
            }
        }

        None
    }

    pub fn subcommand(&self, subcmd: &str) -> Option<&Command> {
        self.subcommands.get(subcmd)
    }
}

#[derive(Copy, Clone)]
pub enum OptType {
    Bool,
    String,
    Int,
}

// represents an option for a command
pub struct Opt {
    name: &'static str, // canonical name for the option
    pub names: Vec<&'static str>,
    pub opt_type: OptType,
    description: &'static str,
}

impl Opt {
    // The first name in the `names` vector is used as canonical name
    pub fn new_bool(names: Vec<&'static str>, desc: &'static str) -> Self {
        Self::new(names, OptType::Bool, desc)
    }

    fn new(mut names: Vec<&'static str>, opt_type: OptType, desc: &'static str) -> Self {
        let canonical = names[0];
        names.sort_by(|a, b| a.len().cmp(&b.len()));
        Opt {
            name: canonical,
            names: names,
            opt_type: opt_type,
            description: desc
        }
    }

    pub fn name(&self) -> &'static str { self.name }
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
