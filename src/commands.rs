use clap::{Args, Parser, Subcommand};

use std::path::PathBuf;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct PngsmCommand {
    #[command(subcommand)]
    pub subcommand: SubC,
}

#[derive(Debug, Subcommand)]
pub enum SubC {
    /// Add secret massage to png file
    Encode(EncodeCommand),
    /// Get the secret massage from png file
    Decode(DecodeCommand),
    /// Remove massage from png file
    Remove(RemoveCommand),
    /// Print all massage in png file
    Print(PrintCommand),
}

#[derive(Debug, Args)]
pub struct EncodeCommand {
    /// png file that you want to encode
    #[arg(value_name = "FILE")]
    path: PathBuf,
    /// chunk type for your secret massage
    chunk_type: String,
    /// the massage you want to add to the chunk type
    message: String,
    /// show the output message
    #[arg(short, long)]
    output: bool,
}

#[derive(Debug, Args)]
pub struct DecodeCommand {
    /// png file that you want to decode
    #[arg(value_name = "FILE")]
    path: PathBuf,
    /// chunk_type that you want to view
    chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    /// png file that you want to remove the massage of
    #[arg(value_name = "FILE")]
    path: PathBuf,
    /// the chunk_type that you want to remove
    chung_type: String,
}

#[derive(Debug, Args)]
pub struct PrintCommand {
    /// png file that you want to print
    #[arg(value_name = "FILE")]
    path: PathBuf,
}

impl PngsmCommand {
    pub fn handle(&self) -> Result<()> {
        match &self.subcommand {
            SubC::Encode(ec) => ec.handle(),
            SubC::Decode(dc) => dc.handle(),
            SubC::Remove(rm) => rm.handle(),
            SubC::Print(pr) => pr.handle(),
        }
    }
}

fn open_png(path: &PathBuf) -> Result<Png> {
    let png_file = std::fs::read(path)?;
    Png::try_from(&png_file[..])
}

impl EncodeCommand {
    fn handle(&self) -> Result<()> {
        let mut png = open_png(&self.path)?;

        let chunk_type = ChunkType::from_str(&self.chunk_type)?;
        let chunk = Chunk::new(chunk_type, (&self.message).clone().into_bytes());

        png.append_chunk(chunk);

        if self.output {
            println!("{}", self.message);
        }

        std::fs::write(&self.path, png.as_bytes())?;
        Ok(())
    }
}

impl DecodeCommand {
    fn handle(&self) -> Result<()> {
        let png = open_png(&self.path)?;
        let chunk = png.chunk_by_type(&self.chunk_type);
        match chunk {
            Some(x) => {
                println!("{}", x.data_as_string()?);
            }
            None => {
                println!("Chunk Type Not Found!");
            }
        }
        Ok(())
    }
}

impl RemoveCommand {
    fn handle(&self) -> Result<()> {
        let mut png = open_png(&self.path)?;

        png.remove_chunk(&self.chung_type)?;
        Ok(())
    }
}

impl PrintCommand {
    fn handle(&self) -> Result<()> {
        let png = open_png(&self.path)?;

        for chunk in png.chunks() {
            println!("{}: {}", chunk.chunk_type(), chunk.data_as_string()?);
        }

        Ok(())
    }
}
