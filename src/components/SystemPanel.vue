<script setup lang="ts">
import type { SystemInfo } from '../types/index';

defineProps<{
  systemInfo: SystemInfo | null;
}>();
</script>

<template>
  <div class="system-panel">
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
</template>

<style scoped>
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

.empty-state {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 200px;
  color: var(--text-secondary);
}
</style>