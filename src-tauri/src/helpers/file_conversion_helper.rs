use std::fs::File;
use std::io::Read;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use mime_guess::from_path;

pub fn image_to_base64(path: &str) -> Result<String, std::io::Error> {
    // Open the file in read mode
    let mut file = File::open(path)?;

    // Create a vector to hold the image bytes
    let mut buffer = Vec::new();

    // Read the file into the byte array
    file.read_to_end(&mut buffer)?;

    // Encode the byte array into a Base64 string using the STANDARD engine
    let encoded = STANDARD.encode(&buffer);

    // Determine the MIME type of the file
    let mime = from_path(path).first_or_octet_stream();
    let mime_type = mime.as_ref();

    // Add the correct header to the encoded string
    let result = format!("data:{};base64,{}", mime_type, encoded);

    Ok(result)
}