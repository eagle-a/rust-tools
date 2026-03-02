#[cfg(test)]
mod test_utils {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn test_scan_empty_result() {
        // Test that the scan function runs without panicking
        use rust_cache_cleaner_lib::commands::scan_cache;
        let result = scan_cache();
        // We can't guarantee cache existence but function should return valid struct
        assert!(result.items.len() >= 0);
        assert!(result.total_size >= 0);
        assert!(result.selected_size >= 0);
    }

    #[test]
    fn test_format_size_boundaries() {
        use rust_cache_cleaner_lib::commands::format_size_command;
        
        assert_eq!(format_size_command(0), "0 B");
        assert_eq!(format_size_command(1023), "1023 B");
        assert_eq!(format_size_command(1024), "1.00 KB");
        assert_eq!(format_size_command(1025), "1.00 KB"); // Rounded
        assert_eq!(format_size_command(1048575), "1024.00 KB"); // Just under 1MB
        assert_eq!(format_size_command(1048576), "1.00 MB"); // Exactly 1MB
        assert_eq!(format_size_command(1073741824), "1.00 GB"); // Exactly 1GB
    }

    #[test]
    fn test_cache_structures() {
        use rust_cache_cleaner_lib::{ScanResult, CacheItem};
        
        // Test that structures are created properly
        let empty_items = vec![];
        let result = ScanResult {
            items: empty_items,
            total_size: 0,
            selected_size: 0,
            logs: vec!["Test log".to_string()],
        };
        assert_eq!(result.total_size, 0);
        assert_eq!(result.selected_size, 0);
        assert_eq!(result.logs.len(), 1);
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
        use std::path::Path;
        
        // Test with a safe temp directory
        let temp_dir = TempDir::new().expect("Unable to create temporary directory");
        assert!(!rust_cache_cleaner_lib::utils::is_protected_path(temp_dir.path()));
    }

    // Performance benchmarking test
    #[test]
    fn benchmark_file_operations() {
        use std::time::Instant;
        let start = Instant::now();
        
        let temp_dir = TempDir::new().expect("Unable to create temporary directory");
        let sample_data = "test".repeat(1000); // Create a moderately sized string
        
        let file_path = temp_dir.path().join("benchmark_file.txt");
        fs::write(&file_path, &sample_data).expect("Unable to write benchmark file");
        
        let contents = fs::read(&file_path).expect("Unable to read benchmark file");
        let elapsed = start.elapsed();
        
        println!("Benchmark file operation took: {:?}", elapsed);
        assert_eq!(contents.len(), sample_data.len());
    }
    
    // Test for the format utility function
    #[test]
    fn test_format_size_function() {
        // Access the format_size function from the outer module
        use rust_cache_cleaner_lib::lib::format_size;
        
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(1048576), "1.00 MB");
        assert_eq!(format_size(1073741824), "1.00 GB");
    }
}

    #[test]
    fn test_format_size_boundaries() {
        use rust_cache_cleaner_lib::commands::format_size_command;

        assert_eq!(format_size_command(0), "0 B");
        assert_eq!(format_size_command(1023), "1023 B");
        assert_eq!(format_size_command(1024), "1.00 KB");
        assert_eq!(format_size_command(1025), "1.00 KB"); // Rounded
        assert_eq!(format_size_command(1048575), "1024.00 KB"); // Just under 1MB
        assert_eq!(format_size_command(1048576), "1.00 MB"); // Exactly 1MB
        assert_eq!(format_size_command(1073741824), "1.00 GB"); // Exactly 1GB
    }

    #[test]
    fn test_cache_structures() {
        // Test that structures are created properly
        let empty_items = vec![];
        let result = ScanResult {
            items: empty_items,
            total_size: 0,
            selected_size: 0,
            logs: vec!["Test log".to_string()],
        };
        assert_eq!(result.total_size, 0);
        assert_eq!(result.selected_size, 0);
        assert_eq!(result.logs.len(), 1);
    }

    #[test]
    fn test_calculate_dir_size_empty_dir() {
        use rust_cache_cleaner_lib::utils::calculate_dir_size;
        let temp_dir = TempDir::new().expect("Unable to create temporary directory");
        let size = calculate_dir_size(temp_dir.path());
        assert_eq!(size, 0);
    }

    #[test]
    fn test_calculate_dir_size_with_files() {
        use rust_cache_cleaner_lib::utils::calculate_dir_size;
        let temp_dir = TempDir::new().expect("Unable to create temporary directory");

        // Create a file in the temp directory
        let file_path = temp_dir.path().join("test_file.txt");
        fs::write(&file_path, "hello world").expect("Unable to write test file");

        let size = calculate_dir_size(temp_dir.path());
        assert_eq!(size, 11); // "hello world" is 11 bytes
    }

    #[test]
    fn test_is_protected_path_basic() {
        use rust_cache_cleaner_lib::utils::is_protected_path;
        use std::path::Path;

        // Test with a safe temp directory
        let temp_dir = TempDir::new().expect("Unable to create temporary directory");
        assert!(!is_protected_path(temp_dir.path()));
    }

    // Performance benchmarking test
    #[test]
    fn benchmark_file_operations() {
        use std::fs;
        use std::time::Instant;
        let start = Instant::now();

        let temp_dir = TempDir::new().expect("Unable to create temporary directory");
        let sample_data = "test".repeat(1000); // Create a moderately sized string

        let file_path = temp_dir.path().join("benchmark_file.txt");
        fs::write(&file_path, &sample_data).expect("Unable to write benchmark file");

        let contents = fs::read(&file_path).expect("Unable to read benchmark file");
        let elapsed = start.elapsed();

        println!("Benchmark file operation took: {:?}", elapsed);
        assert_eq!(contents.len(), sample_data.len());
    }

    // Test for the format utility function
    #[test]
    fn test_format_size_function() {
        use rust_cache_cleaner_lib::lib::format_size;

        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(1048576), "1.00 MB");
        assert_eq!(format_size(1073741824), "1.00 GB");
    }
}
