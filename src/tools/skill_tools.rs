//! 技能相关工具
//!
//! 实现Skill Tool等技能执行工具，集成真正的技能系统

use crate::error::Result;
use async_trait::async_trait;
use crate::skills::executor::SkillContextBuilder;
use super::base::{Tool, ToolBuilder};
use super::types::{
    ToolMetadata, ToolUseContext, ToolResult, ToolInputSchema,
    ToolCategory, ToolPermissionLevel,
};

/// Skill工具
/// 用于执行技能
pub struct SkillTool;

#[async_trait]
impl Tool for SkillTool {
    fn metadata(&self) -> ToolMetadata {
        ToolBuilder::new("Skill", "Execute a skill within the main conversation")
            .category(ToolCategory::AgentSystem)
            .permission_level(ToolPermissionLevel::Standard)
            .aliases(vec!["skill".to_string()])
            .input_schema(ToolInputSchema {
                schema_type: "object".to_string(),
                properties: Some(serde_json::Map::from_iter([
                    ("skill".to_string(), serde_json::json!({
                        "type": "string",
                        "description": "The skill name to execute"
                    })),
                    ("args".to_string(), serde_json::json!({
                        "type": "string",
                        "description": "Arguments to pass to the skill"
                    })),
                ])),
                required: Some(vec!["skill".to_string()]),
            })
            .build_metadata()
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        context: ToolUseContext,
    ) -> Result<ToolResult> {
        let skill_name = input["skill"].as_str()
            .ok_or_else(|| crate::error::ClaudeError::Tool("skill is required".to_string()))?;

        let args = input["args"].as_str();

        // 构建技能上下文
        let skill_context = SkillContextBuilder::new()
            .with_cwd(context.cwd.clone())
            .with_project_root(context.cwd.clone()) // 简化：使用cwd作为项目根目录
            .with_config("tool_context", serde_json::json!({
                "tool_name": "Skill",
                "permission_mode": context.config.permissions.mode
            }))
            .build();

        // 执行技能
        match skills::execute_skill(skill_name, args, skill_context).await {
            Ok(skill_result) => {
                let tool_result = if skill_result.success {
                    ToolResult::success(serde_json::json!({
                        "skill": skill_name,
                        "args": args.unwrap_or(""),
                        "result": skill_result.output,
                        "duration_ms": skill_result.duration_ms,
                        "success": true
                    }))
                } else {
                    ToolResult::error(
                        format!("技能执行失败: {}", skill_result.error.unwrap_or_default()),
                        serde_json::json!({
                            "skill": skill_name,
                            "args": args.unwrap_or(""),
                            "error": skill_result.error,
                            "duration_ms": skill_result.duration_ms,
                            "success": false
                        })
                    )
                };
                Ok(tool_result)
            }
            Err(e) => {
                Ok(ToolResult::error(
                    format!("技能系统错误: {}", e),
                    serde_json::json!({
                        "skill": skill_name,
                        "args": args.unwrap_or(""),
                        "error": e.to_string(),
                        "success": false
                    })
                ))
            }
        }
    }

    fn get_activity_description(&self, input: &serde_json::Value) -> Option<String> {
        input["skill"].as_str().map(|s| format!("Executing skill '{}'", s))
    }
}

/// 技能列表工具
pub struct SkillListTool;

#[async_trait]
impl Tool for SkillListTool {
    fn metadata(&self) -> ToolMetadata {
        ToolBuilder::new("SkillList", "List all available skills")
            .category(ToolCategory::AgentSystem)
            .permission_level(ToolPermissionLevel::Standard)
            .aliases(vec!["skills".to_string(), "list-skills".to_string()])
            .input_schema(ToolInputSchema {
                schema_type: "object".to_string(),
                properties: Some(serde_json::Map::from_iter([
                    ("query".to_string(), serde_json::json!({
                        "type": "string",
                        "description": "Optional search query to filter skills"
                    })),
                    ("category".to_string(), serde_json::json!({
                        "type": "string",
                        "description": "Optional category filter"
                    })),
                ])),
                required: Some(vec![]),
            })
            .build_metadata()
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        _context: ToolUseContext,
    ) -> Result<ToolResult> {
        let query = input["query"].as_str();
        let category = input["category"].as_str();

        // 初始化技能系统
        let manager = skills::init().await?;
        let registry = manager.registry();

        // 获取所有技能
        let all_skills = registry.list().await;

        // 过滤技能
        let filtered_skills: Vec<_> = all_skills.iter()
            .filter(|skill| {
                let metadata = skill.metadata();

                // 应用查询过滤
                let query_match = query.map(|q| {
                    metadata.name.to_lowercase().contains(&q.to_lowercase()) ||
                    metadata.description.to_lowercase().contains(&q.to_lowercase()) ||
                    metadata.tags.iter().any(|tag| tag.to_lowercase().contains(&q.to_lowercase()))
                }).unwrap_or(true);

                // 应用分类过滤
                let category_match = category.map(|c| {
                    format!("{:?}", metadata.category).to_lowercase() == c.to_lowercase()
                }).unwrap_or(true);

                query_match && category_match
            })
            .map(|skill| {
                let metadata = skill.metadata();
                serde_json::json!({
                    "name": metadata.name,
                    "description": metadata.description,
                    "category": format!("{:?}", metadata.category),
                    "version": metadata.version,
                    "author": metadata.author,
                    "tags": metadata.tags,
                    "builtin": metadata.is_builtin,
                    "permissions": metadata.required_permissions
                })
            })
            .collect();

        Ok(ToolResult::success(serde_json::json!({
            "skills": filtered_skills,
            "count": filtered_skills.len(),
            "query": query.unwrap_or(""),
            "category": category.unwrap_or("")
        })))
    }

    fn get_activity_description(&self, _input: &serde_json::Value) -> Option<String> {
        Some("Listing available skills".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_metadata() {
        let tool = SkillTool;
        let metadata = tool.metadata();

        assert_eq!(metadata.name, "Skill");
        assert_eq!(metadata.category, ToolCategory::AgentSystem);
    }

    #[test]
    fn test_skill_list_metadata() {
        let tool = SkillListTool;
        let metadata = tool.metadata();

        assert_eq!(metadata.name, "SkillList");
        assert_eq!(metadata.category, ToolCategory::AgentSystem);
    }
}
