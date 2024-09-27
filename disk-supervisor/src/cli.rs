#[derive(clap::Parser, Debug)]
#[command(
    about = "A disk supervisor utility",
    long_about = None,
)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Command {
    Generate {
        /// A directory to recursively walk and calculate the checksum for.
        path: Option<String>,
    },
    Check {
        /// A directory to check against the checksum.
        path: Option<String>,
    },
}
