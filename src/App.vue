<script setup lang="ts">
import { ref, computed, watch, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface CacheItem {
  name: string;
  path: string;
  relative_path: string;
  size: number;
  item_type: string;
  last_modified: string | null;
  selected: boolean;
}

interface ScanResult {
  items: CacheItem[];
  total_size: number;
  selected_size: number;
  logs: string[];
}

interface CleanResult {
  item_name: string;
  item_type: string;
  size_freed: number;
  success: boolean;
  error_message: string | null;
}

interface MirrorInfo {
  is_tuna: boolean;
  mirror_name: string;
  mirror_url: string;
}

const activeTab = ref<'cache' | 'mirror' | 'logs' | 'system'>('cache');
const isDark = ref(true);
const cacheItems = ref<CacheItem[]>([]);
const logs = ref<string[]>([]);
const totalSize = ref(0);
const selectedSize = ref(0);
const filterType = ref<string>('');
const searchQuery = ref('');
const isScanning = ref(false);
const isCleaning = ref(false);
const mirrorInfo = ref<MirrorInfo>({
  is_tuna: false,
  mirror_name: '未知',
  mirror_url: '未配置'
});
const isCheckingMirror = ref(false);

interface SystemInfo {
  os_name: string;
  os_version: string;
  host_name: string;
  cpu_name: string;
  cpu_cores: number;
  total_memory: string;
  rust_version: string;
  cargo_version: string;
}

const systemInfo = ref<SystemInfo | null>(null);

const filteredItems = computed(() => {
  let items = cacheItems.value;
  if (filterType.value) {
    items = items.filter(item => item.item_type === filterType.value);
  }
  if (searchQuery.value) {
    items = items.filter(item => 
      item.name.toLowerCase().includes(searchQuery.value.toLowerCase())
    );
  }
  return items;
});

const cacheTypes = computed(() => {
  const types = new Set(cacheItems.value.map(item => item.item_type));
  return Array.from(types);
});

function formatSize(size: number): string {
  const KB = 1024;
  const MB = KB * 1024;
  const GB = MB * 1024;

  if (size >= GB) {
    return `${(size / GB).toFixed(2)} GB`;
  } else if (size >= MB) {
    return `${(size / MB).toFixed(2)} MB`;
  } else if (size >= KB) {
    return `${(size / KB).toFixed(2)} KB`;
  } else {
    return `${size} B`;
  }
}

async function scanCache() {
  isScanning.value = true;
  logs.value.push('开始扫描缓存...');
  try {
    const result = await invoke<ScanResult>('scan_cache');
    cacheItems.value = result.items;
    totalSize.value = result.total_size;
    selectedSize.value = result.selected_size;
    result.logs.forEach(log => logs.value.push(log));
  } catch (e) {
    logs.value.push(`扫描失败: ${e}`);
  }
  isScanning.value = false;
}

async function cleanCache() {
  const selectedItems = cacheItems.value.filter(item => item.selected);
  if (selectedItems.length === 0) {
    logs.value.push('请先选择要清理的缓存项');
    return;
  }

  isCleaning.value = true;
  const paths = selectedItems.map(item => item.path);
  logs.value.push(`开始清理 ${paths.length} 个缓存项...`);
  
  try {
    const [, cleanLogs] = await invoke<[CleanResult[], string[]]>('clean_cache', { paths });
    cleanLogs.forEach(log => logs.value.push(log));
    
    // 重新扫描 - 静默模式
    const result = await invoke<ScanResult>('scan_cache');
    cacheItems.value = result.items;
    totalSize.value = result.total_size;
    selectedSize.value = result.selected_size;
  } catch (e) {
    logs.value.push(`清理失败: ${e}`);
  } finally {
    isCleaning.value = false;
  }
}

async function checkMirror(showLog: boolean = true) {
  if (isCheckingMirror.value) return;
  
  isCheckingMirror.value = true;
  if (showLog && logs.value[logs.value.length - 1] !== '检查镜像源配置...') {
    logs.value.push('检查镜像源配置...');
  }
  try {
    const [info, checkLogs] = await invoke<[MirrorInfo, string[]]>('check_mirror');
    mirrorInfo.value = info;
    if (showLog) {
      checkLogs.forEach(log => logs.value.push(log));
    }
  } catch (e) {
    if (showLog) {
      logs.value.push(`检查失败: ${e}`);
    }
  } finally {
    isCheckingMirror.value = false;
  }
}

function selectAll() {
  cacheItems.value.forEach(item => item.selected = true);
  updateSelectedSize();
}

function selectNone() {
  cacheItems.value.forEach(item => item.selected = false);
  updateSelectedSize();
}

function updateSelectedSize() {
  selectedSize.value = cacheItems.value
    .filter(item => item.selected)
    .reduce((sum, item) => sum + item.size, 0);
}

function toggleItem(index: number) {
  if (index >= 0 && index < cacheItems.value.length) {
    cacheItems.value[index].selected = !cacheItems.value[index].selected;
    updateSelectedSize();
  }
}

function getTypeColor(typeName: string): string {
  const colors: Record<string, string> = {
    '注册表缓存': '#64c864',
    'Git缓存': '#6496ff',
    '项目Target': '#ffc864',
    'Cargo目录': '#ff9696',
    'Rustup目录': '#c896ff',
  };
  return colors[typeName] || '#888';
}

function clearLogs() {
  logs.value = [];
}

function toggleTheme() {
  isDark.value = !isDark.value;
  document.body.setAttribute('data-theme', isDark.value ? 'dark' : 'light');
}

function initTheme() {
  const now = new Date();
  const hour = now.getHours();
  // 早上8点到晚上6点（18点）是亮色模式，其他时间是暗色模式
  isDark.value = hour < 8 || hour >= 18;
  document.body.setAttribute('data-theme', isDark.value ? 'dark' : 'light');
}

async function loadSystemInfo() {
  try {
    const info = await invoke<SystemInfo>('get_system_info');
    systemInfo.value = info;
  } catch (e) {
    console.error('获取系统信息失败:', e);
  }
}

// 初始化 - 不打印日志
initTheme();
checkMirror(false);
scanCache();
loadSystemInfo();

// 监听日志变化，自动滚动到底部
watch(logs, async () => {
  await nextTick();
  const logContent = document.querySelector('.logs-content');
  if (logContent) {
    logContent.scrollTop = logContent.scrollHeight;
  }
}, { deep: true });
</script>

<template>
  <div class="app">
    <!-- 顶部导航 -->
    <header class="header">
      <div class="header-title">Rust 缓存清理工具</div>
      <nav class="header-nav">
        <button 
          :class="{ active: activeTab === 'cache' }" 
          @click="activeTab = 'cache'"
        >
          📦 缓存列表
        </button>
        <button 
          :class="{ active: activeTab === 'mirror' }" 
          @click="activeTab = 'mirror'"
        >
          🔗 镜像源
        </button>
        <button 
          :class="{ active: activeTab === 'system' }" 
          @click="activeTab = 'system'"
        >
          💻 系统
        </button>
        <button 
          :class="{ active: activeTab === 'logs' }" 
          @click="activeTab = 'logs'"
        >
          📋 日志
        </button>
        <button class="theme-toggle" @click="toggleTheme" :title="isDark ? '切换到亮色模式' : '切换到黑暗模式'">
          {{ isDark ? '🌙 黑暗' : '☀️ 亮色' }}
        </button>
      </nav>
    </header>

    <!-- 工具栏 -->
    <div class="toolbar">
      <div class="toolbar-left">
        <button class="btn btn-primary" @click="scanCache" :disabled="isScanning || isCleaning">
          {{ isScanning ? '🔄 扫描中...' : '🔍 扫描缓存' }}
        </button>
        <button class="btn btn-danger" @click="cleanCache" :disabled="isCleaning || isScanning || selectedSize === 0">
          {{ isCleaning ? '🧹 清理中...' : '🧹 清理选中' }}
        </button>
        <button class="btn" @click="scanCache" :disabled="isScanning || isCleaning">🔄 刷新</button>
        
        <span class="separator"></span>
        
        <button class="btn" @click="selectAll" :disabled="isScanning || isCleaning">✓ 全选</button>
        <button class="btn" @click="selectNone" :disabled="isScanning || isCleaning">✗ 取消全选</button>
        
        <span class="separator"></span>
        
        <select v-model="filterType" class="select">
          <option value="">全部类型</option>
          <option v-for="type in cacheTypes" :key="type" :value="type">{{ type }}</option>
        </select>
        
        <input 
          v-model="searchQuery" 
          type="text" 
          class="search-input" 
          placeholder="搜索缓存..."
        />
      </div>
      
      <div class="toolbar-right">
        <span class="stats">
          总缓存项: {{ cacheItems.length }} | 
          总大小: {{ formatSize(totalSize) }} | 
          已选中: <span class="selected">{{ formatSize(selectedSize) }}</span>
        </span>
      </div>
    </div>

    <!-- 主内容区 -->
    <main class="main-content">
      <!-- 缓存列表 -->
      <div v-if="activeTab === 'cache'" class="cache-list">
        <div class="list-header">
          <span class="col-check"></span>
          <span class="col-name">名称</span>
          <span class="col-path">路径</span>
          <span class="col-type">类型</span>
          <span class="col-size">大小</span>
          <span class="col-date">最后修改</span>
        </div>
        <div class="list-body">
          <div 
            v-for="item in filteredItems" 
            :key="item.path" 
            class="list-item"
          >
            <span class="col-check">
              <input 
                type="checkbox" 
                :checked="item.selected" 
                @click="toggleItem(cacheItems.indexOf(item))"
              />
            </span>
            <span class="col-name" :title="item.path">{{ item.name }}</span>
            <span class="col-path" :title="item.relative_path">{{ item.relative_path }}</span>
            <span class="col-type">
              <span class="type-badge" :style="{ backgroundColor: getTypeColor(item.item_type) }">
                {{ item.item_type }}
              </span>
            </span>
            <span class="col-size">{{ formatSize(item.size) }}</span>
            <span class="col-date">{{ item.last_modified || '-' }}</span>
          </div>
          <div v-if="filteredItems.length === 0" class="empty-state">
            {{ cacheItems.length === 0 ? '点击"扫描缓存"开始扫描' : '没有匹配的缓存项' }}
          </div>
        </div>
      </div>

      <!-- 镜像源 -->
      <div v-if="activeTab === 'mirror'" class="mirror-panel">
        <h2>镜像源配置信息</h2>
        
        <div class="mirror-info">
          <div class="info-row">
            <span class="label">当前镜像:</span>
            <span class="value strong">{{ mirrorInfo.mirror_name }}</span>
          </div>
          <div class="info-row">
            <span class="label">URL:</span>
            <span class="value">{{ mirrorInfo.mirror_url }}</span>
          </div>
        </div>
        
        <button 
          class="btn btn-primary" 
          @click="checkMirror(true)" 
          :disabled="isCheckingMirror"
        >
          {{ isCheckingMirror ? '🔄 检查中...' : '🔄 重新检查' }}
        </button>
      </div>

      <!-- 系统信息 -->
      <div v-if="activeTab === 'system'" class="system-panel">
        <h2>系统信息</h2>
        
        <div class="system-info" v-if="systemInfo">
          <div class="info-row">
            <span class="label">操作系统:</span>
            <span class="value">{{ systemInfo.os_name }} {{ systemInfo.os_version }}</span>
          </div>
          <div class="info-row">
            <span class="label">主机名:</span>
            <span class="value">{{ systemInfo.host_name }}</span>
          </div>
          <div class="info-row">
            <span class="label">CPU:</span>
            <span class="value">{{ systemInfo.cpu_name }} ({{ systemInfo.cpu_cores }} 核)</span>
          </div>
          <div class="info-row">
            <span class="label">内存:</span>
            <span class="value">{{ systemInfo.total_memory }}</span>
          </div>
          <div class="info-row">
            <span class="label">Rust 版本:</span>
            <span class="value">{{ systemInfo.rust_version }}</span>
          </div>
          <div class="info-row">
            <span class="label">Cargo 版本:</span>
            <span class="value">{{ systemInfo.cargo_version }}</span>
          </div>
        </div>
        
        <div v-else class="empty-state">
          加载中...
        </div>
      </div>

      <!-- 日志 -->
      <div v-if="activeTab === 'logs'" class="logs-panel">
        <div class="logs-header">
          <button class="btn" @click="clearLogs">🗑️ 清除日志</button>
        </div>
        <div class="logs-content">
          <div 
            v-for="(log, index) in logs" 
            :key="index" 
            :class="['log-line', 
              log.includes('✓') || log.includes('成功') ? 'success' : '',
              log.includes('✗') || log.includes('失败') || log.includes('错误') ? 'error' : '',
              log.includes('⚠') || log.includes('警告') ? 'warning' : ''
            ]"
          >
            {{ log }}
          </div>
          <div v-if="logs.length === 0" class="empty-state">暂无日志</div>
        </div>
      </div>
    </main>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  --bg-primary: #1e1e1e;
  --bg-secondary: #232323;
  --bg-tertiary: #2d2d2d;
  --text-primary: #e0e0e0;
  --text-secondary: #a0a0a0;
  --accent: #4a9eff;
  --danger: #ff6b6b;
  --success: #69db7c;
  --warning: #ffd43b;
  --border: #404040;
  --hover-bg: rgba(255, 255, 255, 0.15);
}

