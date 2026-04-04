//! 工具类型系统
//!
//! 这个模块定义了工具系统的核心类型，对应 TypeScript 的 Tool.ts
//! 并提供了与API工具调用的集成类型

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationResult {
    /// 验证通过
    Valid,
    /// 验证失败
    Invalid {
        /// 错误消息
        message: String,
        /// 错误代码
        error_code: u32,
    },
}

impl ValidationResult {
    /// 创建有效的验证结果
    pub fn valid() -> Self {
        Self::Valid
    }

    /// 创建无效的验证结果
    pub fn invalid(message: impl Into<String>, error_code: u32) -> Self {
        Self::Invalid {
            message: message.into(),
            error_code,
        }
    }

    /// 检查是否有效
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid)
    }
}

/// 权限模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionMode {
    /// 默认模式
    Default,
    /// 绕过模式
    Bypass,
    /// 计划模式
    Plan,
}

/// 权限行为
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionBehavior {
    /// 允许
    Allow,
    /// 拒绝
    Deny,
    /// 询问用户
    Ask,
}

/// 权限结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionResult {
    /// 权限行为
    pub behavior: PermissionBehavior,
    /// 更新后的输入（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_input: Option<serde_json::Value>,
    /// 拒绝原因（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denial_reason: Option<String>,
}

impl PermissionResult {
    /// 创建允许结果
    pub fn allow() -> Self {
        Self {
            behavior: PermissionBehavior::Allow,
            updated_input: None,
            denial_reason: None,
        }
    }

    /// 创建允许结果（带更新的输入）
    pub fn allow_with_input(input: serde_json::Value) -> Self {
        Self {
            behavior: PermissionBehavior::Allow,
            updated_input: Some(input),
            denial_reason: None,
        }
    }

    /// 创建拒绝结果
    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            behavior: PermissionBehavior::Deny,
            updated_input: None,
            denial_reason: Some(reason.into()),
        }
    }

    /// 创建询问结果
    pub fn ask() -> Self {
        Self {
            behavior: PermissionBehavior::Ask,
            updated_input: None,
            denial_reason: None,
        }
    }
}

/// 工具权限规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPermissionRule {
    /// 规则名称
    pub name: String,
    /// 规则内容（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// 按来源分组的工具权限规则
pub type ToolPermissionRulesBySource = HashMap<String, Vec<ToolPermissionRule>>;

/// 工具权限上下文
#[derive(Debug, Clone)]
pub struct ToolPermissionContext {
    /// 权限模式
    pub mode: PermissionMode,
    /// 额外的工作目录
    pub additional_working_directories: HashMap<String, String>,
    /// 总是允许规则
    pub always_allow_rules: ToolPermissionRulesBySource,
    /// 总是拒绝规则
    pub always_deny_rules: ToolPermissionRulesBySource,
    /// 总是询问规则
    pub always_ask_rules: ToolPermissionRulesBySource,
    /// 是否可用绕过权限模式
    pub is_bypass_permissions_mode_available: bool,
}

impl Default for ToolPermissionContext {
    fn default() -> Self {
        Self {
            mode: PermissionMode::Default,
            additional_working_directories: HashMap::new(),
            always_allow_rules: HashMap::new(),
            always_deny_rules: HashMap::new(),
            always_ask_rules: HashMap::new(),
            is_bypass_permissions_mode_available: false,
        }
    }
}

impl ToolPermissionContext {
    /// 创建空的权限上下文
    pub fn empty() -> Self {
        Self::default()
    }
}

/// 工具输入 JSON Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputSchema {
    /// Schema 类型
    #[serde(rename = "type")]
    pub schema_type: String,
    /// 属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Map<String, serde_json::Value>>,
    /// 必需字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

impl Default for ToolInputSchema {
    fn default() -> Self {
        Self {
            schema_type: "object".to_string(),
            properties: None,
            required: None,
        }
    }
}

/// 工具结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult<T = serde_json::Value> {
    /// 结果数据
    pub data: T,
    /// 是否应该查询模型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_query: Option<bool>,
    /// 错误信息（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ToolResult<T> {
    /// 创建成功结果
    pub fn success(data: T) -> Self {
        Self {
            data,
            should_query: None,
            error: None,
        }
    }

    /// 创建错误结果
    pub fn error(error: impl Into<String>) -> Self
    where
        T: Default,
    {
        Self {
            data: T::default(),
            should_query: None,
            error: Some(error.into()),
        }
    }
}

/// 工具类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCategory {
    /// 文件操作
    FileOperation,
    /// 代码搜索
    CodeSearch,
    /// 命令执行
    CommandExecution,
    /// 代理系统
    AgentSystem,
    /// 任务管理
    TaskManagement,
    /// 协作
    Collaboration,
    /// 其他
    Other,
}

/// 工具权限级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolPermissionLevel {
    /// 只读
    ReadOnly,
    /// 标准
    Standard,
    /// 危险
    Dangerous,
    /// 高级
    Advanced,
}

