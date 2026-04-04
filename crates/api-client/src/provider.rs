//! API提供者模块
//!
//! 提供多模型提供者支持

use crate::error::Result;

/// API提供者类型
#[derive(Debug, Clone)]
pub enum ProviderType {
    /// Anthropic Claude
    Anthropic,
    /// OpenAI
    OpenAI,
    /// 其他提供者
    Custom(String),
}

/// 提供者配置
#[derive(Debug, Clone)]
pub struct ProviderConfig {
    /// 提供者类型
    pub provider_type: ProviderType,
    /// 基础URL
    pub base_url: String,
    /// API密钥
    pub api_key: Option<String>,
}

/// API提供者
pub struct ApiProvider {
    /// 配置
    pub config: ProviderConfig,
    // 内部状态
}

impl ApiProvider {
    /// 创建新的API提供者
    pub fn new(config: ProviderConfig) -> Result<Self> {
        todo!("Provider support not yet implemented")
    }
}