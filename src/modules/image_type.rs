use colored::Colorize;
use std::fs;
use std::path::Path;

pub fn get_image_type() -> String {
    let image_type_file: Option<&Path>;

    if Path::new("/usr/share/horizon").exists() {
        image_type_file = Some(Path::new("/usr/share/horizon/image_type"));
    } else if Path::new("/usr/share/nova/image_type").exists() {
        image_type_file = Some(Path::new("/usr/share/nova/image_type"));
    } else if Path::new("/usr/share/umbra/image_type").exists() {
        image_type_file = Some(Path::new("/usr/share/umbra/image_type"));
    } else {
        // Return an error if no image type file is found
        panic!("{}: No image type file found!", "ERROR".bold().red());
    }

    let image = fs::read_to_string(image_type_file.unwrap()).expect("Failed to get image type");

    let image_name = image.trim().to_string();

    return image_name;
}

