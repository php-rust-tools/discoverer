use std::path::PathBuf;
use walkdir::{WalkDir, Error};

pub fn discover(extension: &str, within: &[&str]) -> Result<Vec<PathBuf>, Error> {
    let mut paths = Vec::new();

    for directory in within {
        let walker = WalkDir::new(directory);

        for entry in walker {
            let entry = entry?;
            let path = entry.path();

            if ! path.is_file() {
                continue;
            }

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == extension {
                        paths.push(path.to_path_buf());
                    }
                }
            }
        }
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn it_works() {
        let paths = super::discover("rs", &["src"]).unwrap();

        assert_eq!(paths.len(), 1);
        assert_eq!(paths.first(), Some(&PathBuf::from("src/lib.rs")));
    }
}
