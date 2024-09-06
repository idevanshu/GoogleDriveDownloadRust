use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use reqwest::blocking::Client;
use indicatif::{ProgressBar, ProgressStyle};

fn extract_file_id_from_url(url: &str) -> Option<String> {
    url.split('/').nth(5).map(String::from)
}

fn convert_to_direct_download_url(file_id: &str) -> String {
    format!("https://drive.google.com/uc?export=download&id={}", file_id)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::args().nth(1).ok_or("Please provide a URL as an argument")?;

    // Extract file ID and construct the direct download URL
    let file_id = extract_file_id_from_url(&url).ok_or("Failed to extract file ID")?;
    let direct_url = convert_to_direct_download_url(&file_id);

    let client = Client::new();
    println!("Using URL: {}", direct_url); 
    let response = client.get(&direct_url).send()?;
    if response.status().is_success() {
        let total_size = response.content_length().ok_or("Failed to get content length")?;
        
        let filename = response
            .headers()
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                header.split(';').find_map(|part| {
                    let mut split = part.trim().split('=');
                    if split.next()? == "filename" {
                        Some(split.next()?.trim_matches('"').to_string())
                    } else {
                        None
                    }
                })
            })
            .unwrap_or_else(|| "downloaded_file".to_string());

        let progress_bar = ProgressBar::new(total_size);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        progress_bar.set_message(format!("Downloading {}", filename));

        let mut file = BufWriter::new(File::create(&filename)?);
        let mut buffer = vec![0; 8192];

        let mut downloaded: u64 = 0;
        let mut reader = BufReader::new(response);

        while let Ok(chunk_size) = reader.read(&mut buffer) {
            if chunk_size == 0 {
                break;
            }
            file.write_all(&buffer[..chunk_size])?;
            downloaded += chunk_size as u64;
            progress_bar.set_position(downloaded);
        }

        progress_bar.finish_with_message("Download complete!");
    } else {
        eprintln!("Failed to download the file: HTTP {}", response.status());
    }

    Ok(())
}
