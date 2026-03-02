#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;

    // Test using the format_size function directly at the crate level
    #[test]
    fn test_format_size_functionality() {
        assert_eq!(rust_cache_cleaner_lib::format_size(0), "0 B");
        assert_eq!(rust_cache_cleaner_lib::format_size(512), "512 B");
        assert_eq!(rust_cache_cleaner_lib::format_size(1024), "1.00 KB");
        assert_eq!(rust_cache_cleaner_lib::format_size(1025), "1.00 KB"); // Rounded
        assert_eq!(rust_cache_cleaner_lib::format_size(1048575), "1024.00 KB"); // Just under 1MB
        assert_eq!(rust_cache_cleaner_lib::format_size(1048576), "1.00 MB"); // Exactly 1MB
        assert_eq!(rust_cache_cleaner_lib::format_size(1073741824), "1.00 GB"); // Exactly 1GB
        assert_eq!(
            rust_cache_cleaner_lib::format_size(1073741823),
            "1024.00 MB"
        ); // Just under 1GB
    }

    #[test]
    fn test_calculate_dir_size_empty_dir() {
        let temp_dir = TempDir::new().expect("Unable to create temporary directory");
        let size = rust_cache_cleaner_lib::utils::calculate_dir_size(temp_dir.path());
        assert_eq!(size, 0);
    }

    #[test]
    fn test_calculate_dir_size_with_files() {
        let temp_dir = TempDir::new().expect("Unable to create temporary directory");

        // Create a file in the temp directory
        let file_path = temp_dir.path().join("test_file.txt");
        fs::write(&file_path, "hello world").expect("Unable to write test file");

        let size = rust_cache_cleaner_lib::utils::calculate_dir_size(temp_dir.path());
        assert_eq!(size, 11); // "hello world" is 11 bytes
    }

    #[test]
    fn test_is_protected_path_basic() {
        // Test with a safe temp directory
        let temp_dir = TempDir::new().expect("Unable to create temporary directory");
        assert!(!rust_cache_cleaner_lib::utils::is_protected_path(
            temp_dir.path()
        ));
    }
}
