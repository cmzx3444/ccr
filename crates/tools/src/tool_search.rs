//! 工具搜索工具

use crate::base::{Tool, ToolBuilder};
use crate::error::Result;
use crate::types::{ToolCategory, ToolPermissionLevel, ToolMetadata, ToolResult, ToolUseContext};
use async_trait::async_trait;
use serde_json::Value;

/// 工具搜索工具
pub struct ToolSearchTool;

#[async_trait]
impl Tool for ToolSearchTool {
    fn metadata(&self) -> ToolMetadata {
        ToolBuilder::new("ToolSearch", "Search for available tools")
            .category(ToolCategory::CodeSearch)
            .permission_level(ToolPermissionLevel::Standard)
            .aliases(vec!["toolsearch".to_string(), "tools".to_string()])
            .read_only()
            .build_metadata()
    }

    async fn execute(
        &self,
        input: Value,
        _context: ToolUseContext,
    ) -> Result<ToolResult<Value>> {
        // 解析输入参数
        let query = input.get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // 获取所有工具名称（硬编码列表，实际应该从工具注册表获取）
        let all_tools = vec![
            "Read", "Edit", "Write", "Glob", "Grep", "Bash", "PowerShell",
            "WebFetch", "WebSearch", "Skill", "SendMessage", "TaskCreate",
            "EnterPlanMode", "ExitPlanMode", "EnterWorktree", "AskUserQuestion",
            "LSP", "Sleep", "CronCreate", "TeamCreate", "ToolSearch"
        ];

        // 过滤工具
        let filtered_tools: Vec<String> = if query.is_empty() {
            all_tools.into_iter().map(|s| s.to_string()).collect()
        } else {
            all_tools.into_iter()
                .filter(|tool| tool.to_lowercase().contains(&query.to_lowercase()))
                .map(|s| s.to_string())
                .collect()
        };

        // 返回结果
        Ok(ToolResult::success(Value::Array(
            filtered_tools.into_iter().map(Value::String).collect()
        )))
    }
}