use std::collections::{hash_map, HashMap};
use std::slice;

pub mod cli;
pub mod request;

pub struct HelpText {
    pub tagline: &'static str, // used in <cmd usage>
    pub short_desc: &'static str, // used in DESCRIPTION
    pub synopsis: &'static str, // showcasing the cmd
    /*
     * pub usage:       Option<&'static str>, // overrides USAGE section
     * pub long_desc:   Option<&'static str>, // overrides DESCRIPTION section
     * pub options:     Option<&'static str>, // overrides OPTIONS section
     * pub arguments:   Option<&'static str>, // overrides ARGUMENTS section
     * pub subcommands: Option<&'static str> // overrides SUBCOMMANDS section
     * */
}

pub type RunFn = fn(&request::Request) -> Result<(), String>;

pub type ArgName = &'static str;
pub type OptName = &'static str;
pub type CommandName = &'static str;

pub trait Command {
    fn run(&self, &request::Request) -> Result<(), String>;
    fn get_def(&self) -> &CommandDefinition;

    fn get_name(&self) -> CommandName {
        self.get_def().get_name()
    }

    fn get_help_text(&self) -> &HelpText {
        self.get_def().get_help_text()
    }

    fn get_options(&self) -> CommandOptions {
        self.get_def().get_options()
    }

    // TODO: wrap this in arguments iterator type? boilerplate :(
    fn get_arguments(&self) -> slice::Iter<Argument> {
        self.get_def().get_arguments()
    }

    fn get_subcommand(&self, subcmd: &str) -> Option<&Command> {
        self.get_def().get_subcommand(subcmd)
    }

    fn num_args(&self) -> usize {
        self.get_def().num_args()
    }
}

// For easily making a command
pub struct CommandDefinition {
    name: CommandName,
    options: Vec<Opt>,
    arguments: Vec<Argument>,
    help_text: HelpText,
    subcommands: HashMap<CommandName, Box<Command>>,
}

impl CommandDefinition {
    // TODO: disallow an argument that isnt the last argument from being variadic
    pub fn new(name: CommandName,
               options: Vec<Opt>,
               arguments: Vec<Argument>,
               help_text: HelpText,
               subcommands: Vec<Box<Command>>)
               -> Self {
        CommandDefinition {
            name: name,
            options: options,
            arguments: arguments,
            help_text: help_text,
            subcommands: subcommands.into_iter()
                                    .map(|cmd| (cmd.get_name(), cmd))
                                    .collect(),
        }
    }

    pub fn get_name(&self) -> CommandName {
        self.name
    }

    pub fn get_help_text(&self) -> &HelpText {
        &self.help_text
    }

    pub fn get_options(&self) -> CommandOptions {
        CommandOptions::new(self.options.iter())
    }

    pub fn get_subcommand(&self, subcmd: &str) -> Option<&Command> {
        self.subcommands.get(subcmd).map(|cmd| &**cmd)
    }

    pub fn num_args(&self) -> usize {
        self.arguments.len()
    }

    // TODO: evaluate
    pub fn get_arguments(&self) -> slice::Iter<Argument> {
        self.arguments.iter()
    }
}

// iterator over (name, command) pairs. Each command can have multiple names.
struct CommandOptions<'a> {
    opt_iter: slice::Iter<'a, Opt>,
    curr_opt: Option<(&'a Opt, slice::Iter<'a, OptName>)>,
}

impl<'a> CommandOptions<'a> {
    fn new(iter: slice::Iter<'a, Opt>) -> Self {
        CommandOptions {
            opt_iter: iter,
            curr_opt: None,
        }
    }

    fn advance_opt_iter(&mut self) -> bool {
        match self.opt_iter.next() {
            None => false,
            Some(opt) => {
                self.curr_opt = Some((opt, opt.get_names()));
                true
            }
        }
    }

    fn get_curr_opt(&self) -> Option<&'a Opt> {
        self.curr_opt.as_ref().map(|curr| curr.0)
    }

    fn get_name_iter(&mut self) -> Option<&mut slice::Iter<'a, OptName>> {
        self.curr_opt.as_mut().map(|curr| &mut curr.1)
    }
}

impl<'a> Iterator for CommandOptions<'a> {
    type Item = (OptName, &'a Opt);

    fn next(&mut self) -> Option<(OptName, &'a Opt)> {
        if self.curr_opt.is_none() {
            if !self.advance_opt_iter() {
                return None;
            }
        }

        let name: OptName;
        loop {
            match self.get_name_iter().unwrap().next() {
                Some(n) => {
                    name = n;
                    break;
                }
                None => {
                    if !self.advance_opt_iter() {
                        return None;
                    }
                }
            }
        }

        Some((name, self.get_curr_opt().unwrap()))
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
    name: OptName, // canonical name for the option
    pub names: Vec<OptName>,
    pub opt_type: OptType,
    description: &'static str,
}

impl Opt {
    // The first name in the `names` vector is used as canonical name
    pub fn new_bool(names: Vec<OptName>, desc: &'static str) -> Self {
        Self::new(names, OptType::Bool, desc)
    }

    fn new(mut names: Vec<OptName>, opt_type: OptType, desc: &'static str) -> Self {
        let canonical = names[0];
        names.sort_by(|a, b| a.len().cmp(&b.len()));
        Opt {
            name: canonical,
            names: names,
            opt_type: opt_type,
            description: desc,
        }
    }

    pub fn get_name(&self) -> OptName {
        self.name
    }

    pub fn get_names(&self) -> slice::Iter<OptName> {
        self.names.iter()
    }
}

#[derive(Copy, Clone)]
enum ArgumentType {
    String,
    File,
}

pub struct Argument {
    name: ArgName,
    ty: ArgumentType,
    required: bool,
    variadic: bool,
    description: &'static str,
}

impl Argument {
    pub fn new_file(name: ArgName,
                    required: bool,
                    variadic: bool,
                    desc: &'static str)
                    -> Self {
        Self::new(name, ArgumentType::File, required, variadic, desc)
    }

    pub fn new_string(name: ArgName,
                      required: bool,
                      variadic: bool,
                      desc: &'static str)
                      -> Self {
        Self::new(name, ArgumentType::String, required, variadic, desc)
    }

    fn new(name: ArgName,
           ty: ArgumentType,
           required: bool,
           variadic: bool,
           desc: &'static str)
           -> Self {
        Argument {
            name: name,
            ty: ty,
            required: required,
            variadic: variadic,
            description: desc,
        }
    }

    pub fn is_variadic(&self) -> bool {
        self.variadic
    }

    pub fn name(&self) -> ArgName {
        self.name
    }

    pub fn arg_type(&self) -> ArgumentType {
        self.ty
    }
}
