//! API客户端错误处理

use serde::Deserialize;
use serde_json::Error as JsonError;
use std::{fmt, io};
use thiserror::Error;

/// API客户端结果类型
pub type Result<T> = std::result::Result<T, ApiError>;

/// API客户端错误
#[derive(Debug, Error)]
pub enum ApiError {
    /// 网络错误
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// HTTP错误
    #[error("HTTP error {status}: {message}")]
    Http {
        /// 状态码
        status: u16,
        /// 错误消息
        message: String,
    },

    /// 超时错误
    #[error("Request timeout")]
    Timeout,

    /// 序列化错误
    #[error("Serialization error: {0}")]
    Serialization(#[from] JsonError),

    /// IO错误
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// 压缩错误
    #[error("Compression error: {0}")]
    Compression(String),

    /// 业务逻辑错误
    #[error("Business error [{code}]: {message}")]
    Business {
        /// 错误代码
        code: String,
        /// 错误消息
        message: String,
    },

    /// 流式错误
    #[error("Stream error: {0}")]
    Stream(String),

    /// 工具调用错误
    #[error("Tool call error: {0}")]
    ToolCall(String),

    /// 认证错误
    #[error("Authentication error: {0}")]
    Auth(String),

    /// 配置错误
    #[error("Configuration error: {0}")]
    Config(String),

    /// 其他错误
    #[error("Other error: {0}")]
    Other(String),
}

impl ApiError {
    /// 创建HTTP错误
    pub fn http(status: u16, message: impl Into<String>) -> Self {
        Self::Http {
            status,
            message: message.into(),
        }
    }

    /// 创建业务逻辑错误
    pub fn business(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Business {
            code: code.into(),
            message: message.into(),
        }
    }

    /// 创建流式错误
    pub fn stream(message: impl Into<String>) -> Self {
        Self::Stream(message.into())
    }

    /// 创建工具调用错误
    pub fn tool_call(message: impl Into<String>) -> Self {
        Self::ToolCall(message.into())
    }

    /// 创建认证错误
    pub fn auth(message: impl Into<String>) -> Self {
        Self::Auth(message.into())
    }

    /// 创建配置错误
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config(message.into())
    }

    /// 创建其他错误
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}

impl From<String> for ApiError {
    fn from(message: String) -> Self {
        Self::Other(message)
    }
}

impl From<&str> for ApiError {
    fn from(message: &str) -> Self {
        Self::Other(message.to_string())
    }
}

/// API错误响应
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    /// 错误类型
    #[serde(rename = "type")]
    pub error_type: String,
    /// 错误消息
    pub message: String,
    /// 错误代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = &self.code {
            write!(f, "[{}] {}: {}", code, self.error_type, self.message)
        } else {
            write!(f, "{}: {}", self.error_type, self.message)
        }
    }
}