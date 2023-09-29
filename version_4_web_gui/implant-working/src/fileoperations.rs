use std::fs::File;
use std::io::{Read, BufReader, Write};
use std::path::Path;

//Needs to be fixed. Something is probably wrong with opening and closing data connection
pub fn run_download(command: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let args: Vec<&str> = command.split_whitespace().collect();

    if args.len() < 2 {
        return Err("Invalid command format. Expected format: download <file_name>".into());
    }

    let file: File = File::open(&args[1])?;
    let mut reader = BufReader::new(file);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn run_push(command: &str, file_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<&str> = command.split_whitespace().collect();
    if args.len() < 3 {
        return Err("Invalid command format. Expected format: push <source_file> <destination_file>".into());
    }
    let destination_path = Path::new(args[2]);
    let mut file = File::create(destination_path)?;
    file.write_all(file_data)?;
    Ok(())
}
