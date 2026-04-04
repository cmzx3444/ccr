//! Claude API 类型定义
//!
//! 定义了Claude API请求和响应的核心数据结构，
//! 包括消息、工具调用、流式事件等。

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// API消息角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ApiRole {
    /// 用户消息
    User,
    /// 助手消息
    Assistant,
}

/// API内容块类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApiContentType {
    /// 文本内容
    Text,
    /// 工具使用
    ToolUse,
    /// 工具结果
    ToolResult,
    /// 图像内容
    Image,
    /// 文档内容
    Document,
}

/// API内容块
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ApiContentBlock {
    /// 文本块
    Text {
        /// 文本内容
        text: String,
    },
    /// 工具使用块
    ToolUse {
        /// 工具使用ID
        id: String,
        /// 工具名称
        name: String,
        /// 输入参数
        input: Value,
    },
    /// 工具结果块
    ToolResult {
        /// 工具使用ID
        tool_use_id: String,
        /// 工具结果内容
        content: Value,
        /// 是否失败
        #[serde(skip_serializing_if = "Option::is_none")]
        is_error: Option<bool>,
    },
    /// 图像块
    Image {
        /// 图像数据
        source: ImageSource,
    },
    /// 文档块
    Document {
        /// 文档源
        source: DocumentSource,
    },
}

/// 图像源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSource {
    /// 数据类型
    #[serde(rename = "type")]
    pub data_type: String,
    /// 媒体类型
    pub media_type: String,
    /// 图像数据（base64编码）
    pub data: String,
}

/// 文档源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSource {
    /// 数据类型
    #[serde(rename = "type")]
    pub data_type: String,
    /// 媒体类型
    pub media_type: String,
    /// 文档数据（base64编码）
    pub data: String,
}

/// API消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMessage {
    /// 角色
    pub role: ApiRole,
    /// 内容（可以是字符串或内容块数组）
    #[serde(flatten)]
    pub content: MessageContent,
}

/// 消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// 简单文本内容
    Text(String),
    /// 内容块数组
    Blocks(Vec<ApiContentBlock>),
}

/// API工具定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiTool {
    /// 工具名称
    pub name: String,
    /// 工具描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 输入模式（JSON Schema）
    pub input_schema: Value,
}

/// API模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApiModel {
    /// Claude 3.5 Sonnet
    #[serde(rename = "claude-3-5-sonnet-20241022")]
    Claude35Sonnet20241022,
    /// Claude 3.5 Haiku
    #[serde(rename = "claude-3-5-haiku-20241022")]
    Claude35Haiku20241022,
    /// Claude 3 Opus
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus20240229,
    /// 自定义模型名称
    #[serde(untagged)]
    Custom(String),
}

impl Default for ApiModel {
    fn default() -> Self {
        Self::Claude35Sonnet20241022
    }
}

impl std::fmt::Display for ApiModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Claude35Sonnet20241022 => write!(f, "claude-3-5-sonnet-20241022"),
            Self::Claude35Haiku20241022 => write!(f, "claude-3-5-haiku-20241022"),
            Self::Claude3Opus20240229 => write!(f, "claude-3-opus-20240229"),
            Self::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// API请求
#[derive(Debug, Clone, Serialize)]
pub struct ApiRequest {
    /// 模型
    pub model: ApiModel,
    /// 消息列表
    pub messages: Vec<ApiMessage>,
    /// 系统提示
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,
    /// 最大令牌数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// 工具列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ApiTool>>,
    /// 工具选择
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// 思考配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingConfig>,
    /// 流式响应
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// 温度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Top-P采样
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// Top-K采样
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    /// 停止序列
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    /// 元数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Value>>,
    /// Beta功能头
    #[serde(skip_serializing_if = "Option::is_none")]
    pub betas: Option<Vec<String>>,
}

/// 工具选择策略
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum ToolChoice {
    /// 自动选择
    Auto,
    /// 任何工具
    Any,
    /// 指定工具
    Tool {
        /// 工具名称
        name: String,
    },
}

/// 思考配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingConfig {
    /// 思考预算（令牌数）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_tokens: Option<u32>,
    /// 思考类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// API响应
#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse {
    /// 响应ID
    pub id: String,
    /// 响应类型
    #[serde(rename = "type")]
    pub response_type: String,
    /// 角色
    pub role: ApiRole,
    /// 内容
    pub content: Vec<ApiContentBlock>,
    /// 模型
    pub model: String,
    /// 停止原因
    pub stop_reason: Option<String>,
    /// 停止序列
    pub stop_sequence: Option<String>,
    /// 使用统计
    pub usage: ApiUsage,
}

/// API使用统计
#[derive(Debug, Clone, Deserialize)]
pub struct ApiUsage {
    /// 输入令牌数
    pub input_tokens: u32,
    /// 输出令牌数
    pub output_tokens: u32,
}

