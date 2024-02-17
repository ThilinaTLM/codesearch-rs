use std::path::Path;

use validator::ValidationError;

/// Validate that a path exists
///
/// # Arguments
///
/// * `path` - The path to validate
///
/// # Returns
///
/// A Result containing either `()` if the path exists, or a `ValidationError` if the path does not exist
pub(crate) fn validate_path_exists(path: &str) -> Result<(), ValidationError> {
    let path_obj = Path::new(path);
    if !path_obj.exists() {
        let mut error = ValidationError::new("path_exists");
        error.message = Some("The specified path does not exist".into());
        Err(error)
    } else {
        Ok(())
    }
}


/// Validate that at least one item is in the vector
///
/// # Arguments
///
/// * `vec` - The vector to validate
///
/// # Returns
///
/// A Result containing either `()` if the vector is not empty, or a `ValidationError` if the vector is empty
pub(crate) fn validate_at_least_one_item<T>(vec: &Vec<T>) -> Result<(), ValidationError> {
    if vec.is_empty() {
        let mut error = ValidationError::new("at_least_one_item");
        error.message = Some("At least one item must be provided".into());
        Err(error)
    } else {
        Ok(())
    }
}


/// Validate that a string in one of the specified strings
///
/// # Arguments
///
/// * `string` - The string to validate
/// * `patterns` - The patterns to check for
///
/// # Returns
///
/// A Result containing either `()` if the string is in one of the patterns, or a `ValidationError` if the string is not in one of the patterns
pub(crate) fn validate_string_in_patterns(string: &str, patterns: &[&str]) -> Result<(), ValidationError> {
    if !patterns.contains(&string) {
        let mut error = ValidationError::new("string_in_patterns");
        error.message = Some(format!("The specified string is not in the list of patterns: {:?}", patterns).into());
        Err(error)
    } else {
        Ok(())
    }
}