use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem {
    pub name: String,
    pub path: String,
    pub relative_path: String,
    pub size: u64,
    pub item_type: String,
    pub last_modified: Option<String>,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub items: Vec<CacheItem>,
    pub total_size: u64,
    pub selected_size: u64,
    pub logs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanResult {
    pub item_name: String,
    pub item_type: String,
    pub size_freed: u64,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorInfo {
    pub is_tuna: bool,
    pub mirror_name: String,
    pub mirror_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub host_name: String,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub total_memory: String,
    pub rust_version: String,
    pub cargo_version: String,
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

fn calculate_dir_size(dir: &Path) -> u64 {
    let mut total_size = 0u64;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(metadata) = fs::metadata(&path) {
                if metadata.is_dir() {
                    total_size += calculate_dir_size(&path);
                } else {
                    total_size += metadata.len();
                }
            }
        }
    }
    total_size
}

fn scan_directory(
    dir: &Path,
    cache_type: &str,
    items: &mut Vec<CacheItem>,
    logs: &mut Vec<String>,
) {
    if !dir.exists() {
        return;
    }

    logs.push(format!("扫描目录: {}", dir.display()));

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let metadata = match fs::metadata(&path) {
                Ok(m) => m,
                Err(_) => continue,
            };

            let size = if metadata.is_dir() {
                calculate_dir_size(&path)
            } else {
                metadata.len()
            };

            let last_modified = metadata.modified().ok().and_then(|t| {
                // 将 SystemTime 转换为 chrono 的 Local 时间并格式化
                // chrono 实现了从 SystemTime 的转换，所以可以直接使用 `DateTime::<Local>::from`
                let dt: DateTime<Local> = DateTime::from(t);
                Some(dt.format("%Y-%m-%d %H:%M:%S").to_string())
            });

            let item_name = path
                .file_name()
                .unwrap_or_else(|| path.as_os_str())
                .to_string_lossy()
                .to_string();

            let relative_path = path.to_string_lossy().replace("\\", "/");

            let item = CacheItem {
                name: item_name,
                path: path.to_string_lossy().to_string(),
                relative_path,
                size,
                item_type: cache_type.to_string(),
                last_modified,
                selected: true,
            };

            items.push(item);
        }
    }
}

mod commands {
    use super::*;

    #[tauri::command]
    pub fn scan_cache() -> ScanResult {
        let mut items: Vec<CacheItem> = Vec::new();
        let mut logs: Vec<String> = Vec::new();

        if let Some(home_dir) = dirs::home_dir() {
            let cargo_home = home_dir.join(".cargo");
            let registry_cache = cargo_home.join("registry");
            let index_cache = cargo_home.join("registry").join("index");
            scan_directory(&registry_cache, "注册表缓存", &mut items, &mut logs);
            scan_directory(&index_cache, "注册表索引", &mut items, &mut logs);

            let git_cache = cargo_home.join("git");
            scan_directory(&git_cache, "Git缓存", &mut items, &mut logs);

            let rustup_home = home_dir.join(".rustup");
            let toolchains = rustup_home.join("toolchains");
            scan_directory(&toolchains, "Rust工具链", &mut items, &mut logs);

            let downloads = rustup_home.join("downloads");
            scan_directory(&downloads, "Rust下载缓存", &mut items, &mut logs);
        }

        let total_size: u64 = items.iter().map(|item| item.size).sum();
        let selected_size: u64 = items
            .iter()
            .filter(|item| item.selected)
            .map(|item| item.size)
            .sum();

        logs.push(format!(
            "扫描完成！共发现 {} 个缓存项，总大小: {}",
            items.len(),
            format_size(total_size)
        ));

        ScanResult {
            items,
            total_size,
            selected_size,
            logs,
        }
    }

