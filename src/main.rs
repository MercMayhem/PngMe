mod png;
mod chunk;
mod chunk_type;
mod commands;
mod args;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use args::{PngArgs, EntityType};
use clap::Parser;

use commands::{action_encode, action_decode, action_print, action_remove};

fn main() -> Result<()> {
    let cli_command = PngArgs::parse();

    match cli_command.entity_type {
        EntityType::Encode(encode_args) => action_encode(encode_args)?,
        EntityType::Decode(decode_args) => action_decode(decode_args)?,
        EntityType::Remove(remove_args) => action_remove(remove_args)?,
        EntityType::Print(print_args) => action_print(print_args)?
    };

    Ok(())
}