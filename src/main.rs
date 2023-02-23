mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = anyhow::Result<T, Error>;

use crate::commands::PngsmCommand;
use clap::Parser;

fn main() -> Result<()> {
    let program = PngsmCommand::parse();
    program.handle()
}
