use std::fs;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use mime_guess::from_path;
use std::fs::File;
use std::io::Read;
use log::info;

/// This function is used to convert an image file to a Base64 string
/// It returns a Result containing the Base64 string or an error
/// The function takes a path to the image file as an argument
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

/// This function is used to remove a file from the filesystem (used to remove AppImages and icons)
/// It returns a boolean indicating if the file was removed successfully
pub fn rm_file(file_path: &String) -> Result<bool, String> {
    match fs::remove_file(file_path) {
        Ok(_) => {
            info!("File removed successfully");
            Ok(true)
        },
        Err(e) => Err(format!("Failed to remove file: {}", e)),
    }
}
