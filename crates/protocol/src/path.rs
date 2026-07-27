use serde::{Deserialize, Serialize};

use crate::{PathError, limits::MAX_PATH_BYTES};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct RelativePath(String);

impl RelativePath {
    pub fn parse(value: impl Into<String>) -> Result<Self, PathError> {
        let value = value.into();

        validate(&value)?;

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl<'de> Deserialize<'de> for RelativePath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for RelativePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

fn validate(path: &str) -> Result<(), PathError> {
    if path.is_empty() {
        return Err(PathError::Empty);
    }

    if path.len() > MAX_PATH_BYTES {
        return Err(PathError::TooLong { actual: path.len(), maximum: MAX_PATH_BYTES });
    }

    if path.contains('\0') {
        return Err(PathError::NullByte);
    }

    if path.contains('\\') {
        return Err(PathError::InvalidSeparator);
    }

    if path.starts_with('/') || has_windows_drive_prefix(path) {
        return Err(PathError::Absolute);
    }

    for component in path.split('/') {
        match component {
            "" => return Err(PathError::EmptyComponent),
            "." => return Err(PathError::CurrentDirectory),
            ".." => return Err(PathError::ParentTraversal),
            _ => {}
        }
    }

    Ok(())
}

fn has_windows_drive_prefix(path: &str) -> bool {
    let bytes = path.as_bytes();

    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_simple_file() {
        let path = RelativePath::parse("cat.jpg").unwrap();

        assert_eq!(path.as_str(), "cat.jpg");
    }

    #[test]
    fn accepts_nested_path() {
        let path = RelativePath::parse("photos/2026/cat.jpg").unwrap();

        assert_eq!(path.as_str(), "photos/2026/cat.jpg");
    }

    #[test]
    fn accepts_unicode() {
        let path = RelativePath::parse("写真/猫.jpg").unwrap();

        assert_eq!(path.as_str(), "写真/猫.jpg");
    }

    #[test]
    fn rejects_empty_path() {
        assert_eq!(RelativePath::parse(""), Err(PathError::Empty));
    }

    #[test]
    fn rejects_unix_absolute_path() {
        assert_eq!(RelativePath::parse("/etc/passwd"), Err(PathError::Absolute));
    }

    #[test]
    fn rejects_windows_absolute_path() {
        assert_eq!(RelativePath::parse("C:/Users/test.txt"), Err(PathError::Absolute));
    }

    #[test]
    fn rejects_parent_traversal() {
        assert_eq!(RelativePath::parse("photos/../../secret.txt"), Err(PathError::ParentTraversal));
    }

    #[test]
    fn rejects_current_directory_component() {
        assert_eq!(RelativePath::parse("photos/./cat.jpg"), Err(PathError::CurrentDirectory));
    }

    #[test]
    fn rejects_duplicate_separators() {
        assert_eq!(RelativePath::parse("photos//cat.jpg"), Err(PathError::EmptyComponent));
    }

    #[test]
    fn rejects_trailing_separator() {
        assert_eq!(RelativePath::parse("photos/"), Err(PathError::EmptyComponent));
    }

    #[test]
    fn rejects_backslashes() {
        assert_eq!(RelativePath::parse(r"photos\cat.jpg"), Err(PathError::InvalidSeparator));
    }

    #[test]
    fn rejects_null_bytes() {
        assert_eq!(RelativePath::parse("photos/\0cat.jpg"), Err(PathError::NullByte));
    }
}
