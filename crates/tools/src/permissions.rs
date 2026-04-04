//! 权限系统

use crate::types::{PermissionMode, PermissionResult, ToolPermissionContext};
use serde_json::Value;

/// 权限检查器
pub struct PermissionChecker;

impl PermissionChecker {
    /// 检查工具权限
    pub fn check(
        tool_name: &str,
        input: &Value,
        context: &ToolPermissionContext,
    ) -> PermissionResult {
        // 简化实现：根据模式决定
        match context.mode {
            PermissionMode::Default => {
                // 默认模式：检查规则
                Self::check_rules(tool_name, input, context)
            }
            PermissionMode::Bypass => PermissionResult::allow(),
            PermissionMode::Plan => PermissionResult::allow(),
        }
    }

    /// 检查规则
    fn check_rules(
        tool_name: &str,
        _input: &Value,
        context: &ToolPermissionContext,
    ) -> PermissionResult {
        // 检查总是允许规则
        for (_source, rules) in &context.always_allow_rules {
            for rule in rules {
                if rule.name == tool_name {
                    return PermissionResult::allow();
                }
            }
        }

        // 检查总是拒绝规则
        for (_source, rules) in &context.always_deny_rules {
            for rule in rules {
                if rule.name == tool_name {
                    return PermissionResult::deny(format!("Tool {} is denied by rule", tool_name));
                }
            }
        }

        // 检查总是询问规则
        for (_source, rules) in &context.always_ask_rules {
            for rule in rules {
                if rule.name == tool_name {
                    return PermissionResult::ask();
                }
            }
        }

        // 默认允许
        PermissionResult::allow()
    }
}

/// 模式检查器
pub struct ModeChecker;

impl ModeChecker {
    /// 检查是否允许在当前模式下执行
    pub fn check_mode(mode: PermissionMode, _context: &ToolPermissionContext) -> bool {
        match mode {
            PermissionMode::Default => true,
            PermissionMode::Bypass => true,
            PermissionMode::Plan => true,
        }
    }
}