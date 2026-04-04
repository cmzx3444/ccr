//! 时间工具

use crate::base::{Tool, ToolBuilder};
use crate::error::Result;
use crate::types::{ToolCategory, ToolPermissionLevel, ToolMetadata, ToolResult, ToolUseContext};
use async_trait::async_trait;
use serde_json::Value;
use std::time::Duration;

/// 休眠工具
pub struct SleepTool;

#[async_trait]
impl Tool for SleepTool {
    fn metadata(&self) -> ToolMetadata {
        ToolBuilder::new("Sleep", "Sleep for a specified duration")
            .category(ToolCategory::Other)
            .permission_level(ToolPermissionLevel::Standard)
            .aliases(vec!["sleep".to_string()])
            .read_only()
            .build_metadata()
    }

    async fn execute(
        &self,
        input: Value,
        _context: ToolUseContext,
    ) -> Result<ToolResult<Value>> {
        // 解析输入参数
        let duration_ms = input.get("duration_ms")
            .and_then(|v| v.as_u64())
            .or_else(|| input.get("duration_seconds").and_then(|v| v.as_u64()).map(|s| s * 1000))
            .ok_or_else(|| crate::error::ToolError::ValidationError("Missing duration parameter".to_string()))?;

        // 休眠
        tokio::time::sleep(Duration::from_millis(duration_ms)).await;

        // 返回结果
        Ok(ToolResult::success(Value::String(format!("Slept for {} ms", duration_ms))))
    }
}