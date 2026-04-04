//! Claude Code API Client
//!
//! 提供完整的API客户端功能，包括：
//! - HTTP客户端和请求管理
//! - OAuth认证
//! - 流式响应支持
//! - 工具调用（Tool Use）集成
//! - 重试和错误处理

pub mod client;
pub mod streaming;
pub mod tool_use;
pub mod oauth;
pub mod error;
pub mod types;
pub mod provider;
pub mod integration;

// 重新导出主要类型
pub use client::{ApiClient, ApiClientConfig};
pub use error::{ApiError, Result};
pub use streaming::{StreamHandler, StreamResponse};
pub use tool_use::{ToolResult, ToolCall, ToolCallHandler, ToolDefinition};
pub use integration::{ApiToolHandler, ToolRegistryAdapter, ToolToApiConverter, DefaultToolConverter};
pub use types::{
    ApiRequest, ApiResponse, ApiMessage, ApiTool, ApiModel, ApiContentBlock,
    StreamEvent, MessageContent, ToolChoice, ApiUsage, ApiRole, ApiContentType
};
pub use provider::{ApiProvider, ProviderConfig, ProviderType};

/// API客户端版本
pub const API_VERSION: &str = "2023-06-01";

/// Claude API基础URL
pub const CLAUDE_API_BASE_URL: &str = "https://api.anthropic.com";

/// 初始化API客户端
pub async fn init_client(api_key: Option<String>) -> Result<ApiClient> {
    let config = ApiClientConfig::default();
    let mut client = ApiClient::new(CLAUDE_API_BASE_URL, config);

    if let Some(key) = api_key {
        client = client.with_api_key(&key);
    }

    Ok(client)
}