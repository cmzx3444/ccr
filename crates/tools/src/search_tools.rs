//! 代码搜索工具

use crate::base::{Tool, ToolBuilder};
use crate::error::Result;
use crate::types::{ToolCategory, ToolPermissionLevel, ToolMetadata, ToolResult, ToolUseContext};
use async_trait::async_trait;
use serde_json::Value;
use std::path::Path;

/// 文件模式匹配工具
pub struct GlobTool;

#[async_trait]
impl Tool for GlobTool {
    fn metadata(&self) -> ToolMetadata {
        ToolBuilder::new("Glob", "Search for files using glob patterns")
            .category(ToolCategory::CodeSearch)
            .permission_level(ToolPermissionLevel::Standard)
            .aliases(vec!["glob".to_string()])
            .read_only()
            .build_metadata()
    }

    async fn execute(
        &self,
        input: Value,
        context: ToolUseContext,
    ) -> Result<ToolResult<Value>> {
        // 解析输入参数
        let pattern = input.get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::error::ToolError::ValidationError("Missing pattern".to_string()))?;

        // 构建完整路径（相对于当前工作目录）
        let base_path = context.cwd.to_string_lossy().to_string();

        // 使用 glob 库搜索文件
        let full_pattern = if Path::new(pattern).is_absolute() {
            pattern.to_string()
        } else {
            format!("{}/{}", base_path, pattern)
        };

        let mut matches = Vec::new();
        for entry in glob::glob(&full_pattern)
            .map_err(|e| crate::error::ToolError::Other(format!("Glob pattern error: {}", e)))?
        {
            match entry {
                Ok(path) => {
                    matches.push(path.to_string_lossy().to_string());
                }
                Err(e) => {
                    // 记录错误但继续处理其他文件
                    tracing::warn!("Glob entry error: {}", e);
                }
            }
        }

        // 返回结果
        Ok(ToolResult::success(Value::Array(
            matches.into_iter().map(Value::String).collect()
        )))
    }
}

/// 内容搜索工具
pub struct GrepTool;

#[async_trait]
impl Tool for GrepTool {
    fn metadata(&self) -> ToolMetadata {
        ToolBuilder::new("Grep", "Search for text patterns in files")
            .category(ToolCategory::CodeSearch)
            .permission_level(ToolPermissionLevel::Standard)
            .aliases(vec!["grep".to_string()])
            .read_only()
            .build_metadata()
    }

    async fn execute(
        &self,
        input: Value,
        context: ToolUseContext,
    ) -> Result<ToolResult<Value>> {
        // 解析输入参数
        let pattern = input.get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::error::ToolError::ValidationError("Missing pattern".to_string()))?;

        let path = input.get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(".");

        // 构建完整路径
        let base_path = context.cwd.to_string_lossy().to_string();
        let full_path = if Path::new(path).is_absolute() {
            path.to_string()
        } else {
            format!("{}/{}", base_path, path)
        };

        // 使用 ignore 库进行递归搜索
        let mut matches = Vec::new();
        let mut builder = ignore::WalkBuilder::new(&full_path);
        builder.hidden(false);
        builder.git_ignore(false);

        for result in builder.build() {
            match result {
                Ok(entry) => {
                    if let Some(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                                for (line_num, line) in content.lines().enumerate() {
                                    if line.contains(pattern) {
                                        matches.push(Value::Object(serde_json::Map::from_iter(vec![
                                            ("file".to_string(), Value::String(entry.path().to_string_lossy().to_string())),
                                            ("line".to_string(), Value::Number((line_num + 1).into())),
                                            ("content".to_string(), Value::String(line.to_string())),
                                        ])));
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Walk error: {}", e);
                }
            }
        }

        // 返回结果
        Ok(ToolResult::success(Value::Array(matches)))
    }
}