//! 技能类型定义
//!
//! 定义技能系统的核心数据类型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 技能元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    /// 技能名称
    pub name: String,

    /// 技能描述
    pub description: String,

    /// 技能版本
    pub version: Option<String>,

    /// 技能作者
    pub author: Option<String>,

    /// 技能分类
    pub category: SkillCategory,

    /// 技能标签
    pub tags: Vec<String>,

    /// 输入模式
    pub input_schema: Option<SkillInputSchema>,

    /// 输出模式
    pub output_schema: Option<serde_json::Value>,

    /// 所需权限
    pub required_permissions: Vec<String>,

    /// 是否内置技能
    pub is_builtin: bool,

    /// 技能文件路径
    pub file_path: Option<PathBuf>,

    /// 技能配置
    pub config: HashMap<String, serde_json::Value>,
}

/// 技能分类
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillCategory {
    /// 代码相关技能
    Code,

    /// 文件操作技能
    File,

    /// Git操作技能
    Git,

    /// 构建和测试技能
    Build,

    /// 部署技能
    Deployment,

    /// 调试技能
    Debug,

    /// 文档技能
    Documentation,

    /// 工具集成技能
    ToolIntegration,

    /// 其他技能
    Other,
}

impl Default for SkillCategory {
    fn default() -> Self {
        Self::Other
    }
}

/// 技能输入模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInputSchema {
    /// 模式类型
    pub schema_type: String,

    /// 属性定义
    pub properties: Option<serde_json::Map<String, serde_json::Value>>,

    /// 必需属性
    pub required: Option<Vec<String>>,
}

/// 技能执行上下文
#[derive(Debug, Clone)]
pub struct SkillContext {
    /// 当前工作目录
    pub cwd: PathBuf,

    /// 项目根目录
    pub project_root: PathBuf,

    /// 环境变量
    pub env: HashMap<String, String>,

    /// 配置数据
    pub config: HashMap<String, serde_json::Value>,

    /// 会话状态
    pub session_state: HashMap<String, serde_json::Value>,
}

impl SkillContext {
    /// 创建新的技能上下文
    pub fn new(cwd: PathBuf, project_root: PathBuf) -> Self {
        Self {
            cwd,
            project_root,
            env: std::env::vars().collect(),
            config: HashMap::new(),
            session_state: HashMap::new(),
        }
    }

    /// 获取环境变量
    pub fn get_env(&self, key: &str) -> Option<&String> {
        self.env.get(key)
    }

    /// 设置配置值
    pub fn set_config(&mut self, key: &str, value: serde_json::Value) {
        self.config.insert(key.to_string(), value);
    }

    /// 获取配置值
    pub fn get_config(&self, key: &str) -> Option<&serde_json::Value> {
        self.config.get(key)
    }
}

/// 技能结果
#[derive(Debug, Clone, Serialize)]
pub struct SkillResult {
    /// 是否成功
    pub success: bool,

    /// 输出数据
    pub output: serde_json::Value,

    /// 错误信息（如果有）
    pub error: Option<String>,

    /// 执行耗时（毫秒）
    pub duration_ms: u64,

    /// 技能名称
    pub skill_name: String,

    /// 技能版本
    pub skill_version: Option<String>,
}

impl SkillResult {
    /// 创建成功结果
    pub fn success(skill_name: String, output: serde_json::Value, duration_ms: u64) -> Self {
        Self {
            success: true,
            output,
            error: None,
            duration_ms,
            skill_name,
            skill_version: None,
        }
    }

    /// 创建失败结果
    pub fn error(skill_name: String, error: String, duration_ms: u64) -> Self {
        Self {
            success: false,
            output: serde_json::json!({}),
            error: Some(error),
            duration_ms,
            skill_name,
            skill_version: None,
        }
    }
}

/// 技能 trait
#[async_trait::async_trait]
pub trait Skill: Send + Sync {
    /// 获取技能元数据
    fn metadata(&self) -> SkillMetadata;

    /// 执行技能
    async fn execute(&self, args: Option<&str>, context: SkillContext) -> Result<SkillResult, crate::error::ClaudeError>;
}

/// 技能定义宏
#[macro_export]
macro_rules! define_skill {
    ($name:expr, $desc:expr, $executor:expr) => {
        pub struct SkillImpl;

        #[async_trait::async_trait]
        impl $crate::skills::types::Skill for SkillImpl {
            fn metadata(&self) -> $crate::skills::types::SkillMetadata {
                $crate::skills::types::SkillMetadata {
                    name: $name.to_string(),
                    description: $desc.to_string(),
                    version: None,
                    author: None,
                    category: $crate::skills::types::SkillCategory::Other,
                    tags: Vec::new(),
                    input_schema: None,
                    output_schema: None,
                    required_permissions: Vec::new(),
                    is_builtin: true,
                    file_path: None,
                    config: std::collections::HashMap::new(),
                }
            }

            async fn execute(
                &self,
                args: Option<&str>,
                context: $crate::skills::types::SkillContext,
            ) -> Result<$crate::skills::types::SkillResult, crate::error::ClaudeError> {
                $executor(args, context).await
            }
        }
    };
}