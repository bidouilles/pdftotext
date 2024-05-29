use pdf_extract::extract_text_from_mem;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Extracts text from a PDF file at the given path.
///
/// # Arguments
///
/// * `path` - A path to the PDF file to be processed.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok`: Contains the extracted text as a `String` if the extraction is successful.
/// - `Err`: Contains a boxed `dyn std::error::Error` if an error occurs during the process.
fn extract_text_from_document<P: AsRef<Path>>(
    path: P,
) -> Result<String, Box<dyn std::error::Error>> {
    // Read the PDF file into a byte array
    let bytes = fs::read(path)?;
    // Extract text from the byte array using the pdf_extract crate
    let text = extract_text_from_mem(&bytes)?;
    Ok(text)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the correct number of arguments are provided
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_pdf>", args[0]);
        std::process::exit(1);
    }

    // Get the PDF file path from the arguments
    let pdf_path = &args[1];

    // Extract text from the PDF and handle the result
    match extract_text_from_document(pdf_path) {
        Ok(text) => {
            // Write the extracted text to stdout
            io::stdout().write_all(text.as_bytes())?;
            Ok(())
        }
        Err(e) => {
            // Print the error message to stderr and exit with a non-zero status
            eprintln!("Error processing file {}: {}", pdf_path, e);
            std::process::exit(1);
        }
    }
}
