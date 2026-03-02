<script setup lang="ts">
import type { MirrorInfo } from '../types/index';

defineProps<{
  mirrorInfo: MirrorInfo;
  isChecking: boolean;
}>();

const emit = defineEmits<{
  checkMirror: [];
}>();
</script>

<template>
  <div class="mirror-panel">
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
      <div class="info-row">
        <span class="label">状态:</span>
        <span class="value" :class="mirrorInfo.is_tuna ? 'success' : 'warning'">
          {{ mirrorInfo.is_tuna ? '✓ 已配置国内镜像' : '⚠ 使用默认源（可能较慢）' }}
        </span>
      </div>
    </div>
    
    <button 
      class="btn btn-primary" 
      @click="emit('checkMirror')" 
      :disabled="isChecking"
    >
      {{ isChecking ? '🔄 检查中...' : '🔄 重新检查' }}
    </button>
  </div>
</template>

<style scoped>
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
}

.info-row .warning {
  color: var(--warning);
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
</style>