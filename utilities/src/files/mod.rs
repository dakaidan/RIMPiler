use std::fs::File;
use std::io::Read;

pub fn load_file(path: &str) -> Result<String, String> {
    let file = File::open(path).map_err(|e| e.to_string());
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            let result = file
                .read_to_string(&mut contents)
                .map_err(|e| e.to_string());
            match result {
                Ok(_) => Ok(contents),
                Err(e) => Err(e.to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
