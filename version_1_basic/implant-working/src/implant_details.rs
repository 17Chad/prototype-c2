use std::process::Command;
use std::string::FromUtf8Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::from_utf8;

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
