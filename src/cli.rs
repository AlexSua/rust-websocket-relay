use std::{net::IpAddr, path::PathBuf};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    /// Enable SSL by introducing the certificate and the private key.
    Ssl {
        /// SSL certificate
        #[clap(value_parser)]
        certificate_path: PathBuf,

        /// SSL private key
        #[clap(value_parser)]
        privkey_path: PathBuf,
    },
}
/// A lightweight and resource friendly relay server to easily communicate two applications using WebSockets.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Address where the server will be listening.
    #[clap(short, long, value_parser, default_value = "0.0.0.0")]
    pub ip: IpAddr,

    /// Port where the server will be listening
    #[clap(short,long,value_parser = clap::value_parser!(u16).range(1..), default_value_t=8080)]
    pub port: u16,

	/// Path where the server will be listening
	#[clap(short,long,value_parser)]
	pub scope: Option<String>,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

pub fn parse_arguments() -> Cli {
    Cli::parse()
}
