use std::{fs::File, io::Read, path::Path};

pub(crate) fn read_document(path: &Path) -> Result<Vec<u8>, String> {
    if !path
        .symlink_metadata()
        .map_err(|e| e.to_string())?
        .is_file()
    {
        return Err("input must be a regular file".into());
    }
    let file = File::open(path).map_err(|e| e.to_string())?;
    if !file.metadata().map_err(|e| e.to_string())?.is_file() {
        return Err("input must be a regular file".into());
    }
    read_bounded(file)
}

// Read at most one byte beyond the parser limit; never allocate the full input.
pub(crate) fn read_bounded(reader: impl Read) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    reader
        .take(1_048_577)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() > 1_048_576 {
        return Err("input exceeds 1 MiB".into());
    }
    Ok(bytes)
}

#[cfg(test)]
mod bounded_input_tests {
    use super::{read_bounded, read_document};
    #[test]
    fn rejects_special_files_and_accepts_regular_input() {
        let root = std::env::temp_dir().join(format!("worker-input-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("test directory");
        let file = root.join("input.json");
        std::fs::write(&file, b"{}").expect("normal input");
        assert_eq!(read_document(&file).expect("regular input"), b"{}");
        assert!(read_document(&root).is_err());
        #[cfg(unix)]
        {
            let link = root.join("link");
            std::os::unix::fs::symlink(&file, &link).expect("symlink fixture");
            assert!(read_document(&link).is_err());
            let fifo = root.join("fifo");
            assert!(
                std::process::Command::new("mkfifo")
                    .arg(&fifo)
                    .status()
                    .expect("FIFO fixture")
                    .success()
            );
            assert!(read_document(&fifo).is_err());
        }
        std::fs::remove_dir_all(root).expect("test cleanup");
    }
    #[test]
    fn accepts_exact_limit_and_rejects_unbounded_input() {
        assert_eq!(
            read_bounded(&vec![0; 1_048_576][..]).unwrap().len(),
            1_048_576
        );
        assert!(read_bounded(std::io::repeat(0)).is_err());
        assert_eq!(read_bounded(&b"{}"[..]).unwrap(), b"{}");
    }
}
