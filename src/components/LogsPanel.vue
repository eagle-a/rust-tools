<script setup lang="ts">
defineProps<{
  logs: string[];
}>();

const emit = defineEmits<{
  clearLogs: [];
}>();
</script>

<template>
  <div class="logs-panel">
    <div class="logs-header">
      <button class="btn" @click="emit('clearLogs')">🗑️ 清除日志</button>
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
</template>

<style scoped>
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

.btn:hover {
  background-color: rgba(136, 136, 136, 0.4);
}

.empty-state {
  color: var(--text-secondary);
}
</style>