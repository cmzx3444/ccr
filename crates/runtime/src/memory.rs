//! 记忆管理

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 记忆配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// 最大记忆数量
    pub max_memories: usize,
    /// 存储路径
    pub storage_path: Option<String>,
    /// 是否启用持久化
    pub enable_persistence: bool,
    /// 压缩等级
    pub compression_level: u8,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_memories: 1000,
            storage_path: None,
            enable_persistence: true,
            compression_level: 6,
        }
    }
}

/// 记忆存储 trait
pub trait MemoryStore: Send + Sync {
    /// 保存记忆
    async fn save_memory(&self, memory: &Memory) -> anyhow::Result<()>;

    /// 加载记忆
    async fn load_memory(&self, memory_id: &str) -> anyhow::Result<Option<Memory>>;

    /// 删除记忆
    async fn delete_memory(&self, memory_id: &str) -> anyhow::Result<()>;

    /// 列出所有记忆ID
    async fn list_memory_ids(&self) -> anyhow::Result<Vec<String>>;

    /// 搜索记忆
    async fn search_memories(&self, query: &str, limit: usize) -> anyhow::Result<Vec<Memory>>;
}

/// 记忆
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// 记忆ID
    pub id: String,
    /// 内容
    pub content: String,
    /// 元数据
    pub metadata: std::collections::HashMap<String, String>,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 最后访问时间
    pub last_accessed_at: chrono::DateTime<chrono::Utc>,
    /// 访问次数
    pub access_count: u64,
    /// 权重
    pub weight: f32,
}

impl Memory {
    /// 创建新记忆
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: id.into(),
            content: content.into(),
            metadata: std::collections::HashMap::new(),
            created_at: now,
            last_accessed_at: now,
            access_count: 0,
            weight: 1.0,
        }
    }

    /// 更新最后访问时间
    pub fn touch(&mut self) {
        self.last_accessed_at = chrono::Utc::now();
        self.access_count += 1;
    }

    /// 设置权重
    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
    }

    /// 设置元数据
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }
}

/// 内存记忆存储（内存实现）
pub struct InMemoryMemoryStore {
    /// 记忆映射（记忆ID -> 记忆）
    memories: Arc<RwLock<std::collections::HashMap<String, Memory>>>,
}

impl InMemoryMemoryStore {
    /// 创建新的内存记忆存储
    pub fn new() -> Self {
        Self {
            memories: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl MemoryStore for InMemoryMemoryStore {
    async fn save_memory(&self, memory: &Memory) -> anyhow::Result<()> {
        let mut memories = self.memories.write().await;
        memories.insert(memory.id.clone(), memory.clone());
        Ok(())
    }

    async fn load_memory(&self, memory_id: &str) -> anyhow::Result<Option<Memory>> {
        let memories = self.memories.read().await;
        Ok(memories.get(memory_id).cloned())
    }

    async fn delete_memory(&self, memory_id: &str) -> anyhow::Result<()> {
        let mut memories = self.memories.write().await;
        memories.remove(memory_id);
        Ok(())
    }

    async fn list_memory_ids(&self) -> anyhow::Result<Vec<String>> {
        let memories = self.memories.read().await;
        Ok(memories.keys().cloned().collect())
    }

    async fn search_memories(&self, query: &str, limit: usize) -> anyhow::Result<Vec<Memory>> {
        let memories = self.memories.read().await;
        let mut results: Vec<Memory> = memories
            .values()
            .filter(|memory| memory.content.contains(query))
            .cloned()
            .collect();

        // 按权重排序
        results.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);

        Ok(results)
    }
}

/// 记忆管理器
pub struct MemoryManager {
    /// 记忆存储
    store: Arc<dyn MemoryStore>,
    /// 配置
    config: MemoryConfig,
}

impl MemoryManager {
    /// 创建新的记忆管理器
    pub fn new(config: MemoryConfig) -> Self {
        let store = Arc::new(InMemoryMemoryStore::new());
        Self { store, config }
    }

    /// 创建新记忆
    pub async fn create_memory(
        &self,
        id: impl Into<String>,
        content: impl Into<String>,
    ) -> anyhow::Result<Memory> {
        let memory = Memory::new(id, content);
        self.store.save_memory(&memory).await?;
        Ok(memory)
    }

    /// 获取记忆
    pub async fn get_memory(&self, memory_id: &str) -> anyhow::Result<Option<Memory>> {
        let memory = self.store.load_memory(memory_id).await?;
        if let Some(mut memory) = memory {
            memory.touch();
            self.store.save_memory(&memory).await?;
            Ok(Some(memory))
        } else {
            Ok(None)
        }
    }

    /// 更新记忆
    pub async fn update_memory(
        &self,
        memory_id: &str,
        content: impl Into<String>,
    ) -> anyhow::Result<()> {
        if let Some(mut memory) = self.store.load_memory(memory_id).await? {
            memory.content = content.into();
            memory.touch();
            self.store.save_memory(&memory).await
        } else {
            Err(anyhow::anyhow!("Memory not found: {}", memory_id))
        }
    }

    /// 删除记忆
    pub async fn delete_memory(&self, memory_id: &str) -> anyhow::Result<()> {
        self.store.delete_memory(memory_id).await
    }

    /// 搜索记忆
    pub async fn search_memories(
        &self,
        query: &str,
        limit: usize,
    ) -> anyhow::Result<Vec<Memory>> {
        self.store.search_memories(query, limit).await
    }

    /// 获取配置
    pub fn config(&self) -> &MemoryConfig {
        &self.config
    }

    /// 获取存储
    pub fn store(&self) -> &Arc<dyn MemoryStore> {
        &self.store
    }
}