/// 流式响应事件
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamEvent {
    /// 消息开始
    MessageStart {
        /// 消息
        message: MessageStart,
    },
    /// 内容块开始
    ContentBlockStart {
        /// 内容块索引
        index: u32,
        /// 内容块
        content_block: ContentBlockStart,
    },
    /// 内容块增量
    ContentBlockDelta {
        /// 内容块索引
        index: u32,
        /// 增量
        delta: ContentBlockDelta,
    },
    /// 内容块结束
    ContentBlockStop {
        /// 内容块索引
        index: u32,
    },
    /// 消息增量
    MessageDelta {
        /// 增量
        delta: MessageDelta,
        /// 使用统计
        usage: ApiUsage,
    },
    /// 消息结束
    MessageStop,
    /// 错误事件
    Error {
        /// 错误
        error: StreamError,
    },
}

/// 消息开始事件
#[derive(Debug, Clone, Deserialize)]
pub struct MessageStart {
    /// 消息ID
    pub id: String,
    /// 消息类型
    #[serde(rename = "type")]
    pub message_type: String,
    /// 角色
    pub role: ApiRole,
    /// 模型
    pub model: String,
    /// 使用统计
    pub usage: ApiUsage,
}

/// 内容块开始事件
#[derive(Debug, Clone, Deserialize)]
pub struct ContentBlockStart {
    /// 内容块索引
    pub index: u32,
    /// 内容块类型
    #[serde(rename = "type")]
    pub content_block_type: ApiContentType,
}

/// 内容块增量
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlockDelta {
    /// 文本增量
    TextDelta {
        /// 文本
        text: String,
    },
    /// 工具使用增量
    ToolUseDelta {
        /// 工具使用ID
        id: String,
        /// 工具名称
        name: String,
        /// 输入（部分）
        input: Value,
    },
}

/// 消息增量
#[derive(Debug, Clone, Deserialize)]
pub struct MessageDelta {
    /// 停止原因
    pub stop_reason: Option<String>,
    /// 停止序列
    pub stop_sequence: Option<String>,
}

/// 流式错误
#[derive(Debug, Clone, Deserialize)]
pub struct StreamError {
    /// 错误类型
    #[serde(rename = "type")]
    pub error_type: String,
    /// 错误消息
    pub message: String,
}

/// 工具调用
#[derive(Debug, Clone)]
pub struct ToolCall {
    /// 工具使用ID
    pub id: String,
    /// 工具名称
    pub name: String,
    /// 输入参数
    pub input: Value,
    /// 工具定义
    pub tool: Option<ApiTool>,
}

/// 工具结果
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// 工具使用ID
    pub tool_use_id: String,
    /// 结果内容
    pub content: Value,
    /// 是否错误
    pub is_error: bool,
}

/// 流式响应处理器
pub type StreamHandler = Box<dyn FnMut(StreamEvent) -> anyhow::Result<()> + Send>;

/// 简化API请求构建器
#[derive(Debug, Clone, Default)]
pub struct ApiRequestBuilder {
    request: ApiRequest,
}

impl ApiRequestBuilder {
    /// 创建新的请求构建器
    pub fn new(model: ApiModel) -> Self {
        Self {
            request: ApiRequest {
                model,
                messages: Vec::new(),
                system: None,
                max_tokens: Some(1024),
                tools: None,
                tool_choice: None,
                thinking: None,
                stream: None,
                temperature: None,
                top_p: None,
                top_k: None,
                stop_sequences: None,
                metadata: None,
                betas: None,
            },
        }
    }

    /// 添加消息
    pub fn add_message(mut self, role: ApiRole, content: impl Into<MessageContent>) -> Self {
        self.request.messages.push(ApiMessage {
            role,
            content: content.into(),
        });
        self
    }

    /// 设置系统提示
    pub fn system(mut self, system: impl Into<String>) -> Self {
        self.request.system = Some(system.into());
        self
    }

    /// 设置最大令牌数
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_tokens = Some(max_tokens);
        self
    }

    /// 添加工具
    pub fn add_tool(mut self, tool: ApiTool) -> Self {
        if self.request.tools.is_none() {
            self.request.tools = Some(Vec::new());
        }
        if let Some(ref mut tools) = self.request.tools {
            tools.push(tool);
        }
        self
    }

    /// 设置工具选择
    pub fn tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.request.tool_choice = Some(tool_choice);
        self
    }

    /// 启用流式响应
    pub fn stream(mut self, stream: bool) -> Self {
        self.request.stream = Some(stream);
        self
    }

    /// 设置温度
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.request.temperature = Some(temperature);
        self
    }

    /// 设置Beta功能头
    pub fn add_beta(mut self, beta: impl Into<String>) -> Self {
        if self.request.betas.is_none() {
            self.request.betas = Some(Vec::new());
        }
        if let Some(ref mut betas) = self.request.betas {
            betas.push(beta.into());
        }
        self
    }

    /// 构建请求
    pub fn build(self) -> ApiRequest {
        self.request
    }
}

impl From<String> for MessageContent {
    fn from(text: String) -> Self {
        MessageContent::Text(text)
    }
}

impl From<&str> for MessageContent {
    fn from(text: &str) -> Self {
        MessageContent::Text(text.to_string())
    }
}

impl From<Vec<ApiContentBlock>> for MessageContent {
    fn from(blocks: Vec<ApiContentBlock>) -> Self {
        MessageContent::Blocks(blocks)
    }
}