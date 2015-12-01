use super::{Request, Command};

pub type ParseError = String;

// Subcommands inherit options from supercommands
// options can appear in any order in the input
// no arguments for now
pub fn parse<I>(input: I, root: &Command) 
            -> Result<(Request, &Command), ParseError>
    where I : Iterator<Item=String>
{
    /*
     * loop through all the tokens in the input, gathering up the options
     * and their associated values. also get the subcommand
     * 
     */
    let mut opts = Vec::new();
    let mut cmd_opts = Vec::new();
    cmd_opts.extend(root.options());
    for token in input {
        // don't parse short args correctly, for now
        // you should be able to combine them, like -a -b -c can be written
        // -abc, with the caveat that all but the last option need to be boolean
        if token.starts_with("--") || token.starts_with("-") {
            if token.starts_with("--") {
                opts.push(token[2..].to_string());
            } else {
                opts.push(token[1..].to_string());
            }
        } else {
            // no arguments for now, so it must be a subcommand!
            let subcmd = match root.subcommand(&token) {
                None => return Err(format!("Subcommand {} not found", &token)),
                Some(cmd) => cmd,
            };

            cmd_opts.extend(subcmd.options());
        }
    }
    unimplemented!()
}
