use thiserror::Error;

/// Standard exit code for successful execution.
pub const EXIT_SUCCESS: i32 = 0;
/// Standard exit code for operational / domain errors.
pub const EXIT_OPERATION_ERROR: i32 = 1;
/// Standard exit code for invalid CLI arguments or syntax.
pub const EXIT_INVALID_ARGUMENT: i32 = 2;
/// Standard exit code for missing external engine dependencies.
pub const EXIT_MISSING_ENGINE: i32 = 3;
/// Standard exit code for SIGINT interruption.
pub const EXIT_INTERRUPTED: i32 = 130;

/// Root error enumeration for pure domain operations across Toolbox suites.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
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

impl TbError {
    /// Returns the standard Unix process exit code corresponding to this domain error.
    pub fn exit_code(&self) -> i32 {
        match self {
            TbError::InvalidArgument(_) => EXIT_INVALID_ARGUMENT,
            TbError::OperationFailed(_) | TbError::Io(_) => EXIT_OPERATION_ERROR,
            TbError::EngineMissing(_) => EXIT_MISSING_ENGINE,
            TbError::Interrupted => EXIT_INTERRUPTED,
        }
    }

    /// Returns the standardized machine-readable error code string.
    pub fn error_code(&self) -> &'static str {
        match self {
            TbError::InvalidArgument(_) => "INVALID_ARGUMENT",
            TbError::OperationFailed(_) => "OPERATION_FAILED",
            TbError::Io(_) => "IO_ERROR",
            TbError::EngineMissing(_) => "ENGINE_MISSING",
            TbError::Interrupted => "INTERRUPTED",
        }
    }

    /// Returns an optional actionable suggestion for resolving the error.
    pub fn suggested_action(&self) -> Option<String> {
        match self {
            TbError::InvalidArgument(_) => {
                Some("Check 'tb --help' for valid options and usage.".to_string())
            }
            TbError::EngineMissing(engine) => {
                Some(format!("Install missing dependency: {engine}"))
            }
            _ => None,
        }
    }
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

    #[test]
    fn test_tb_error_exit_and_error_codes() {
        assert_eq!(EXIT_SUCCESS, 0);
        assert_eq!(EXIT_OPERATION_ERROR, 1);
        assert_eq!(EXIT_INVALID_ARGUMENT, 2);
        assert_eq!(EXIT_MISSING_ENGINE, 3);
        assert_eq!(EXIT_INTERRUPTED, 130);

        let err_inv = TbError::InvalidArgument("bad flag".to_string());
        assert_eq!(err_inv.exit_code(), 2);
        assert_eq!(err_inv.error_code(), "INVALID_ARGUMENT");

        let err_op = TbError::OperationFailed("failed op".to_string());
        assert_eq!(err_op.exit_code(), 1);
        assert_eq!(err_op.error_code(), "OPERATION_FAILED");

        let err_io = TbError::Io("not found".to_string());
        assert_eq!(err_io.exit_code(), 1);
        assert_eq!(err_io.error_code(), "IO_ERROR");

        let err_eng = TbError::EngineMissing("oxipng".to_string());
        assert_eq!(err_eng.exit_code(), 3);
        assert_eq!(err_eng.error_code(), "ENGINE_MISSING");

        let err_int = TbError::Interrupted;
        assert_eq!(err_int.exit_code(), 130);
        assert_eq!(err_int.error_code(), "INTERRUPTED");
    }

    #[test]
    fn test_tb_error_suggested_action() {
        let err_inv = TbError::InvalidArgument("bad flag".to_string());
        assert_eq!(
            err_inv.suggested_action(),
            Some("Check 'tb --help' for valid options and usage.".to_string())
        );

        let err_eng = TbError::EngineMissing("oxipng".to_string());
        assert_eq!(
            err_eng.suggested_action(),
            Some("Install missing dependency: oxipng".to_string())
        );

        let err_op = TbError::OperationFailed("failed op".to_string());
        assert_eq!(err_op.suggested_action(), None);

        let err_io = TbError::Io("not found".to_string());
        assert_eq!(err_io.suggested_action(), None);

        let err_int = TbError::Interrupted;
        assert_eq!(err_int.suggested_action(), None);
    }
}