body[data-theme="light"] {
  --bg-primary: #f5f5f5;
  --bg-secondary: #ffffff;
  --bg-tertiary: #e8e8e8;
  --text-primary: #1a1a1a;
  --text-secondary: #666666;
  --accent: #2196f3;
  --danger: #f44336;
  --success: #4caf50;
  --warning: #ff9800;
  --border: #ddd;
  --hover-bg: rgba(0, 0, 0, 0.1);
}

body {
  font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, sans-serif;
  font-size: 14px;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 20px;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border);
}

.header-title {
  font-size: 18px;
  font-weight: 600;
}

.header-nav {
  display: flex;
  gap: 5px;
}

.header-nav button {
  padding: 8px 16px;
  border: none;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.2s;
}

.header-nav button:hover {
  background-color: rgba(136, 136, 136, 0.4);
}

.header-nav button.active {
  background-color: var(--accent);
  color: white;
}

.theme-toggle {
  margin-left: 10px !important;
  padding: 6px 12px !important;
  border: 1px solid var(--border) !important;
  background-color: var(--bg-tertiary) !important;
  color: var(--text-primary) !important;
  border-radius: 6px !important;
}

.theme-toggle:hover {
  background-color: rgba(136, 136, 136, 0.4) !important;
}

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 20px;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border);
  flex-wrap: wrap;
  gap: 10px;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.toolbar-right {
  color: var(--text-secondary);
}

