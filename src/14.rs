use std::{
    fs::{create_dir, write},
};

fn main() {
    let filename = "example.txt";
    let dirname = "new_directory";

    // Create a new directory
    create_dir(dirname).expect("failed to create directory");

    // Write some content to the file
    write(filename, "Hello, world!").unwrap();
}
