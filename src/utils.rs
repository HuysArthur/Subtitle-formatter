use std::{fs::{read, write}, path::PathBuf, io};

use serde_json::Value;

pub fn read_content_file(path: PathBuf) -> io::Result<Vec<u8>> {
    read(path)
}

pub fn write_content_file(content: Vec<u8>, path: PathBuf) -> Result<(), io::Error> {
    write(path, content)
}

pub fn read_json_file(path: PathBuf) -> Result<Value, io::Error> {
    match serde_json::from_slice::<serde_json::Value>(&read_content_file(path)?) {
        Ok(result) => Ok(result),
        Err(_) => Err(io::ErrorKind::InvalidData.into())
    }
}

pub fn format_subtitle(rules: Value, path: PathBuf) -> Result<(), io::Error> {
    let mut content = read_content_file(path.clone())?;
    for (key, value) in rules.as_object().unwrap() {
        content = replace_bytes(&content, key.as_bytes(), value.as_str().unwrap().as_bytes());
    }

    write_content_file(content, path)
}

fn replace_bytes(input: &[u8], from: &[u8], to: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut pos = 0;

    while let Some(i) = input[pos..].windows(from.len()).position(|window| window == from) {
        output.extend_from_slice(&input[pos..pos+i]);
        output.extend_from_slice(to);
        pos += i + from.len();
    }

    output.extend_from_slice(&input[pos..]);

    output
}