.stats .selected {
  color: var(--success);
  font-weight: 600;
}

.separator {
  width: 1px;
  height: 24px;
  background-color: var(--border);
  margin: 0 8px;
}

.btn {
  padding: 8px 16px;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  cursor: pointer;
  border-radius: 6px;
  font-size: 14px;
  transition: all 0.2s;
}

.btn:hover:not(:disabled) {
  background-color: rgba(136, 136, 136, 0.4);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background-color: var(--accent);
  border-color: var(--accent);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: #3a8ee8;
}

.btn-danger {
  background-color: var(--danger);
  border-color: var(--danger);
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background-color: #e85555;
}

.select {
  padding: 8px 12px;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  border-radius: 6px;
  font-size: 14px;
}

.select:hover:not(:disabled) {
  background-color: rgba(136, 136, 136, 0.4);
}

.search-input {
  padding: 8px 12px;
  border: 1px solid var(--border);
  background-color: var(--bg-tertiary);
  color: var(--text-primary);
  border-radius: 6px;
  font-size: 14px;
  width: 200px;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent);
}

.search-input:hover {
  background-color: rgba(136, 136, 136, 0.4);
}

.main-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.cache-list {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.list-header {
  display: flex;
  padding: 12px 20px;
  background-color: var(--bg-tertiary);
  font-weight: 600;
  border-bottom: 1px solid var(--border);
}

.list-body {
  flex: 1;
  overflow-y: auto;
}

.list-item {
  display: flex;
  padding: 10px 20px;
  border-bottom: 1px solid var(--border);
  align-items: center;
}

.list-item:hover {
  background-color: var(--hover-bg);
}

.col-check {
  width: 40px;
}

.col-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.col-path {
  flex: 2;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-secondary);
  font-size: 12px;
}

