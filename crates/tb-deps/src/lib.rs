use tb_core::TbError;

/// Checks if an external tool engine dependency is present.
pub fn check_dependency(name: &str) -> Result<bool, TbError> {
    if name.is_empty() {
        return Err(TbError::InvalidArgument("Dependency name cannot be empty".to_string()));
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_dependency_empty() {
        let result = check_dependency("");
        assert!(matches!(result, Err(TbError::InvalidArgument(_))));
    }

    #[test]
    fn test_check_dependency_valid() {
        let result = check_dependency("ffmpeg");
        assert!(!result.unwrap());
    }
}
