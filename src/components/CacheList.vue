<script setup lang="ts">
import { computed } from 'vue';
import type { CacheItem } from '../types/index';

const props = defineProps<{
  items: CacheItem[];
  filterType: string;
  searchQuery: string;
  sortBy: string;
  sortAsc: boolean;
}>();

const emit = defineEmits<{
  toggleItem: [index: number];
}>();

const filteredItems = computed(() => {
  let items = [...props.items];
  
  if (props.filterType) {
    items = items.filter(item => item.item_type === props.filterType);
  }
  if (props.searchQuery) {
    const query = props.searchQuery.toLowerCase();
    items = items.filter(item => item.name.toLowerCase().includes(query));
  }
  
  items.sort((a, b) => {
    let cmp = 0;
    switch (props.sortBy) {
      case 'name':
        cmp = a.name.localeCompare(b.name);
        break;
      case 'size':
        cmp = a.size - b.size;
        break;
      case 'date':
        cmp = (a.last_modified || '').localeCompare(b.last_modified || '');
        break;
      case 'type':
        cmp = a.item_type.localeCompare(b.item_type);
        break;
      default:
        cmp = 0;
    }
    return props.sortAsc ? cmp : -cmp;
  });
  
  return items;
});

function formatSize(size: number): string {
  const KB = 1024;
  const MB = KB * 1024;
  const GB = MB * 1024;

  if (size >= GB) return `${(size / GB).toFixed(2)} GB`;
  if (size >= MB) return `${(size / MB).toFixed(2)} MB`;
  if (size >= KB) return `${(size / KB).toFixed(2)} KB`;
  return `${size} B`;
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

function getLocalIndex(item: CacheItem): number {
  return props.items.indexOf(item);
}
</script>

<template>
  <div class="cache-list">
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
            @click="emit('toggleItem', getLocalIndex(item))"
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
        {{ items.length === 0 ? '点击"扫描缓存"开始扫描' : '没有匹配的缓存项' }}
      </div>
    </div>
  </div>
</template>

<style scoped>
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
</style>