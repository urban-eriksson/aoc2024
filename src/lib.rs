use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn fetch_or_load_input(day: u32) -> Result<String, Box<dyn Error>> {
    let input_dir = "inputs";
    let input_file = format!("{}/input_day{}.txt", input_dir, day);

    if Path::new(&input_file).exists() {
        let input_text = fs::read_to_string(&input_file)?;
        println!("Using cached input from {}", input_file);
        Ok(input_text)
    } else {
        let session_cookie = env::var("AOC_SESSION")
            .map_err(|_| "Environment variable AOC_SESSION is not set or invalid")?;

        let client = Client::builder()
            .cookie_store(true)
            .build()?;

        let url = format!("https://adventofcode.com/2024/day/{}/input", day);

        let response = client
            .get(&url)
            .header(COOKIE, format!("session={}", session_cookie))
            .send()?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let input_text = response.text()?;
                fs::write(&input_file, &input_text)?;
                println!("Fetched and cached input to {}", input_file);
                Ok(input_text)
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                Err("Unauthorized: Invalid session cookie.".into())
            }
            reqwest::StatusCode::NOT_FOUND => {
                Err("Not found: The requested resource does not exist.".into())
            }
            status => Err(format!("Failed to fetch input: HTTP {}", status).into()),
        }
    }    

}