#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use subtitle_formatter::utils;

    #[test]
    pub fn read_write_file() {
        let path: PathBuf = PathBuf::from(r"D:\Documents\Projects\subtitle-formatter\resources\tests\example.srt");

        if let Ok(before) = utils::read_content_file(path.clone()) {
            utils::write_content_file(before.clone(), path.clone());
            if let Ok(after) = utils::read_content_file(path.clone()) {
                assert_eq!(before, after);
            }
        }
    }

    #[test]
    pub fn erace_content_file() {
        use std::fs::File;
        use std::fs::remove_file;

        let path: PathBuf = PathBuf::from(r"D:\Documents\Projects\subtitle-formatter\resources\tests\test.srt");

        if let Ok(_) = File::create(path.clone()) {
            utils::write_content_file("test".to_string(), path.clone());
            if let Ok(after) = utils::read_content_file(path.clone()) {
                remove_file(path.clone()).ok();
    
                assert_eq!(after, "test".to_string());
            }
        }
    }
}