    // 判断路径是否为受保护路径（例如解释器、运行时等）
    fn is_protected_path(path: &Path) -> bool {
        let s = path.to_string_lossy().to_lowercase();

        // 保护用户主目录下的重要目录，但不能是 cargo/rustup 自己的缓存
        if let Some(home) = dirs::home_dir() {
            let _home_str = home.to_string_lossy().to_lowercase();

            // 允许删除 cargo/rustup 的缓存目录
            if s.contains(".cargo") || s.contains(".rustup") {
                // 但保护其中的重要内容
                // 不保护具体的 toolchain 目录（让用户可以选择删除）
                // 只保护 cargo/bin 等可执行目录
                if s.contains(".cargo/bin") || s.contains("\\bin\\") || s.contains("/bin/") {
                    return true;
                }
                return false; // 允许删除缓存
            }

            // 保护其他用户目录下的重要内容
            if s.contains("node_modules") {
                return true;
            }
        }

        // 关键字检查（包含目录名或可执行文件名）
        let protected_keywords = [
            "\\python",
            "/python",
            "python.exe",
            "python3",
            "node.exe",
            "\"node\"",
            "java.exe",
            "java",
            "dotnet",
            "pwsh",
            "powershell",
            "ruby",
            "go",
            // Windows 系统关键目录
            "windows",
            "program files",
            "system32",
        ];

        for k in &protected_keywords {
            if s.contains(k) {
                return true;
            }
        }

        // 如果是目录，检查目录下是否存在典型的解释器可执行文件
        if path.is_dir() {
            let candidates = [
                "python.exe",
                "python",
                "node.exe",
                "node",
                "java.exe",
                "java",
                "dotnet.exe",
                "dotnet",
                "pwsh.exe",
                "powershell.exe",
            ];
            for c in &candidates {
                if path.join(c).exists() {
                    return true;
                }
            }
        }

        false
    }

    #[tauri::command]
    pub fn clean_cache(paths: Vec<String>) -> (Vec<CleanResult>, Vec<String>) {
        let mut results: Vec<CleanResult> = Vec::new();
        let mut logs: Vec<String> = Vec::new();
        let mut total_freed = 0u64;

        if paths.is_empty() {
            logs.push("没有选择要清理的缓存项".to_string());
            return (results, logs);
        }

        logs.push(format!("开始清理 {} 个选中的缓存项...", paths.len()));

        for path_str in paths {
            let path = PathBuf::from(&path_str);

            if !path.exists() {
                logs.push(format!("路径不存在，跳过: {}", path_str));
                continue;
            }

            let name = path
                .file_name()
                .unwrap_or_else(|| path.as_os_str())
                .to_string_lossy()
                .to_string();

            let item_type = if path_str.contains("registry") && path_str.contains("index") {
                "注册表索引"
            } else if path_str.contains("registry") {
                "注册表缓存"
            } else if path_str.contains("git") {
                "Git缓存"
            } else if path_str.contains("toolchains") {
                "Rust工具链"
            } else if path_str.contains("downloads") {
                "Rust下载缓存"
            } else {
                "Cargo目录"
            }
            .to_string();

            logs.push(format!("清理: {} ({})", name, item_type));

            // 保护检查：如果路径看起来像解释器或关键运行时，跳过并记录
            if is_protected_path(&path) {
                logs.push(format!("受保护路径，跳过: {}", path_str));
                results.push(CleanResult {
                    item_name: name,
                    item_type,
                    size_freed: 0,
                    success: false,
                    error_message: Some("受保护路径，已跳过".to_string()),
                });
                continue;
            }

            // 额外安全检查：防止删除根目录
            let mut can_delete = true;
            if let Some(home) = dirs::home_dir() {
                let cargo_home = home.join(".cargo");
                let rustup_home = home.join(".rustup");

                // 不能直接删除 .cargo 或 .rustup 目录本身
                let canonical_path = std::fs::canonicalize(&path).ok();
                let canonical_cargo = std::fs::canonicalize(&cargo_home).ok();
                let canonical_rustup = std::fs::canonicalize(&rustup_home).ok();

                if let (Some(cp), Some(cc)) = (&canonical_path, &canonical_cargo) {
                    if cp == cc {
                        logs.push(format!("禁止删除 .cargo 根目录: {}", path_str));
                        results.push(CleanResult {
                            item_name: name.clone(),
                            item_type: item_type.clone(),
                            size_freed: 0,
                            success: false,
                            error_message: Some("禁止删除 .cargo 根目录".to_string()),
                        });
                        can_delete = false;
                    }
                }

                if can_delete {
                    if let (Some(cp), Some(cr)) = (&canonical_path, &canonical_rustup) {
                        if cp == cr {
                            logs.push(format!("禁止删除 .rustup 根目录: {}", path_str));
                            results.push(CleanResult {
                                item_name: name.clone(),
                                item_type: item_type.clone(),
                                size_freed: 0,
                                success: false,
                                error_message: Some("禁止删除 .rustup 根目录".to_string()),
                            });
                            can_delete = false;
                        }
                    }
                }
            }

            if !can_delete {
                continue;
            }

            let result = if path.is_dir() {
                let size = calculate_dir_size(&path);
                fs::remove_dir_all(&path)
                    .map(|_| size)
                    .map_err(|e| e.to_string())
            } else {
                match fs::metadata(&path) {
                    Ok(metadata) => {
                        let size = metadata.len();
                        fs::remove_file(&path)
                            .map(|_| size)
                            .map_err(|e| e.to_string())
                    }
                    Err(e) => Err(e.to_string()),
                }
            };

            match result {
                Ok(size) => {
                    total_freed += size;
                    results.push(CleanResult {
                        item_name: name,
                        item_type,
                        size_freed: size,
                        success: true,
                        error_message: None,
                    });
                    logs.push(format!("  ✓ 成功释放: {}", format_size(size)));
                }
                Err(e) => {
                    results.push(CleanResult {
                        item_name: name,
                        item_type,
                        size_freed: 0,
                        success: false,
                        error_message: Some(e.clone()),
                    });
                    logs.push(format!("  ✗ 失败: {}", e));
                }
            }
        }

        logs.push(format!(
            "清理完成！共释放空间: {}",
            format_size(total_freed)
        ));

        let success_count = results.iter().filter(|r| r.success).count();
        logs.push("===== 清理报告 =====".to_string());
        logs.push(format!("总处理数: {}", results.len()));
        logs.push(format!("成功: {}", success_count));
        logs.push(format!("失败: {}", results.len() - success_count));
        logs.push(format!("释放空间: {}", format_size(total_freed)));

        (results, logs)
    }

