//! Claude Code Runtime
//!
//! 提供运行时服务，包括：
//! - 会话状态管理
//! - 上下文压缩
//! - MCP编排
//! - Prompt构建
//! - 记忆管理

pub mod session;
pub mod context;
pub mod mcp;
pub mod prompt;
pub mod memory;
pub mod state;
pub mod compact;
pub mod coordinator;

// 重新导出主要类型
pub use session::{Session, SessionManager, SessionConfig, SessionState};
pub use context::{Context, ContextManager, ContextConfig, ContextCompression};
pub use mcp::{MCPManager, MCPConnection, MCPResource, MCPTool};
pub use prompt::{PromptBuilder, PromptTemplate, PromptContext};
pub use memory::{Memory, MemoryManager, MemoryStore};
pub use state::{State, StateManager, StateTransition};
pub use compact::{Compactor, CompressionStrategy, CompactConfig};
pub use coordinator::{Coordinator, CoordinatorMode, CoordinatorTask};

use anyhow::Result;

/// 运行时配置
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// 会话配置
    pub session: SessionConfig,
    /// 上下文配置
    pub context: ContextConfig,
    /// MCP配置
    pub mcp: MCPConfig,
    /// 压缩配置
    pub compact: CompactConfig,
    /// 内存配置
    pub memory: MemoryConfig,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            session: SessionConfig::default(),
            context: ContextConfig::default(),
            mcp: MCPConfig::default(),
            compact: CompactConfig::default(),
            memory: MemoryConfig::default(),
        }
    }
}

/// MCP配置
#[derive(Debug, Clone)]
pub struct MCPConfig {
    /// 是否启用MCP支持
    pub enabled: bool,
    /// MCP服务器列表
    pub servers: Vec<MCPServerConfig>,
    /// 自动连接
    pub auto_connect: bool,
}

impl Default for MCPConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            servers: Vec::new(),
            auto_connect: true,
        }
    }
}

/// MCP服务器配置
#[derive(Debug, Clone)]
pub struct MCPServerConfig {
    /// 服务器名称
    pub name: String,
    /// 服务器类型
    pub server_type: MCPServerType,
    /// 连接参数
    pub connection: MCPConnectionParams,
}

/// MCP服务器类型
#[derive(Debug, Clone)]
pub enum MCPServerType {
    /// 标准MCP服务器
    Standard,
    /// Claude in Chrome
    ClaudeInChrome,
    /// 自定义服务器
    Custom,
}

/// MCP连接参数
#[derive(Debug, Clone)]
pub struct MCPConnectionParams {
    /// 连接类型
    pub connection_type: MCPConnectionType,
    /// 主机地址
    pub host: Option<String>,
    /// 端口
    pub port: Option<u16>,
    /// 命令（用于stdio）
    pub command: Option<String>,
    /// 参数
    pub args: Vec<String>,
}

/// MCP连接类型
#[derive(Debug, Clone)]
pub enum MCPConnectionType {
    /// WebSocket连接
    WebSocket,
    /// Stdio连接
    Stdio,
    /// HTTP/SSE连接
    HttpSse,
}

/// 内存配置
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    /// 是否启用内存
    pub enabled: bool,
    /// 最大记忆数量
    pub max_memories: usize,
    /// 记忆存储路径
    pub storage_path: Option<String>,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_memories: 1000,
            storage_path: None,
        }
    }
}

/// 运行时管理器
pub struct Runtime {
    /// 会话管理器
    session_manager: SessionManager,
    /// 上下文管理器
    context_manager: ContextManager,
    /// MCP管理器
    mcp_manager: Option<MCPManager>,
    /// 内存管理器
    memory_manager: MemoryManager,
    /// 协调器
    coordinator: Coordinator,
    /// 配置
    config: RuntimeConfig,
}

impl Runtime {
    /// 创建新的运行时
    pub async fn new(config: RuntimeConfig) -> Result<Self> {
        let session_manager = SessionManager::new(config.session.clone());
        let context_manager = ContextManager::new(config.context.clone());
        let memory_manager = MemoryManager::new(config.memory.clone());

        let mcp_manager = if config.mcp.enabled {
            Some(MCPManager::new(config.mcp.clone()).await?)
        } else {
            None
        };

        let coordinator = Coordinator::new();

        Ok(Self {
            session_manager,
            context_manager,
            mcp_manager,
            memory_manager,
            coordinator,
            config,
        })
    }

    /// 获取会话管理器
    pub fn session_manager(&self) -> &SessionManager {
        &self.session_manager
    }

    /// 获取上下文管理器
    pub fn context_manager(&self) -> &ContextManager {
        &self.context_manager
    }

    /// 获取MCP管理器
    pub fn mcp_manager(&self) -> Option<&MCPManager> {
        self.mcp_manager.as_ref()
    }

    /// 获取内存管理器
    pub fn memory_manager(&self) -> &MemoryManager {
        &self.memory_manager
    }

    /// 获取协调器
    pub fn coordinator(&self) -> &Coordinator {
        &self.coordinator
    }

    /// 获取配置
    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }

    /// 启动运行时
    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting runtime...");

        // 启动MCP管理器
        if let Some(mcp_manager) = &mut self.mcp_manager {
            mcp_manager.start().await?;
        }

        // 启动协调器
        self.coordinator.start().await?;

        tracing::info!("Runtime started successfully");

        Ok(())
    }

    /// 停止运行时
    pub async fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping runtime...");

        // 停止协调器
        self.coordinator.stop().await?;

        // 停止MCP管理器
        if let Some(mcp_manager) = &mut self.mcp_manager {
            mcp_manager.stop().await?;
        }

        tracing::info!("Runtime stopped successfully");

        Ok(())
    }
}

/// 初始化运行时
pub async fn init_runtime(config: RuntimeConfig) -> Result<Runtime> {
    Runtime::new(config).await
}