/// 工具元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    /// 工具名称
    pub name: String,
    /// 工具描述
    pub description: String,
    /// 工具类别
    pub category: ToolCategory,
    /// 权限级别
    pub permission_level: ToolPermissionLevel,
    /// 工具别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// 是否只读
    pub is_read_only: bool,
    /// 是否破坏性
    pub is_destructive: bool,
    /// 是否启用
    pub is_enabled: bool,
    /// 是否为 MCP 工具
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_mcp: Option<bool>,
    /// 输入 Schema
    pub input_schema: ToolInputSchema,
}

/// API工具定义（用于Claude API）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToolDefinition {
    /// 工具名称
    pub name: String,
    /// 工具描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 输入模式（JSON Schema）
    pub input_schema: Value,
}

impl ApiToolDefinition {
    /// 从工具元数据创建API工具定义
    pub fn from_metadata(metadata: &ToolMetadata) -> Self {
        Self {
            name: metadata.name.clone(),
            description: Some(metadata.description.clone()),
            input_schema: serde_json::to_value(&metadata.input_schema).unwrap_or_default(),
        }
    }
}

/// API工具调用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToolCall {
    /// 工具使用ID
    pub id: String,
    /// 工具名称
    pub name: String,
    /// 输入参数
    pub input: Value,
    /// 对应的工具定义
    #[serde(skip)]
    pub tool_definition: Option<ApiToolDefinition>,
}

impl ApiToolCall {
    /// 创建新的API工具调用
    pub fn new(id: impl Into<String>, name: impl Into<String>, input: Value) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            input,
            tool_definition: None,
        }
    }

    /// 设置工具定义
    pub fn with_tool_definition(mut self, definition: ApiToolDefinition) -> Self {
        self.tool_definition = Some(definition);
        self
    }
}

/// API工具结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToolResult {
    /// 工具使用ID
    pub tool_use_id: String,
    /// 结果内容
    pub content: Value,
    /// 是否错误
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

impl ApiToolResult {
    /// 创建成功的工具结果
    pub fn success(tool_use_id: impl Into<String>, content: impl Into<Value>) -> Self {
        Self {
            tool_use_id: tool_use_id.into(),
            content: content.into(),
            is_error: Some(false),
        }
    }

    /// 创建失败的工具结果
    pub fn error(tool_use_id: impl Into<String>, error_message: impl Into<String>) -> Self {
        Self {
            tool_use_id: tool_use_id.into(),
            content: Value::String(error_message.into()),
            is_error: Some(true),
        }
    }
}

/// 工具调用上下文
#[derive(Debug, Clone)]
pub struct ToolUseContext {
    /// 当前工作目录
    pub cwd: std::path::PathBuf,
    /// 是否非交互会话
    pub is_non_interactive_session: bool,
    /// 权限上下文
    pub permission_context: ToolPermissionContext,
}

impl ToolUseContext {
    /// 创建新的工具使用上下文
    pub fn new(cwd: std::path::PathBuf) -> Self {
        Self {
            cwd,
            is_non_interactive_session: false,
            permission_context: ToolPermissionContext::default(),
        }
    }
}

/// 工具调用响应
#[derive(Debug, Clone)]
pub enum ToolCallResponse {
    /// 直接工具结果
    Direct(ToolResult<Value>),
    /// 需要API工具调用
    ApiToolCall(ApiToolCall),
    /// 需要用户交互
    UserInteraction {
        /// 交互类型
        interaction_type: String,
        /// 交互数据
        data: Value,
    },
}

/// 工具执行选项
#[derive(Debug, Clone)]
pub struct ToolExecutionOptions {
    /// 是否启用API工具调用
    pub enable_api_tool_use: bool,
    /// 最大重试次数
    pub max_retries: u32,
    /// 超时时间（秒）
    pub timeout_seconds: Option<u64>,
    /// 是否验证输入
    pub validate_input: bool,
}

impl Default for ToolExecutionOptions {
    fn default() -> Self {
        Self {
            enable_api_tool_use: false,
            max_retries: 3,
            timeout_seconds: Some(30),
            validate_input: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::valid();
        assert!(valid.is_valid());

        let invalid = ValidationResult::invalid("Test error", 100);
        assert!(!invalid.is_valid());
    }

    #[test]
    fn test_permission_result() {
        let allow = PermissionResult::allow();
        assert_eq!(allow.behavior, PermissionBehavior::Allow);

        let deny = PermissionResult::deny("Test denial");
        assert_eq!(deny.behavior, PermissionBehavior::Deny);

        let ask = PermissionResult::ask();
        assert_eq!(ask.behavior, PermissionBehavior::Ask);
    }

    #[test]
    fn test_tool_result() {
        let success = ToolResult::success("test data");
        assert!(success.error.is_none());

        let error: ToolResult<String> = ToolResult::error("Test error");
        assert!(error.error.is_some());
    }

    #[test]
    fn test_api_tool_definition() {
        let metadata = ToolMetadata {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            category: ToolCategory::Other,
            permission_level: ToolPermissionLevel::Standard,
            aliases: None,
            is_read_only: false,
            is_destructive: false,
            is_enabled: true,
            is_mcp: None,
            input_schema: ToolInputSchema::default(),
        };

        let api_def = ApiToolDefinition::from_metadata(&metadata);
        assert_eq!(api_def.name, "test_tool");
        assert_eq!(api_def.description, Some("A test tool".to_string()));
    }
}