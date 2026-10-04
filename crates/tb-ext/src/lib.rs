use tb_core::TbError;

/// Lists installed extensions.
pub fn list_extensions() -> Result<Vec<String>, TbError> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_extensions() {
        let extensions = list_extensions().expect("list_extensions should succeed");
        assert!(extensions.is_empty());
    }
}
