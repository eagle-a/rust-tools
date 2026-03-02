export interface CacheItem {
  name: string;
  path: string;
  relative_path: string;
  size: number;
  item_type: string;
  last_modified: string | null;
  selected: boolean;
}

export interface ScanResult {
  items: CacheItem[];
  total_size: number;
  selected_size: number;
  logs: string[];
}

export interface CleanResult {
  item_name: string;
  item_type: string;
  size_freed: number;
  success: boolean;
  error_message: string | null;
}

export interface MirrorInfo {
  is_tuna: boolean;
  mirror_name: string;
  mirror_url: string;
}

export interface SystemInfo {
  os_name: string;
  os_version: string;
  host_name: string;
  cpu_name: string;
  cpu_cores: number;
  total_memory: string;
  rust_version: string;
  cargo_version: string;
}