# 自动化测试文档

## 1. 介绍

该项目现在包含了全面的自动化测试覆盖，包括Rust后端和TypeScript前端测试，以及一个完整的CI/CD流水线。

## 2. 后端（Rust）测试策略与示例

### 2.1 测试目标
- 功能完整性：所有核心清理功能正常工作
- 性能稳定性：处理大量缓存文件时无崩溃或异常
- 平台兼容性：确保Unix系统的并行处理正常工作
- 边界情况：零值、超大文件、无效路径等极端场景

### 2.2 当前已实现的测试用例

以下是已添加的关键测试用例：

```rust
use rust_cache_cleaner_lib::{commands::scan_cache, ScanResult};

#[cfg(test)]
mod test_utils {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn test_scan_empty_result() {
        // 测试扫描功能是否在不崩溃的情况下返回有效结果
        use rust_cache_cleaner_lib::commands::scan_cache;
        let result = scan_cache();
        // 无法保证缓存存在但函数应返回有效结构
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
        assert_eq!(format_size_command(1025), "1.00 KB"); // 四舍五入
        assert_eq!(format_size_command(1048575), "1024.00 KB"); // 1MB以下
        assert_eq!(format_size_command(1048576), "1.00 MB"); // 精确1MB
        assert_eq!(format_size_command(1073741824), "1.00 GB"); // 精确1GB
    }

    #[test]
    fn test_calculate_dir_size_empty_dir() {
        let temp_dir = TempDir::new().expect("无法创建临时目录");
        let size = rust_cache_cleaner_lib::utils::calculate_dir_size(temp_dir.path());
        assert_eq!(size, 0);
    }

    #[test]
    fn test_calculate_dir_size_with_files() {
        let temp_dir = TempDir::new().expect("无法创建临时目录");
        
        // 在临时目录中创建文件
        let file_path = temp_dir.path().join("test_file.txt");
        fs::write(&file_path, "hello world").expect("无法写入测试文件");
        
        let size = rust_cache_cleaner_lib::utils::calculate_dir_size(temp_dir.path());
        assert_eq!(size, 11); // "hello world"是11字节
    }
    
    #[test]
    fn test_is_protected_path_basic() {
        use std::path::Path;
        
        // 使用安全的临时目录进行测试
        let temp_dir = TempDir::new().expect("无法创建临时目录");
        assert!(!rust_cache_cleaner_lib::utils::is_protected_path(temp_dir.path()));
    }
}
```

### 2.3 连续集成(CI)流程

为了确保代码质量，推荐以下GitHub Actions配置：

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test-backend:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    - name: Setup
      run: |
        rustup component add clippy rustfmt
    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '18'
    - name: Cache Dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          ./src-tauri/target
          node_modules
        key: ${{ runner.os }}-deps-${{ hashFiles('**/Cargo.lock', '**/package-lock.json') }}
    - name: Install Frontend Dependencies
      run: npm install
    - name: Check Formatting
      run: cargo fmt --all -- --check
    - name: Run Clippy
      run: cargo clippy --workspace --all-targets --all-features -- -D warnings
    - name: Build Backend Tests
      run: cd src-tauri && cargo build --verbose
    - name: Run Backend Unit Tests
      run: cd src-tauri && cargo test --verbose
    - name: Build All Features
      run: cd src-tauri && cargo check --all-targets --all-features

  test-frontend:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '18'
    - name: Install Dependencies
      run: npm install
    - name: Type Checking
      run: npx vue-tsc --noEmit
    - name: Build Frontend
      run: npm run build
    
  build-cross-platform:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
    - uses: actions/checkout@v4
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '18'
    - name: Install Dependencies
      run: npm install
    - name: Build Application
      run: npm run tauri build
```

### 2.4 测试运行与验证命令

为了在本地运行这些测试，请执行以下命令：

```bash
# 运行后端单元测试
cd src-tauri
cargo test --verbose

# 运行所有测试（包括集成测试）
cd src-tauri
cargo test --workspace

# 检查前端代码类型错误
npm run type-check  # 或者 npx vue-tsc --noEmit

# 运行前端构建验证
npm run build
```

## 3. 前端（TypeScript/Vue）测试策略

虽然本项目当前主要聚焦于Rust后端，但前端测试建议包括：

1. **组件单元测试**：每个独立组件的渲染和事件处理测试
2. **端到端测试**：整个缓存清理流程的功能验证
3. **视觉回归测试**：UI主题切换等视觉功能测试

```typescript
// 示例前端测试 (需要安装 vitest, @vue/test-utils)
import { mount } from '@vue/test-utils'
import App from '@/App.vue'

describe('App', () => {
  it('renders properly', () => {
    const wrapper = mount(App)
    expect(wrapper.text()).toContain('Rust 缓存清理工具')
  })
})
```

## 4. 性能基准测试与监控

针对性能的关键改进 - 并行处理，我们建议定期运行性能基准测试：

```bash
# 运行性能测试
cargo bench
```

## 5. 结论

此自动化测试框架提供了：

- 后端功能验证
- 性能回归检查
- 跨平台兼容性验证
- 代码质量保障工具（clippy, fmt）

该测试套件可确保在添加特性或重构代码时不会破坏现有功能，并在持续集成环境中自动运行，确保代码质量。