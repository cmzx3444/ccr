//! JetBrains IDE 编辑器集成模块
//!
//! 实现 JetBrains IDE (IntelliJ, PyCharm, WebStorm等) 集成

use super::common::*;
use crate::error::Result;

/// JetBrains IDE 集成
pub struct JetBrainsIntegration {
    config: EditorConfig,
}

impl JetBrainsIntegration {
    /// 创建新的 JetBrains IDE 集成
    pub fn new() -> Self {
        Self {
            config: EditorConfig::default(),
        }
    }
}

#[async_trait::async_trait]
impl EditorIntegration for JetBrainsIntegration {
    async fn init(&mut self, config: &EditorConfig) -> Result<()> {
        self.config = config.clone();
        tracing::info!("JetBrains IDE integration initialized");
        Ok(())
    }

    fn supported_features(&self) -> Vec<EditorFeature> {
        vec![
            EditorFeature::CodeCompletion,
            EditorFeature::SyntaxHighlighting,
            EditorFeature::ErrorChecking,
            EditorFeature::CodeNavigation,
            EditorFeature::Refactoring,
            EditorFeature::Debugging,
            EditorFeature::VersionControl,
            EditorFeature::PluginSystem,
        ]
    }

    fn supports_command(&self, command: &str) -> bool {
        matches!(command, "getState")
    }

    async fn execute_command(&self, command: &str, _args: serde_json::Value) -> Result<serde_json::Value> {
        match command {
            "getState" => {
                Ok(serde_json::json!({
                    "success": true,
                    "editor": "JetBrains IDE",
                    "note": "Integration is a placeholder"
                }))
            }
            _ => {
                Err(crate::error::ClaudeError::Editor(format!("Unknown command: {}", command)))
            }
        }
    }

    async fn get_state(&self) -> Result<EditorState> {
        Ok(EditorState {
            is_running: true,
            editor_version: Some("Placeholder".to_string()),
            ..Default::default()
        })
    }

    async fn update_config(&mut self, config: &EditorConfig) -> Result<()> {
        self.config = config.clone();
        Ok(())
    }

    fn name(&self) -> &str {
        "JetBrains"
    }
}

impl Default for JetBrainsIntegration {
    fn default() -> Self {
        Self::new()
    }
}