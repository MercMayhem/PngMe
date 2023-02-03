use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::fs::{read, write};
use std::path::Path;

pub fn action_encode(args: EncodeArgs) -> Result<(), std::io::Error>{
    let bytes_array = bytes_from_png(&args.file_path);

    let mut png_file = Png::try_from(&bytes_array[..]).unwrap();

    let chunk_type:[u8; 4] = args.chunk_type.as_bytes().try_into().unwrap();
    let new_chunk_type = ChunkType::try_from(chunk_type).unwrap();
    let data = args.message.as_bytes().to_vec();
    let new_chunk = Chunk::new(new_chunk_type, data);

    png_file.append_chunk(new_chunk);

    png_from_bytes(&args.file_path, &png_file.as_bytes())
}

pub fn action_decode(args: DecodeArgs) -> Result<(), std::io::Error>{
    let bytes_array = bytes_from_png(&args.file_path);

    let png_file = Png::try_from(&bytes_array[..]).unwrap();
    let chunk = png_file.chunk_by_type(&args.chunk_type).expect("Chunk Not found");

    println!("Message: {}", chunk.data_as_string().unwrap());
    
    Ok(())
}

pub fn action_remove(args: RemoveArgs) -> Result<(), std::io::Error>{
    let bytes_array = bytes_from_png(&args.file_path);

    let mut png_file = Png::try_from(&bytes_array[..]).unwrap();

    png_file.remove_chunk(&args.chunk_type).unwrap();
    png_from_bytes(&args.file_path, &png_file.as_bytes())
}

pub fn action_print(args: PrintArgs) -> Result<(), std::io::Error>{
    let bytes_array = bytes_from_png(&args.file_path);

    let png_file = Png::try_from(&bytes_array[..]).unwrap();
    println!("Chunks:");
    for chunk in png_file.chunks().iter(){
        println!("{}", chunk.chunk_type())
    }
    Ok(())
}

fn bytes_from_png(file_path: &str)->Vec<u8>{
    let path = Path::new(file_path);
    let bytes = read(path);

    if bytes.is_err(){
        panic!("Couldn't read File")
    }

    bytes.unwrap()
}

fn png_from_bytes(file_path: &str, contents: &[u8])->Result<(), std::io::Error>{
    let path = Path::new(file_path);

    write(path, contents)
}