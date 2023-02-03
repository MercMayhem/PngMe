use clap:: {Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct PngArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Encode message into PNG File
    Encode(EncodeArgs),
    
    /// Decode message from PNG File
    Decode(DecodeArgs),
    
    /// Remove message from PNG File
    Remove(RemoveArgs),

    /// Print PNG File 
    Print(PrintArgs)
}

#[derive(Debug, Args)]
pub struct EncodeArgs{
    /// File Path
    pub file_path: String,

    /// Chunk Type
    pub chunk_type: String,

    /// Message
    pub message: String,

    /// Output File Name (Optional)
    pub output_file: Option<String>
}

#[derive(Debug, Args)]
pub struct DecodeArgs{
    /// File Path
    pub file_path: String,

    /// Chunk Type
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveArgs{
    /// File Path
    pub file_path: String,

    /// Chunk Type
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs{
    /// File Path
    pub file_path: String,
}