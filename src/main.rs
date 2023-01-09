use std::{env, fs, path::Path};

use goblin::Object;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = Path::new(args.get(1).unwrap());

    let buffer = fs::read(input_path).unwrap();

    if let Object::PE(pe) = Object::parse(&buffer).unwrap() {
        let optional_header = pe.header.optional_header.unwrap();
        let size_of_image = optional_header.windows_fields.size_of_image as usize;
        let trimmed = &buffer[..size_of_image];

        let original_filename = input_path.file_stem().unwrap().to_str().unwrap();
        let original_extension = input_path.extension().unwrap();
        let output_path = input_path
            .with_file_name(format!("{}-trimmed", original_filename))
            .with_extension(original_extension);
        fs::write(output_path, trimmed).unwrap();
    }
}
