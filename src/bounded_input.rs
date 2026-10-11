use std::io::Read;

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
    use super::read_bounded;
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
