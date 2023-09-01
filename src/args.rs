use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(
    author="Antwan van Houdt",
    version="0.1.0",
    about="Simple utility to dump CosmosDB NoSQL data",
    long_about = None
)]
pub struct Cli {
    #[arg(short, long)]
    pub account: Option<String>,
    #[arg(short, long)]
    pub key: Option<String>,
    #[arg(short, long)]
    pub connection_string: Option<String>,
    pub out: PathBuf,
}
