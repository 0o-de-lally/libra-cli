use anyhow::Result;
use clap::Parser;

mod init;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"), author, version, about, long_about = None, arg_required_else_help = true)]
pub struct LibraConfigCli {
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Generate config.yaml file to store 0L configuration
    Init {
        /// Signing Ed25519 private key
        #[clap(long)]
        private_key: Option<String>,

        /// Profile to use from the CLI config
        ///
        /// This will be used to override associated settings such as
        /// the REST URL, the Faucet URL, and the private key arguments.
        ///
        /// Defaults to "default"
        #[clap(long)]
        profile: Option<String>,
    },
}

impl LibraConfigCli {
    pub fn run(&self) -> Result<()> {
        match &self.subcommand {
            Some(Subcommand::Init {
                private_key,
                profile,
            }) => init::run(private_key.to_owned(), profile.to_owned()),
            _ => Ok(()),
        }
    }
}
