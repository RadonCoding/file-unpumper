use std::{cmp, env, fs, path::Path};
use goblin::Object;

fn main() {
    // Get command-line arguments as a vector of strings
    let args: Vec<String> = env::args().collect();

    // Get the path to the input file from the command-line arguments
    let input_path = Path::new(args.get(1).unwrap());

    // Read the contents of the input file into a buffer
    let buffer = fs::read(input_path).unwrap();

    // Parse the buffer as a PE file using the goblin library
    if let Object::PE(pe) = Object::parse(&buffer).unwrap() {
        // Get the optional header from the PE file
        let optional_header = pe.header.optional_header.unwrap();
        // Get the size of image field from the optional header
        let size_of_image = optional_header.windows_fields.size_of_image as usize;

        // Initialize the eof_offset to the end of buffer
        let mut eof_offset = buffer.len();
        // Iterate through the buffer in reverse order starting from the last byte
        for i in (0..buffer.len()).rev() {
            // Check if the current byte is not 0x30
            if buffer[i] != 0x30 {
                // Save the index and break the loop
                eof_offset = i+1;
                break;
            }
        }
        // get the minimum value of both size_of_image and eof_offset
        let eof_offset = cmp::min(size_of_image, eof_offset);

        // Create a new slice of the buffer that contains only the bytes up to eof_offset
        let trimmed = &buffer[..eof_offset];

        // Get the original filename and extension of the input file
        let original_filename = input_path.file_stem().unwrap().to_str().unwrap();
        let original_extension = input_path.extension().unwrap();

        // Create the output file path by adding "-trimmed" to the original filename and keeping the original extension
        let output_path = input_path
            .with_file_name(format!("{}-trimmed", original_filename))
            .with_extension(original_extension);

        // Write the trimmed buffer to the output file
        fs::write(output_path, trimmed).unwrap();
    }
}
