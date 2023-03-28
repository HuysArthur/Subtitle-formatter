use std::{fs::{read_to_string, write}, path::PathBuf, io};

use serde_json::Value;

pub fn read_content_file(path: PathBuf) -> io::Result<String> {
    read_to_string(path)
}

pub fn write_content_file(content: String, path: PathBuf) -> Result<(), io::Error> {
    write(path, content)
}

pub fn read_json_file(path: PathBuf) -> Result<Value, io::Error> {
    match serde_json::from_str::<serde_json::Value>(&read_content_file(path)?) {
        Ok(result) => Ok(result),
        Err(_) => Err(io::ErrorKind::InvalidData.into())
    }
}

pub fn format_subtitle(rules: Value, path: PathBuf) -> Result<(), io::Error> {
    let mut content = read_content_file(path.clone())?;
    for (key, value) in rules.as_object().unwrap() {
        content = content.replace(key, value.as_str().unwrap());
    }
    
    write_content_file(content, path)
}
