//! 编辑器集成通用模块
//!
//! 定义编辑器集成的通用接口和类型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::error::Result;

/// 编辑器功能枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorFeature {
    /// 代码补全
    CodeCompletion,
    /// 语法高亮
    SyntaxHighlighting,
    /// 错误检查
    ErrorChecking,
    /// 代码导航
    CodeNavigation,
    /// 重构支持
    Refactoring,
    /// 调试支持
    Debugging,
    /// 版本控制集成
    VersionControl,
    /// 终端集成
    TerminalIntegration,
    /// 状态查询
    StateQuery,
    /// 配置管理
    Configuration,
    /// 插件系统
    PluginSystem,
}

impl EditorFeature {
    /// 获取功能描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::CodeCompletion => "Code completion and IntelliSense",
            Self::SyntaxHighlighting => "Syntax highlighting and colorization",
            Self::ErrorChecking => "Error checking and diagnostics",
            Self::CodeNavigation => "Code navigation (go to definition, find references)",
            Self::Refactoring => "Code refactoring support",
            Self::Debugging => "Debugging support",
            Self::VersionControl => "Version control integration",
            Self::TerminalIntegration => "Terminal/console integration",
            Self::StateQuery => "Editor state querying",
            Self::Configuration => "Configuration management",
            Self::PluginSystem => "Plugin/extension system",
        }
    }
}

/// 编辑器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    /// 是否启用代码补全
    pub enable_code_completion: bool,
    /// 是否启用语法高亮
    pub enable_syntax_highlighting: bool,
    /// 是否启用错误检查
    pub enable_error_checking: bool,
    /// 自动保存设置
    pub auto_save: AutoSaveConfig,
    /// 格式化设置
    pub formatting: FormattingConfig,
    /// 键盘快捷键映射
    pub keybindings: HashMap<String, String>,
    /// 主题设置
    pub theme: ThemeConfig,
    /// 扩展配置
    pub extensions: HashMap<String, serde_json::Value>,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            enable_code_completion: true,
            enable_syntax_highlighting: true,
            enable_error_checking: true,
            auto_save: AutoSaveConfig::default(),
            formatting: FormattingConfig::default(),
            keybindings: HashMap::new(),
            theme: ThemeConfig::default(),
            extensions: HashMap::new(),
        }
    }
}

/// 自动保存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSaveConfig {
    /// 是否启用自动保存
    pub enabled: bool,
    /// 自动保存延迟（毫秒）
    pub delay_ms: u64,
    /// 保存文件类型
    pub file_types: Vec<String>,
}

impl Default for AutoSaveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            delay_ms: 1000,
            file_types: vec!["*.rs".to_string(), "*.toml".to_string(), "*.json".to_string()],
        }
    }
}

/// 格式化配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormattingConfig {
    /// 是否启用自动格式化
    pub enabled: bool,
    /// 格式化工具
    pub formatter: String,
    /// 格式化选项
    pub options: HashMap<String, String>,
}

impl Default for FormattingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            formatter: "rustfmt".to_string(),
            options: HashMap::from_iter([
                ("max_width".to_string(), "100".to_string()),
                ("tab_spaces".to_string(), "4".to_string()),
            ]),
        }
    }
}

/// 主题配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// 主题名称
    pub name: String,
    /// 是否暗色模式
    pub dark_mode: bool,
    /// 颜色方案
    pub color_scheme: String,
    /// 字体设置
    pub font: FontConfig,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            dark_mode: false,
            color_scheme: "Default".to_string(),
            font: FontConfig::default(),
        }
    }
}

/// 字体配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    /// 字体家族
    pub family: String,
    /// 字体大小
    pub size: u32,
    /// 行高
    pub line_height: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            family: "Monaco, Consolas, 'Courier New', monospace".to_string(),
            size: 14,
            line_height: 1.5,
        }
    }
}

/// 编辑器状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorState {
    /// 是否运行中
    pub is_running: bool,
    /// 当前打开的文件
    pub open_files: Vec<PathBuf>,
    /// 当前活动文件
    pub active_file: Option<PathBuf>,
    /// 光标位置
    pub cursor_position: Option<CursorPosition>,
    /// 选择范围
    pub selection_range: Option<SelectionRange>,
    /// 项目信息
    pub project_info: Option<ProjectInfo>,
    /// 编辑器版本
    pub editor_version: Option<String>,
    /// 扩展状态
    pub extensions: HashMap<String, ExtensionState>,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            is_running: true,
            open_files: Vec::new(),
            active_file: None,
            cursor_position: None,
            selection_range: None,
            project_info: None,
            editor_version: None,
            extensions: HashMap::new(),
        }
    }
}

/// 光标位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPosition {
    /// 行号（0-based）
    pub line: u32,
    /// 列号（0-based）
    pub column: u32,
    /// 文件路径
    pub file_path: PathBuf,
}

/// 选择范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionRange {
    /// 起始位置
    pub start: CursorPosition,
    /// 结束位置
    pub end: CursorPosition,
}

/// 项目信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    /// 项目名称
    pub name: String,
    /// 项目路径
    pub path: PathBuf,
    /// 项目类型
    pub project_type: String,
    /// 项目文件
    pub project_files: Vec<PathBuf>,
}

/// 扩展状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionState {
    /// 扩展ID
    pub id: String,
    /// 扩展名称
    pub name: String,
    /// 是否启用
    pub enabled: bool,
    /// 版本
    pub version: String,
    /// 状态信息
    pub status: ExtensionStatus,
}

/// 扩展状态枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionStatus {
    /// 活动状态
    Active,
    /// 加载中
    Loading,
    /// 错误状态
    Error(String),
    /// 禁用状态
    Disabled,
    /// 未知状态
    Unknown,
}

/// 编辑器集成 trait
#[async_trait::async_trait]
pub trait EditorIntegration: Send + Sync {
    /// 初始化集成
    async fn init(&mut self, config: &EditorConfig) -> Result<()>;

    /// 获取支持的编辑器功能
    fn supported_features(&self) -> Vec<EditorFeature>;

    /// 检查是否支持特定功能
    fn supports_feature(&self, feature: EditorFeature) -> bool {
        self.supported_features().contains(&feature)
    }

    /// 检查是否支持特定命令
    fn supports_command(&self, command: &str) -> bool;

    /// 执行编辑器命令
    async fn execute_command(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value>;

    /// 获取编辑器状态
    async fn get_state(&self) -> Result<EditorState>;

    /// 更新编辑器配置
    async fn update_config(&mut self, config: &EditorConfig) -> Result<()>;

    /// 获取集成名称
    fn name(&self) -> &str;
}

/// 编辑器命令结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorCommandResult {
    /// 是否成功
    pub success: bool,
    /// 结果数据
    pub data: Option<serde_json::Value>,
    /// 错误信息
    pub error: Option<String>,
}

impl EditorCommandResult {
    /// 创建成功结果
    pub fn success(data: serde_json::Value) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// 创建失败结果
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}