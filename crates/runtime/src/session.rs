//! 会话状态管理

use crate::memory::MemoryStore;
use crate::context::Context;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// 会话配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// 会话超时时间（秒）
    pub timeout_seconds: Option<u64>,
    /// 最大上下文长度
    pub max_context_length: usize,
    /// 是否启用压缩
    pub enable_compression: bool,
    /// 是否启用持久化
    pub enable_persistence: bool,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: Some(3600),
            max_context_length: 4096,
            enable_compression: true,
            enable_persistence: false,
        }
    }
}

/// 会话状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionState {
    /// 活跃
    Active,
    /// 挂起
    Suspended,
    /// 已结束
    Terminated,
    /// 错误
    Error(String),
}

/// 会话
pub struct Session {
    /// 会话ID
    id: String,
    /// 用户ID
    user_id: Option<String>,
    /// 会话状态
    state: SessionState,
    /// 上下文
    context: Arc<RwLock<Context>>,
    /// 内存存储
    memory_store: Arc<dyn MemoryStore>,
    /// 配置
    config: SessionConfig,
    /// 创建时间
    created_at: chrono::DateTime<chrono::Utc>,
    /// 最后活动时间
    last_activity_at: chrono::DateTime<chrono::Utc>,
}

impl Session {
    /// 创建新会话
    pub fn new(
        user_id: Option<String>,
        context: Context,
        memory_store: Arc<dyn MemoryStore>,
        config: SessionConfig,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            state: SessionState::Active,
            context: Arc::new(RwLock::new(context)),
            memory_store,
            config,
            created_at: now,
            last_activity_at: now,
        }
    }

    /// 获取会话ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 获取用户ID
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    /// 获取会话状态
    pub fn state(&self) -> &SessionState {
        &self.state
    }

    /// 更新会话状态
    pub fn set_state(&mut self, state: SessionState) {
        self.state = state;
        self.last_activity_at = chrono::Utc::now();
    }

    /// 获取上下文
    pub async fn context(&self) -> tokio::sync::RwLockReadGuard<'_, Context> {
        self.context.read().await
    }

    /// 获取上下文（可变）
    pub async fn context_mut(&self) -> tokio::sync::RwLockWriteGuard<'_, Context> {
        self.context.write().await
    }

    /// 获取内存存储
    pub fn memory_store(&self) -> &Arc<dyn MemoryStore> {
        &self.memory_store
    }

    /// 获取配置
    pub fn config(&self) -> &SessionConfig {
        &self.config
    }

    /// 更新最后活动时间
    pub fn touch(&mut self) {
        self.last_activity_at = chrono::Utc::now();
    }

    /// 检查会话是否过期
    pub fn is_expired(&self) -> bool {
        if let Some(timeout) = self.config.timeout_seconds {
            let elapsed = chrono::Utc::now()
                .signed_duration_since(self.last_activity_at)
                .num_seconds();
            elapsed > timeout as i64
        } else {
            false
        }
    }

    /// 保存会话状态
    pub async fn save(&self) -> anyhow::Result<()> {
        if self.config.enable_persistence {
            // TODO: 实现持久化
        }
        Ok(())
    }

    /// 加载会话状态
    pub async fn load(_session_id: &str, _memory_store: Arc<dyn MemoryStore>) -> anyhow::Result<Self> {
        // TODO: 实现加载
        Err(anyhow::anyhow!("Not implemented"))
    }
}

/// 会话管理器
pub struct SessionManager {
    /// 会话映射（会话ID -> 会话）
    sessions: Arc<RwLock<std::collections::HashMap<String, Arc<RwLock<Session>>>>>,
    /// 默认配置
    default_config: SessionConfig,
}

impl SessionManager {
    /// 创建新的会话管理器
    pub fn new(default_config: SessionConfig) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
            default_config,
        }
    }

    /// 创建新会话
    pub async fn create_session(
        &self,
        user_id: Option<String>,
        context: Context,
        memory_store: Arc<dyn MemoryStore>,
        config: Option<SessionConfig>,
    ) -> anyhow::Result<String> {
        let session = Session::new(
            user_id,
            context,
            memory_store,
            config.unwrap_or(self.default_config.clone()),
        );
        let session_id = session.id().to_string();
        let session_arc = Arc::new(RwLock::new(session));

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session_arc);

        Ok(session_id)
    }

    /// 获取会话
    pub async fn get_session(&self, session_id: &str) -> Option<Arc<RwLock<Session>>> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// 移除会话
    pub async fn remove_session(&self, session_id: &str) -> Option<Arc<RwLock<Session>>> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id)
    }

    /// 获取所有会话ID
    pub async fn session_ids(&self) -> Vec<String> {
        let sessions = self.sessions.read().await;
        sessions.keys().cloned().collect()
    }

    /// 清理过期会话
    pub async fn cleanup_expired_sessions(&self) -> anyhow::Result<Vec<String>> {
        let mut expired = Vec::new();
        let mut sessions = self.sessions.write().await;

        for (session_id, session) in sessions.iter() {
            if session.read().await.is_expired() {
                expired.push(session_id.clone());
            }
        }

        for session_id in &expired {
            sessions.remove(session_id);
        }

        Ok(expired)
    }

    /// 获取默认配置
    pub fn default_config(&self) -> &SessionConfig {
        &self.default_config
    }
}