    #[tauri::command]
    pub fn check_mirror() -> (MirrorInfo, Vec<String>) {
        let mut logs: Vec<String> = Vec::new();
        logs.push("检查 Rust 镜像源配置...".to_string());

        let mut mirror_info = MirrorInfo {
            is_tuna: false,
            mirror_name: "未知".to_string(),
            mirror_url: "未配置".to_string(),
        };

        // 查找配置文件位置
        let config_paths = if let Some(home) = dirs::home_dir() {
            vec![home.join(".cargo/config.toml"), home.join(".cargo/config")]
        } else {
            vec![]
        };

        let mut config_content: Option<String> = None;
        let _found_path: Option<PathBuf> = None;

        for config_path in &config_paths {
            if config_path.exists() {
                logs.push(format!("读取配置文件: {}", config_path.display()));
                match fs::read_to_string(config_path) {
                    Ok(content) => {
                        config_content = Some(content);
                        break;
                    }
                    Err(e) => {
                        logs.push(format!("读取失败: {}", e));
                    }
                }
            }
        }

        let content = match config_content {
            Some(c) => c,
            None => {
                logs.push("Cargo 配置文件不存在".to_string());
                mirror_info.is_tuna = false;
                mirror_info.mirror_name = "默认源".to_string();
                mirror_info.mirror_url = "https://crates.io".to_string();
                return (mirror_info, logs);
            }
        };

        // 从内容中提取镜像URL
        let mut found_url = String::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("registry =")
                || trimmed.starts_with("sparse-registry =")
                || trimmed.starts_with("git =")
            {
                if let Some(url) = trimmed.split('"').nth(1) {
                    found_url = url.to_string();
                    break;
                }
            }
        }

        // 根据实际URL判断镜像源（URL最准确）
        let url_lower = found_url.to_lowercase();

