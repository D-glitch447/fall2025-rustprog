use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::copy;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(String),          // Stores file path of downloaded image
    ApiError(String),
    NetworkError(String),
    FileError(String),
}

fn fetch_and_download_image(i: usize) -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";

    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => {
                        let image_url = dog_image.message;
                        println!("🖼️ Image URL: {}", image_url);

                        match ureq::get(&image_url).call() {
                            Ok(image_response) => {
                                let filename = format!("dog_image_{}.jpg", i);
                                let path = Path::new(&filename);

                                match File::create(path) {
                                    Ok(mut file) => {
                                        let mut reader = image_response.into_reader();
                                        if let Err(e) = copy(&mut reader, &mut file) {
                                            return ApiResult::FileError(format!(
                                                "Failed to write file: {}",
                                                e
                                            ));
                                        }
                                        ApiResult::Success(filename)
                                    }
                                    Err(e) => {
                                        ApiResult::FileError(format!("Failed to create file: {}", e))
                                    }
                                }
                            }
                            Err(e) => ApiResult::NetworkError(format!(
                                "Failed to download image: {}",
                                e
                            )),
                        }
                    }
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Downloader 🐶");
    println!("========================\n");

    for i in 1..=5 {
        println!("Fetching and downloading dog image #{}", i);
        match fetch_and_download_image(i) {
            ApiResult::Success(file_path) => {
                println!("✅ Image saved to '{}'\n", file_path);
            }
            ApiResult::ApiError(e)
            | ApiResult::NetworkError(e)
            | ApiResult::FileError(e) => println!("❌ Error: {}\n", e),
        }
    }

    println!("All done! Check your folder for downloaded images 🐾");
    Ok(())
}
