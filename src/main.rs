use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("/test_log.log")?;

    let reader = BufReader::new(file);

    // Create the regex using a raw string literal with correct format
    let re = Regex::new(r#"\[(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})\] "(.*?)" \[(\d{3})\]"#).unwrap();

    // Read lines and apply regex
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            let timestamp = caps.get(1).map_or("Unavailable", |m| m.as_str());
            let message = caps.get(2).map_or("Unavailable", |m| m.as_str());
            let status_code = caps.get(3).map_or("Unavailable", |m| m.as_str());

            println!("Timestamp: {}", timestamp);
            println!("Message: {}", message);
            println!("Status Code: {}", status_code);
        } else {
            println!("Line didn't match the format");
        }
    }

    Ok(())
}