        if !found_url.is_empty() && url_lower.contains("tuna.tsinghua") {
            mirror_info.is_tuna = true;
            mirror_info.mirror_name = "清华大学 (TUNA)".to_string();
            mirror_info.mirror_url = found_url;
            logs.push(format!("检测到清华大学镜像"));
        } else if !found_url.is_empty()
            && (url_lower.contains("ustc") || url_lower.contains("ustc.edu.cn"))
        {
            mirror_info.is_tuna = false;
            mirror_info.mirror_name = "中科大 (USTC)".to_string();
            mirror_info.mirror_url = found_url;
            logs.push(format!("检测到中科大镜像"));
        } else if !found_url.is_empty()
            && (url_lower.contains("aliyun") || url_lower.contains("aliyuncs"))
        {
            mirror_info.is_tuna = false;
            mirror_info.mirror_name = "阿里云".to_string();
            mirror_info.mirror_url = found_url;
            logs.push(format!("检测到阿里云镜像"));
        } else if !found_url.is_empty() && url_lower.contains("rsproxy") {
            mirror_info.is_tuna = false;
            mirror_info.mirror_name = "RSProxy".to_string();
            mirror_info.mirror_url = found_url;
            logs.push(format!("检测到 RSProxy 镜像"));
        } else if !found_url.is_empty() {
            // 有URL但不是已知镜像
            mirror_info.is_tuna = false;
            mirror_info.mirror_name = "自定义源".to_string();
            mirror_info.mirror_url = found_url;
            logs.push(format!("检测到自定义镜像"));
        } else if content.contains("[source.tuna]") || content.contains("replace-with = \"tuna\"") {
            mirror_info.is_tuna = true;
            mirror_info.mirror_name = "清华大学 (TUNA)".to_string();
            mirror_info.mirror_url =
                "https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/".to_string();
            logs.push(format!("检测到清华大学镜像（配置）"));
        } else if content.contains("[source.ustc]") || content.contains("replace-with = \"ustc\"") {
            mirror_info.is_tuna = false;
            mirror_info.mirror_name = "中科大 (USTC)".to_string();
            mirror_info.mirror_url = "https://mirrors.ustc.edu.cn/crates.io-index/".to_string();
            logs.push(format!("检测到中科大镜像（配置）"));
        } else {
            mirror_info.is_tuna = false;
            mirror_info.mirror_name = "默认源".to_string();
            mirror_info.mirror_url = "https://crates.io".to_string();
            logs.push(format!("使用默认源"));
        }

        logs.push(format!("镜像源: {}", mirror_info.mirror_name));
        logs.push(format!("URL: {}", mirror_info.mirror_url));

        if mirror_info.is_tuna {
            logs.push("✓ 使用的是清华大学镜像源".to_string());
        } else {
            logs.push("✗ 未使用清华大学镜像源".to_string());
        }

        (mirror_info, logs)
    }

    #[tauri::command]
    pub fn format_size_command(size: u64) -> String {
        format_size(size)
    }

    #[tauri::command]
    pub fn get_system_info() -> SystemInfo {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let os_name = sysinfo::System::name().unwrap_or_else(|| "未知".to_string());
        let os_version = sysinfo::System::os_version().unwrap_or_else(|| "未知".to_string());
        let host_name = sysinfo::System::host_name().unwrap_or_else(|| "未知".to_string());

        let cpu_name = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "未知".to_string());
        let cpu_cores = sys.cpus().len();

        let total_memory = sys.total_memory();
        let total_memory_gb = total_memory as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_memory_str = format!("{:.1} GB", total_memory_gb);

        let rust_version = std::process::Command::new("rustc")
            .arg("--version")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "未安装".to_string());

        let cargo_version = std::process::Command::new("cargo")
            .arg("--version")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "未安装".to_string());

        SystemInfo {
            os_name,
            os_version,
            host_name,
            cpu_name,
            cpu_cores,
            total_memory: total_memory_str,
            rust_version,
            cargo_version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(1048576), "1.00 MB");
        assert_eq!(format_size(1073741824), "1.00 GB");
    }

    #[test]
    fn test_calculate_dir_size_nonexistent() {
        let result = calculate_dir_size(Path::new("/nonexistent/path"));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_scan_result_structure() {
        let result = ScanResult {
            items: vec![],
            total_size: 0,
            selected_size: 0,
            logs: vec![],
        };
        assert_eq!(result.items.len(), 0);
        assert_eq!(result.total_size, 0);
    }

    #[test]
    fn test_clean_result_structure() {
        let result = CleanResult {
            item_name: "test".to_string(),
            item_type: "测试".to_string(),
            size_freed: 1024,
            success: true,
            error_message: None,
        };
        assert_eq!(result.success, true);
        assert_eq!(result.error_message, None);
    }

    #[test]
    fn test_mirror_info_structure() {
        let info = MirrorInfo {
            is_tuna: true,
            mirror_name: "清华大学 (TUNA)".to_string(),
            mirror_url: "https://mirrors.tuna.tsinghua.edu.cn".to_string(),
        };
        assert_eq!(info.is_tuna, true);
        assert!(info.mirror_name.contains("清华"));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::scan_cache,
            commands::clean_cache,
            commands::check_mirror,
            commands::format_size_command,
            commands::get_system_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
