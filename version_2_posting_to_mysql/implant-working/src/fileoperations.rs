use std::fs::File;
use std::io::{Read, BufReader, Write};
use std::path::Path;


// This function takes a command string as input, extracts the file name from it,
// opens the file, reads its content, and returns the content as a string.
pub fn run_download(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    
    // Split the command string by spaces and collect the parts into a vector.
    let args: Vec<&str> = command.split(" ").collect();

    // Check if there are at least two parts after splitting by spaces
    if args.len() < 2 {
        return Err("Invalid command format. Expected format: download <file_name>".into());
    }

    // Open the file with the extracted file name (args[1]).
    let file: File = File::open(&args[1])?;

    // Create a buffered reader for the opened file.
    let mut reader = BufReader::new(file);

    // Create a mutable buffer to store the file content.
    let mut buffer: Vec<u8> = Vec::new();

    // Read the file content into the buffer.
    reader.read_to_end(&mut buffer)?;

    // Convert the buffer (Vec<u8>) into a string slice (&str).
    let data: &str = std::str::from_utf8(&buffer)?;

    // Convert the string slice into a String and return it.
    Ok(std::string::String::from(data))
}
pub fn run_push(command: &str, file_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<&str> = command.split(" ").collect();
    if args.len() < 3 {
        return Err("Invalid command format. Expected format: push <source_file> <destination_file>".into());
    }
    let destination_path = Path::new(args[2]);
    let mut file = File::create(destination_path)?;
    file.write_all(file_data)?;
    Ok(())
}


