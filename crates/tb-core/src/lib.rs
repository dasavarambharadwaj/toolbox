use thiserror::Error;

/// Root error enumeration for pure domain operations across Toolbox suites.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TbError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),

    #[error("Missing required engine: {0}")]
    EngineMissing(String),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("Execution interrupted")]
    Interrupted,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tb_error_display() {
        let err = TbError::InvalidArgument("test argument".to_string());
        assert_eq!(err.to_string(), "Invalid argument: test argument");

        let err = TbError::OperationFailed("conversion error".to_string());
        assert_eq!(err.to_string(), "Operation failed: conversion error");

        let err = TbError::EngineMissing("oxipng".to_string());
        assert_eq!(err.to_string(), "Missing required engine: oxipng");

        let err = TbError::Io("file not found".to_string());
        assert_eq!(err.to_string(), "I/O error: file not found");

        let err = TbError::Interrupted;
        assert_eq!(err.to_string(), "Execution interrupted");
    }
}
