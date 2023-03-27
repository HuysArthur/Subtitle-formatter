use std::{fs::{read_to_string, write}, path::PathBuf, io};

pub fn read_content_file(path: PathBuf) -> io::Result<String> {
    read_to_string(path)
}

pub fn write_content_file(content: String, path: PathBuf) -> Result<(), io::Error> {
    write(path, content)
}