#[cfg(test)]
mod tests {
    use std::{path::PathBuf, fs::{File, remove_file}};

    use subtitle_formatter::utils;

    #[test]
    pub fn read_write_file() {
        let path: PathBuf = PathBuf::from(r"resources\tests\example.srt");

        if let Ok(before) = utils::read_content_file(path.clone()) {
            utils::write_content_file(before.clone(), path.clone());
            if let Ok(after) = utils::read_content_file(path.clone()) {
                assert_eq!(before, after);
            }
        }
    }

    #[test]
    pub fn erace_content_file() {
        let path: PathBuf = PathBuf::from(r"resources\tests\test.srt");

        if let Ok(_) = File::create(path.clone()) {
            utils::write_content_file("test".as_bytes().to_vec(), path.clone());
            if let Ok(after) = utils::read_content_file(path.clone()) {
                remove_file(path.clone()).ok();
    
                assert_eq!(after, "test".as_bytes());
            }
        }
    }

    #[test]
    pub fn format_subtitle() {
        let path: PathBuf = PathBuf::from("rules.json");
        if let Ok(rules_json) = utils::read_json_file(path.clone()) {
            let mut content: String = String::from("");
            for (key, _) in rules_json.as_object().unwrap() {
                content += key;
                content += "\r\n";
            }

            let path_content: PathBuf = PathBuf::from(r"resources\tests\content.srt");
            if let Ok(_) = File::create(path_content.clone()) {
                utils::write_content_file(content.as_bytes().to_vec(), path_content.clone());

                let mut content_result: String = String::from("");
                for (_, value) in rules_json.as_object().unwrap() {
                    content_result += value.as_str().unwrap();
                    content_result += "\r\n";
                }
                let path_content_result: PathBuf = PathBuf::from(r"resources\tests\content_result.srt");
                if let Ok(_) = File::create(path_content_result.clone()) {
                    utils::write_content_file(content_result.as_bytes().to_vec(), path_content_result.clone());
 
                    utils::format_subtitle(utils::read_json_file(path.clone()).unwrap(), path_content.clone());

                    assert_eq!(utils::read_content_file(path_content.clone()).unwrap(), utils::read_content_file(path_content_result.clone()).unwrap());
                
                    remove_file(path_content.clone()).ok();
                    remove_file(path_content_result.clone()).ok();
                }
            }
        }
    }
}