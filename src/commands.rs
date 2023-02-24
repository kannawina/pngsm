use clap::{Args, Parser, Subcommand};

use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct PngsmCommand {
    /// png file
    #[arg(value_name = "FILE")]
    path: PathBuf,
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
    /// chunk_type that you want to view
    chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    /// the chunk_type that you want to remove
    chung_type: String,
}

#[derive(Debug, Args)]
pub struct PrintCommand {}

impl PngsmCommand {
    pub fn handle(&self) -> Result<()> {
        match &self.subcommand {
            SubC::Encode(ec) => ec.handle(&self.path),
            SubC::Decode(dc) => dc.handle(&self.path),
            SubC::Remove(rm) => rm.handle(&self.path),
            SubC::Print(pr) => pr.handle(&self.path),
        }
    }
}

fn open_png(path: &PathBuf) -> Result<Png> {
    let png_file = std::fs::read(path)?;
    Png::try_from(&png_file[..])
}

impl EncodeCommand {
    fn handle(&self, path: &PathBuf) -> Result<()> {
        let mut png = open_png(path)?;

        let chunk_type = ChunkType::from_str(&self.chunk_type)?;

        if !chunk_type.is_valid() {
            return Err(format!(
                "invalid chunk type of {}. the third character must be uppercase",
                chunk_type
            )
            .into());
        }

        let chunk = Chunk::new(chunk_type, (&self.message).clone().into_bytes());

        png.append_chunk(chunk);

        if self.output {
            println!("{}", self.message);
        }

        std::fs::write(path, png.as_bytes())?;
        Ok(())
    }
}

impl DecodeCommand {
    fn handle(&self, path: &PathBuf) -> Result<()> {
        let png = open_png(path)?;
        match png.chunk_by_type(&self.chunk_type) {
            Some(chunks) => {
                for chunk in chunks {
                    println!(
                        "chunk type : {}\nmessage : {}\n",
                        chunk.chunk_type(),
                        chunk.data_as_string()?
                    );
                }
            }
            None => println!("No chunk that have chunk type of {}", self.chunk_type),
        }
        Ok(())
    }
}

impl RemoveCommand {
    fn handle(&self, path: &PathBuf) -> Result<()> {
        let mut png = open_png(path)?;

        let mut chunks: Vec<Chunk> = Vec::new();

        while let Ok(chunk) = png.remove_chunk(&self.chung_type) {
            chunks.push(chunk);
        }

        if chunks.len() < 2 {
            return Ok(());
        }

        for (index, chunk) in chunks.iter().enumerate() {
            println!(
                "chunk : {}\nchunk type : {}\nmessage : {}",
                index,
                chunk.chunk_type(),
                chunk.data_as_string()?
            );
        }

        print!(
            "chose chunk to remove (0-{}, default = 0) :",
            chunks.len() - 1
        );
        std::io::stdout().flush()?;

        let mut user_input = String::new();

        std::io::stdin().read_line(&mut user_input)?;

        let user_input = user_input.trim();

        if user_input.len() == 0 {
            for chunk in chunks.into_iter().skip(1) {
                png.append_chunk(chunk);
            }
        } else {
            let remove_index = user_input.parse::<usize>()?;
            if remove_index > chunks.len() - 1 {
                return Err("it's bigger than displayd chunks len".into());
            }

            for (index, chunk) in chunks.into_iter().enumerate() {
                if index == remove_index {
                    continue;
                }
                png.append_chunk(chunk);
            }
        }

        std::fs::write(path, png.as_bytes())?;
        Ok(())
    }
}

impl PrintCommand {
    fn handle(&self, path: &PathBuf) -> Result<()> {
        let png = open_png(path)?;

        println!();
        for chunk in png.chunks() {
            if let Ok(message) = chunk.data_as_string() {
                println!("chunk type : {}\nmessage : {}", chunk.chunk_type(), message);
                println!();
            }
        }

        Ok(())
    }
}
