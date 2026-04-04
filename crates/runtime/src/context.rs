//! 上下文管理

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 上下文配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    /// 最大长度（token数）
    pub max_length: usize,
    /// 压缩策略
    pub compression_strategy: CompressionStrategy,
    /// 是否启用缓存
    pub enable_caching: bool,
    /// 缓存大小
    pub cache_size: usize,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            max_length: 4096,
            compression_strategy: CompressionStrategy::Smart,
            enable_caching: true,
            cache_size: 100,
        }
    }
}

/// 压缩策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionStrategy {
    /// 无压缩
    None,
    /// 智能压缩
    Smart,
    /// 激进压缩
    Aggressive,
}

/// 上下文压缩器
#[derive(Debug, Clone)]
pub struct ContextCompression {
    /// 策略
    strategy: CompressionStrategy,
    /// 压缩级别
    level: u8,
}

impl ContextCompression {
    /// 创建新的上下文压缩器
    pub fn new(strategy: CompressionStrategy, level: u8) -> Self {
        Self { strategy, level }
    }

    /// 压缩上下文
    pub async fn compress(&self, context: &Context) -> anyhow::Result<Context> {
        // TODO: 实现压缩算法
        Ok(context.clone())
    }

    /// 解压缩上下文
    pub async fn decompress(&self, context: &Context) -> anyhow::Result<Context> {
        // TODO: 实现解压缩算法
        Ok(context.clone())
    }
}

/// 上下文
#[derive(Debug, Clone, Default)]
pub struct Context {
    /// 上下文ID
    id: String,
    /// 内容
    content: String,
    /// 元数据
    metadata: std::collections::HashMap<String, String>,
    /// 标记
    tokens: Vec<String>,
    /// 版本
    version: u64,
}

impl Context {
    /// 创建新上下文
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
            metadata: std::collections::HashMap::new(),
            tokens: Vec::new(),
            version: 0,
        }
    }

    /// 获取上下文ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 获取内容
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 更新内容
    pub fn update_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
        self.version += 1;
    }

    /// 追加内容
    pub fn append_content(&mut self, content: impl Into<String>) {
        self.content.push_str(&content.into());
        self.version += 1;
    }

    /// 获取元数据
    pub fn metadata(&self) -> &std::collections::HashMap<String, String> {
        &self.metadata
    }

    /// 设置元数据
    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// 获取版本
    pub fn version(&self) -> u64 {
        self.version
    }

    /// 获取标记数量
    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    /// 设置标记
    pub fn set_tokens(&mut self, tokens: Vec<String>) {
        self.tokens = tokens;
    }

    /// 获取标记
    pub fn tokens(&self) -> &[String] {
        &self.tokens
    }
}

/// 上下文管理器
pub struct ContextManager {
    /// 上下文映射（上下文ID -> 上下文）
    contexts: Arc<RwLock<std::collections::HashMap<String, Arc<RwLock<Context>>>>>,
    /// 配置
    config: ContextConfig,
    /// 压缩器
    compressor: ContextCompression,
}

impl ContextManager {
    /// 创建新的上下文管理器
    pub fn new(config: ContextConfig) -> Self {
        let compressor = ContextCompression::new(config.compression_strategy, 6);
        Self {
            contexts: Arc::new(RwLock::new(std::collections::HashMap::new())),
            config,
            compressor,
        }
    }

    /// 创建新上下文
    pub async fn create_context(
        &self,
        id: impl Into<String>,
        content: impl Into<String>,
    ) -> anyhow::Result<String> {
        let id = id.into();
        let context = Context::new(&id, content);
        let context_arc = Arc::new(RwLock::new(context));

        let mut contexts = self.contexts.write().await;
        contexts.insert(id.clone(), context_arc);

        Ok(id)
    }

    /// 获取上下文
    pub async fn get_context(&self, id: &str) -> Option<Arc<RwLock<Context>>> {
        let contexts = self.contexts.read().await;
        contexts.get(id).cloned()
    }

    /// 更新上下文
    pub async fn update_context(
        &self,
        id: &str,
        content: impl Into<String>,
    ) -> anyhow::Result<()> {
        let contexts = self.contexts.read().await;
        if let Some(context) = contexts.get(id) {
            let mut context = context.write().await;
            context.update_content(content);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Context not found: {}", id))
        }
    }

    /// 删除上下文
    pub async fn delete_context(&self, id: &str) -> Option<Arc<RwLock<Context>>> {
        let mut contexts = self.contexts.write().await;
        contexts.remove(id)
    }

    /// 获取所有上下文ID
    pub async fn context_ids(&self) -> Vec<String> {
        let contexts = self.contexts.read().await;
        contexts.keys().cloned().collect()
    }

    /// 压缩上下文
    pub async fn compress_context(&self, id: &str) -> anyhow::Result<Context> {
        let contexts = self.contexts.read().await;
        if let Some(context) = contexts.get(id) {
            let context = context.read().await;
            self.compressor.compress(&context).await
        } else {
            Err(anyhow::anyhow!("Context not found: {}", id))
        }
    }

    /// 获取配置
    pub fn config(&self) -> &ContextConfig {
        &self.config
    }
}