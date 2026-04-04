//! 命令执行工具

use crate::base::{Tool, ToolBuilder};
use crate::error::Result;
use crate::types::{ToolCategory, ToolPermissionLevel, ToolMetadata, ToolResult, ToolUseContext};
use async_trait::async_trait;
use serde_json::Value;
use std::process::Command;

/// Bash 命令执行工具
pub struct BashTool;

#[async_trait]
impl Tool for BashTool {
    fn metadata(&self) -> ToolMetadata {
        ToolBuilder::new("Bash", "Execute bash commands")
            .category(ToolCategory::CommandExecution)
            .permission_level(ToolPermissionLevel::Dangerous)
            .aliases(vec!["bash".to_string(), "shell".to_string()])
            .build_metadata()
    }

    async fn execute(
        &self,
        input: Value,
        _context: ToolUseContext,
    ) -> Result<ToolResult<Value>> {
        // 解析输入参数
        let command = input.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::error::ToolError::ValidationError("Missing command".to_string()))?;

        // 执行 bash 命令
        let output = if cfg!(target_os = "windows") {
            // Windows: 使用 cmd.exe /C
            Command::new("cmd")
                .args(["/C", command])
                .output()
        } else {
            // Unix-like: 使用 bash -c
            Command::new("bash")
                .args(["-c", command])
                .output()
        }.map_err(|e| crate::error::ToolError::ExecutionError(format!("Command execution failed: {}", e)))?;

        // 构建结果
        let result = serde_json::json!({
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout).to_string(),
            "stderr": String::from_utf8_lossy(&output.stderr).to_string(),
            "success": output.status.success(),
        });

        Ok(ToolResult::success(result))
    }
}

/// PowerShell 命令执行工具
pub struct PowerShellTool;

#[async_trait]
impl Tool for PowerShellTool {
    fn metadata(&self) -> ToolMetadata {
        ToolBuilder::new("PowerShell", "Execute PowerShell commands")
            .category(ToolCategory::CommandExecution)
            .permission_level(ToolPermissionLevel::Dangerous)
            .aliases(vec!["powershell".to_string(), "ps".to_string()])
            .build_metadata()
    }

    async fn execute(
        &self,
        input: Value,
        _context: ToolUseContext,
    ) -> Result<ToolResult<Value>> {
        // 解析输入参数
        let command = input.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| crate::error::ToolError::ValidationError("Missing command".to_string()))?;

        // 执行 PowerShell 命令
        let output = Command::new("powershell")
            .args(["-Command", command])
            .output()
            .map_err(|e| crate::error::ToolError::ExecutionError(format!("PowerShell execution failed: {}", e)))?;

        // 构建结果
        let result = serde_json::json!({
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout).to_string(),
            "stderr": String::from_utf8_lossy(&output.stderr).to_string(),
            "success": output.status.success(),
        });

        Ok(ToolResult::success(result))
    }
}