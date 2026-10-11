use std::collections::HashMap;

pub struct Tree {
    pub name: String,
    pub desc: Option<String>,
    pub version: Option<String>,
    pub commands: Option<Vec<Command>>,
    pub handler: Option<fn()>,
}

pub struct Command {
    pub name: String,
    pub desc: Option<String>,
    pub flags: Option<Vec<Flag>>,
    pub options: Option<Vec<OptionSpec>>,
    pub handler: Option<fn()>,
    pub args: Option<Args>,
    pub subcommands: Option<Vec<Command>>,
}

pub struct Args {
    pub number: usize,
    pub data_type: Vec<OptionType>,
}

pub struct Flag {
    pub name: String,
    pub short: Option<String>,
    pub desc: Option<String>,
}

pub struct OptionSpec {
    pub name: String,
    pub short: Option<String>,
    pub desc: Option<String>,
    pub data_type: OptionType,
}

pub enum OptionType {
    Str,
    Int,
    Float,
}

pub struct Context {
    pub commands: Vec<String>,
    pub cmds_fn: HashMap<String, fn()>,
}

impl Tree {
    pub fn new(name: String, desc: Option<String>) -> Self {
        Self {
            name,
            desc,
            version: Some(String::from("0.0.1")),
            commands: None,
            handler: None,
        }
    }

    pub fn description(&mut self, desc: String) {
        self.desc = Some(desc);
    }

    pub fn version(&mut self, version: String) {
        self.version = Some(version);
    }

    pub fn add_command(&mut self, command: Command) {
        self.commands.get_or_insert_with(Vec::new).push(command);
    }

    pub fn handle(&mut self, handle: fn()) {
        self.handler = Some(handle);
    }
}

impl Command {
    pub fn new(name: String, handler: Option<fn()>) -> Self {
        Self {
            name,
            desc: None,
            flags: None,
            options: None,
            handler,
            args: None,
            subcommands: None,
        }
    }

    pub fn description(&mut self, desc: String) {
        self.desc = Some(desc);
    }

    pub fn add_flag(&mut self, flag: Flag) {
        self.flags.get_or_insert_with(Vec::new).push(flag);
    }

    pub fn add_option(&mut self, option: OptionSpec) {
        self.options.get_or_insert_with(Vec::new).push(option);
    }

    pub fn handle(&mut self, handle: fn()) {
        self.handler = Some(handle);
    }

    pub fn add_subcommand(&mut self, subcmd: Command) {
        self.subcommands.get_or_insert_with(Vec::new).push(subcmd);
    }

    pub fn add_arg(&mut self, arg: Args) {
        self.args = Some(arg);
    }
}

impl Args {
    pub fn new(number: usize, data_type: Vec<OptionType>) -> Self {
        Self { number, data_type }
    }
}

impl Flag {
    pub fn new(name: String) -> Self {
        Self {
            name,
            short: None,
            desc: None,
        }
    }

    pub fn short(&mut self, short: String) {
        self.short = Some(short);
    }

    pub fn description(&mut self, desc: String) {
        self.desc = Some(desc);
    }
}

impl OptionSpec {
    pub fn new(name: String, data_type: OptionType) -> Self {
        Self {
            name,
            short: None,
            desc: None,
            data_type,
        }
    }

    pub fn short(&mut self, short: String) {
        self.short = Some(short);
    }

    pub fn description(&mut self, desc: String) {
        self.desc = Some(desc);
    }
}

impl Context {
    pub fn new(commands: Vec<String>, cmds_fn: HashMap<String, fn()>) -> Self {
        Self { commands, cmds_fn }
    }
}
