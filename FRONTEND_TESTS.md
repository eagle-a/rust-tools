# 前端自动化测试说明

## 设置前端测试环境

1. 安装测试依赖：
```bash
npm install -D vitest @vue/test-utils jsdom happy-dom @vitejs/plugin-vue
```

2. 配置 vitest：
```js
// vite.config.ts
/// <reference types="vitest" />
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  test: {
    environment: 'jsdom',
    transformMode: {
      web: [/\.[jt]sx$/],
    },
  },
  
  // 忽略 defineConfig 错误
  resolve: {
    alias: {
      '@': __dirname + '/src',
    },
  },
})
```

3. 在 package.json 中添加测试脚本：
```js
{
  "scripts": {
    "test:unit": "vitest",
    "test:unit:ui": "vitest --ui",
    "test:unit:run": "vitest run"
  }
}
```

## 现有的测试内容

前端的测试主要包括：

1. **组件渲染测试**：
   - 验证各组件是否能正确渲染
   - 检查初始状态是否符合预期

2. **事件交互测试**：
   - 验证用户交互是否触发正确的响应
   - 检查状态变换是否正常

3. **API 调用测试**：
   - 验证与后端 Rust 的交互是否正常
   - 测试各种状态和错误处理

## 建议的前端测试用例

1. App.vue 组件测试
2. CacheList 组件测试
3. 扫描缓存功能测试
4. 清理缓存功能测试
5. 镜像源检测功能测试
6. 主题切换功能测试

## 运行测试

- `npm run test:unit`: 运行所有单元测试
- `npm run test:unit:run`: 一次性运行测试（不需要监听模式）
- `npm run test`: 同时运行前端和后端的测试

## 重要提醒

为了完整地运行前端测试，你需要安装额外的测试依赖。本项目主要后端已包含完整的单元测试，
前端测试是一个扩展选项，可以根据需求选择性启用。