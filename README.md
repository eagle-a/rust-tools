# Rust 缓存清理工具 - 使用教程

## 项目简介

这是一款 Rust 缓存清理工具，可以扫描并清理 Rust 开发环境中的各种缓存文件，包括：
- 注册表缓存
- Git 缓存
- 项目 Target 目录
- Cargo 目录
- Rustup 目录
- 镜像源检测（清华源）

## 环境要求

1. **Node.js** (推荐 v18+)
2. **Rust** (推荐 v1.70+)
3. **Windows 10/11**

## 快速开始

### 1. 安装依赖

```bash
cd rust-cache-cleaner
npm install
```

### 2. 开发模式运行

```bash
npm run tauri dev
```

这会启动开发服务器，实时热加载代码。

### 3. 生产打包

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
   - 勾选要清理的缓存项
   - 点击"清理选中"删除选中的缓存

2. **镜像源**
   - 自动检测当前使用的 Cargo 镜像源
   - 显示是否为清华源

3. **日志**
   - 显示所有操作日志
   - 可手动清除日志

### 主题模式

- **自动切换**：根据时间自动切换主题
  - 白天模式（8:00-18:00）：白色主题
  - 夜晚模式（18:00-次日8:00）：暗色主题
- **手动切换**：点击右上角"🌙 黑暗"或"☀️ 亮色"按钮切换

## 常见问题

### Q: 扫描不到缓存？
A: 确保已安装 Rust 工具链，运行 `rustup show` 检查。

### Q: 清理失败？
A: 有些缓存可能被程序占用，关闭相关程序后重试。

### Q: 需要管理员权限吗？
A: 大多数缓存清理不需要管理员权限，但某些系统目录可能需要。

## 项目结构

```
rust-cache-cleaner/
├── src/                    # Vue 前端代码
│   ├── App.vue            # 主应用组件
│   └── main.ts           # 入口文件
├── src-tauri/             # Tauri 后端代码
│   ├── src/
│   │   ├── lib.rs        # Rust 核心逻辑
│   │   └── main.rs       # 程序入口
│   ├── tauri.conf.json   # Tauri 配置
│   └── Cargo.toml        # Rust 依赖
├── package.json           # Node 依赖
└── vite.config.ts         # Vite 配置
```

## 技术栈

- **前端**：Vue 3 + TypeScript + Vite
- **后端**：Rust + Tauri 2.0
- **UI**：原生 CSS（支持亮色/暗色主题）

## 许可证

MIT License
