use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about)]
pub struct GenmakeArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Set the executable name for THIS Makefile
    #[arg(short, long, value_name = "EXECUTABLE_NAME")]
    pub executable: Option<String>,

    /// Set the compiler name for THIS Makefile
    #[arg(short, long, value_name = "COMPILER_NAME")]
    pub compiler: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Set the executable name PERMANENTLY
    SetExecutable(NameArgument),

    /// Set the compiler name PERMANENTLY
    SetCompiler(NameArgument),
}

#[derive(Args)]
#[group(required = true)]
pub struct NameArgument {
    pub name: String,
}

impl GenmakeArgs {
    pub fn subcommands_provided(&self) -> bool {
        self.command.is_some()
    }

    pub fn flags_provided(&self) -> bool {
        self.executable.is_some() || self.compiler.is_some()
    }
}
