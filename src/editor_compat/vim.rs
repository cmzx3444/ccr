//! Vim/Neovim 编辑器集成模块
//!
//! 实现 Vim 和 Neovim 编辑器集成

use super::common::*;
use crate::error::Result;

/// Vim/Neovim 集成
pub struct VimIntegration {
    config: EditorConfig,
}

impl VimIntegration {
    /// 创建新的 Vim/Neovim 集成
    pub fn new() -> Self {
        Self {
            config: EditorConfig::default(),
        }
    }
}

#[async_trait::async_trait]
impl EditorIntegration for VimIntegration {
    async fn init(&mut self, config: &EditorConfig) -> Result<()> {
        self.config = config.clone();
        tracing::info!("Vim/Neovim integration initialized");
        Ok(())
    }

    fn supported_features(&self) -> Vec<EditorFeature> {
        vec![
            EditorFeature::SyntaxHighlighting,
            EditorFeature::CodeNavigation,
            EditorFeature::TerminalIntegration,
            EditorFeature::Configuration,
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
                    "editor": "Vim/Neovim",
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
        "Vim"
    }
}

impl Default for VimIntegration {
    fn default() -> Self {
        Self::new()
    }
}