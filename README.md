# Rust 缓存清理工具

一款基于 Tauri + Vue 3 开发的高性能 Rust 开发环境缓存清理工具，支持跨平台，具备现代化 UI 和完整的安全保护机制。

## 项目简介

Rust 缓存清理工具是一款用于管理和清理 Rust 开发环境缓存的桌面应用程序，可以智能扫描并安全清理各类缓存文件，帮助开发者快速释放磁盘空间。

### 核心功能

- **智能扫描**：自动扫描 Rust 开发环境中的各类缓存文件
  - 注册表缓存（Registry）
  - Git 依赖缓存
  - Rust 工具链（Toolchains）
  - 下载缓存（Downloads）
- **多源支持**：支持识别多种镜像源配置（清华源、中科大源、RSProxy、阿里云等）
- **性能优化**：使用 `walkdir` 优化目录遍历，Unix 平台支持并行处理
- **安全保障**：多重安全过滤机制，防止误删系统关键文件
- **主题适配**：支持亮暗色主题自动/手动切换
- **跨平台**：支持 Windows、macOS 和 Linux

## 技术架构

### 技术栈 aaaaaaaaaaaaaaaa

- **前端**：Vue 3 + TypeScript + Vite
- **后端**：Rust + Tauri 2.0
- **构建工具**：npm + Cargo
- **UI**：原生 CSS（支持亮色/暗色主题）

### 项目结构

```
rust-cache-cleaner/
├── src/                      # Vue 前端源码
│   ├── components/          # 组件目录（已拆分）
│   │   ├── CacheList.vue   # 缓存列表组件
│   │   ├── MirrorPanel.vue # 镜像面板组件
│   │   ├── SystemPanel.vue # 系统信息组件
│   │   ├── LogsPanel.vue   # 日志面板组件
│   │   └── ConfirmDialog.vue # 确认对话框组件
│   ├── types/              # 类型定义
│   ├── App.vue             # 主应用组件
│   └── main.ts             # 入口文件
├── src-tauri/              # Rust 后端源码
│   ├── src/
│   │   ├── lib.rs          # 核心业务逻辑
│   │   └── main.rs         # 程序入口
│   ├── tests/              # 集成测试
│   ├── Cargo.toml          # Rust 依赖配置
│   └── tauri.conf.json     # Tauri 配置
├── package.json            # npm 依赖配置
└── vite.config.ts          # Vite 配置
```

## 环境要求

1. **Node.js** (推荐 v18+)
2. **Rust** (推荐 v1.70+)
3. **Windows 10/11 / macOS / Linux**

## 快速开始

### 安装依赖

```bash
cd rust-cache-cleaner
npm install
```

### 开发模式运行

```bash
npm run tauri dev
```

这会启动开发服务器，实时热加载代码。

### 生产打包

```bash
npm run tauri build
```

打包完成后，可执行文件位于：
```
src-tauri/target/release/rust-cache-cleaner.exe
```

## 使用说明

### 启动应用

直接运行 `rust-cache-cleaner.exe` 即可。

### 主界面功能

1. **缓存列表**
   - 点击"扫描缓存"按钮扫描系统中的 Rust 缓存
   - 勾选要清理的缓存项（支持全选/取消全选）
   - 支持按类型筛选和搜索
   - 支持排序（按名称、大小、日期、类型）
   - 点击"清理选中"删除选中的缓存

2. **镜像源**
   - 自动检测当前使用的 Cargo 镜像源
   - 显示镜像源名称和 URL
   - 支持重新检查

3. **系统信息**
   - 显示操作系统、CPU、内存等硬件信息
   - 显示 Rust 和 Cargo 版本

4. **日志**
   - 显示所有操作日志
   - 支持清除日志
   - 自动滚动到底部

### 主题模式

- **自动切换**：根据时间自动切换主题
  - 白天模式（8:00-18:00）：白色主题
  - 夜晚模式（18:00-次日8:00）：暗色主题
- **手动切换**：点击右上角"🌙 黑暗"或"☀️ 亮色"按钮切换

## 技术实现

### 前端实现

#### 状态管理

```typescript
// 核心响应式状态
const activeTab = ref<'cache' | 'mirror' | 'logs' | 'system'>('cache')
const isDark = ref(true)                    // 主题模式
const cacheItems = ref<CacheItem[]>([])     // 缓存列表
const logs = ref<string[]>([])              // 日志
const isScanning = ref(false)               // 扫描中状态
const isCleaning = ref(false)               // 清理中状态
```

#### 核心函数

1. **initTheme()** - 主题初始化
   - 根据当前时间自动设置主题
   - 应用 CSS 变量切换

2. **scanCache()** - 扫描缓存
   - 调用 Rust 后端 `scan_cache` 命令
   - 异步获取缓存项列表
   - 更新缓存数量和大小统计

3. **cleanCache()** - 清理缓存
   - 获取选中的缓存项
   - 调用 Rust 后端 `clean_cache` 命令
   - 清理完成后自动重新扫描

### 后端实现