.col-type {
  width: 120px;
}

.col-size {
  width: 100px;
  text-align: right;
}

.col-date {
  width: 160px;
  text-align: right;
  color: var(--text-secondary);
}

.type-badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  color: #000;
}

.empty-state {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
  color: var(--text-secondary);
}

.mirror-panel {
  padding: 30px;
}

.mirror-panel h2 {
  margin-bottom: 20px;
}

.mirror-info {
  background-color: var(--bg-secondary);
  padding: 20px;
  border-radius: 8px;
  margin-bottom: 20px;
}

.info-row {
  display: flex;
  padding: 10px 0;
  border-bottom: 1px solid var(--border);
}

.info-row:last-child {
  border-bottom: none;
}

.info-row .label {
  width: 120px;
  color: var(--text-secondary);
}

.info-row .value {
  flex: 1;
}

.info-row .strong {
  font-weight: 600;
  font-size: 16px;
}

.info-row .success {
  color: var(--success);
  font-size: 18px;
}

.info-row .error {
  color: var(--danger);
  font-size: 18px;
}

.mirror-tip {
  padding: 20px;
  border-radius: 8px;
  margin-bottom: 20px;
}

.mirror-tip.success {
  background-color: rgba(105, 219, 124, 0.1);
  border: 1px solid var(--success);
  color: var(--success);
}

.mirror-tip.warning {
  background-color: rgba(255, 212, 59, 0.1);
  border: 1px solid var(--warning);
  color: var(--warning);
}

.config-guide {
  margin-top: 15px;
  color: var(--text-primary);
}

.config-guide pre {
  margin-top: 10px;
  padding: 15px;
  background-color: var(--bg-primary);
  border-radius: 4px;
  overflow-x: auto;
}

.system-panel {
  padding: 30px;
}

.system-panel h2 {
  margin-bottom: 20px;
}

.system-info {
  background-color: var(--bg-secondary);
  padding: 20px;
  border-radius: 8px;
}

.logs-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 20px;
  overflow: hidden;
}

.logs-header {
  margin-bottom: 10px;
}

.logs-content {
  flex: 1;
  overflow-y: auto;
  background-color: var(--bg-secondary);
  padding: 15px;
  border-radius: 8px;
  font-family: 'Consolas', monospace;
  font-size: 13px;
}

.log-line {
  padding: 3px 0;
}

.log-line.success {
  color: var(--success);
}

.log-line.error {
  color: var(--danger);
}

.log-line.warning {
  color: var(--warning);
}
</style>