#### 核心数据结构

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem {
    pub name: String,           // 缓存名称
    pub path: String,          // 完整路径
    pub relative_path: String,  // 相对路径（用于UI显示）
    pub size: u64,             // 大小（字节）
    pub item_type: String,      // 类型（注册表/Git/工具链等）
    pub last_modified: Option<String>,  // 最后修改时间
    pub selected: bool,         // 是否选中
}
```

#### 性能优化

1. **目录遍历优化**
   - 使用 `walkdir` 库替代手动递归
   - 更高效地处理大型目录

2. **并行处理**
   - Unix 平台使用 `rayon` 实现并行计算
   - 显著提升大目录扫描速度

3. **内存优化**
   - 使用 `Lazy` 静态常量避免重复分配
   - 关键字数组只在首次访问时初始化

#### 安全机制

1. **路径保护**
   - `is_protected_path()` 函数检查关键路径
   - 防止删除系统目录（Windows、Program Files 等）
   - 防止删除解释器运行时（Python、Node.js、Java 等）

2. **根目录保护**
   - 禁止直接删除 `.cargo` 和 `.rustup` 根目录
   - 只允许删除其子目录中的缓存

3. **白名单检查**
   - 仅允许删除已知缓存目录下的子路径
   - 对非缓存目录需要更严格确认

#### 主要命令

1. **scan_cache** - 扫描缓存
   - 扫描 `~/.cargo/registry` - 注册表缓存
   - 扫描 `~/.cargo/git` - Git 依赖缓存
   - 扫描 `~/.rustup/toolchains` - 工具链
   - 扫描 `~/.rustup/downloads` - 下载缓存
   - 返回 `ScanResult` 包含所有缓存项和统计信息

2. **clean_cache** - 清理缓存
   - 接收要清理的路径列表
   - 检查路径是否存在和安全性
   - 计算目录大小
   - 安全删除文件或目录
   - 返回清理结果和详细日志

3. **check_mirror** - 检测镜像源
   - 读取 `~/.cargo/config.toml`
   - 识别镜像源类型（清华/中科大/RSProxy/阿里云/默认）
   - 返回镜像配置信息

4. **get_system_info** - 获取系统信息
   - 使用 `sysinfo` 库获取硬件信息
   - 调用 `rustc` 和 `cargo` 命令获取版本
   - 返回完整的系统和环境信息

## 自动化测试

### 测试架构

项目包含完整的自动化测试体系：

1. **Rust 单元测试** - 位于 `src-tauri/src/lib.rs`
2. **集成测试** - 位于 `src-tauri/tests/`
3. **前端类型检查** - 使用 `vue-tsc`

### 运行测试

```bash
# 运行后端单元测试
cd src-tauri
cargo test

# 运行前端类型检查
npx vue-tsc --noEmit

# 运行所有测试
npm run test
```

### 测试覆盖

- **format_size** - 测试字节格式化函数的各种边界条件
- **calculate_dir_size** - 测试目录大小计算，包括空目录和包含文件的目录
- **is_protected_path** - 测试路径保护机制
- **scan_cache** - 测试扫描功能的基本行为

### CI/CD 配置

推荐使用 GitHub Actions 进行持续集成：

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
    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '18'
    - name: Run Backend Tests
      run: cd src-tauri && cargo test --verbose

  test-frontend:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - name: Type Checking
      run: npx vue-tsc --noEmit

  build-cross-platform:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
    - uses: actions/checkout@v4
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    - name: Build Application
      run: npm run tauri build
```

## 优化历史

### 后端优化

1. ✅ 使用 `walkdir` 替代手动递归目录遍历
2. ✅ 添加 `rayon` 支持 Unix 平台的并行处理
3. ✅ 使用 `Lazy` 静态常量优化关键字数组
4. ✅ 创建 `utils` 模块便于测试和维护

### 前端优化

1. ✅ 拆分大型 `App.vue` 为多个独立组件
2. ✅ 添加排序功能（名称、大小、日期、类型）
3. ✅ 添加搜索和筛选功能
4. ✅ 创建类型定义文件统一管理接口

## 常见问题

### Q: 扫描不到缓存？

A: 确保已安装 Rust 工具链，运行 `rustup show` 检查。

### Q: 清理失败？

A: 有些缓存可能被程序占用，关闭相关程序后重试。

### Q: 需要管理员权限吗？

A: 大多数缓存清理不需要管理员权限，但某些系统目录可能需要。

### Q: 清理后会重新下载吗？

A: 是的，清理缓存后需要重新下载依赖，首次编译会较慢。

## 安全注意事项

1. **默认选中风险**：扫描结果默认全部选中，建议清理前仔细确认
2. **路径检查**：虽然有安全机制，但仍建议谨慎操作
3. **备份建议**：重要项目建议先备份再清理

## 开发计划

### 已完成功能

- [x] 基础缓存扫描和清理
- [x] 镜像源检测
- [x] 主题切换
- [x] 日志系统
- [x] 组件拆分
- [x] 自动化测试
- [x] 性能优化

### 待实现功能

- [ ] 清理确认对话框
- [ ] 清理进度条
- [ ] 历史记录统计
- [ ] 定时自动清理
- [ ] 更多镜像源一键切换
- [ ] 导出清理报告
- [ ] MSI 安装包修复

## 贡献指南

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 许可证

MIT License

---

**项目链接**: [https://github.com/eagle-a/rust-tools](https://github.com/eagle-a/rust-tools)

*最后更新: 2026年3月*
