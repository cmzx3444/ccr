//! VSCode 编辑器集成模块
//!
//! 实现 Visual Studio Code 编辑器集成

use super::common::*;
use crate::error::Result;
use std::collections::HashMap;
use std::path::PathBuf;

/// VSCode 集成
pub struct VSCodeIntegration {
    config: EditorConfig,
    vs_code_path: Option<PathBuf>,
    extensions: HashMap<String, ExtensionState>,
}

impl VSCodeIntegration {
    /// 创建新的 VSCode 集成
    pub fn new() -> Self {
        Self {
            config: EditorConfig::default(),
            vs_code_path: Self::detect_vs_code_path(),
            extensions: HashMap::new(),
        }
    }

    /// 检测 VSCode 安装路径
    fn detect_vs_code_path() -> Option<PathBuf> {
        // 检查常见安装路径
        let possible_paths = if cfg!(target_os = "windows") {
            vec![
                PathBuf::from(r"C:\Program Files\Microsoft VS Code\bin\code.cmd"),
                PathBuf::from(r"C:\Program Files (x86)\Microsoft VS Code\bin\code.cmd"),
                PathBuf::from(r"%USERPROFILE%\AppData\Local\Programs\Microsoft VS Code\bin\code.cmd"),
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                PathBuf::from("/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code"),
                PathBuf::from("/usr/local/bin/code"),
            ]
        } else {
            vec![
                PathBuf::from("/usr/bin/code"),
                PathBuf::from("/usr/local/bin/code"),
                PathBuf::from("/snap/bin/code"),
            ]
        };

        for path in possible_paths {
            if path.exists() {
                return Some(path);
            }
        }

        // 检查环境变量 PATH
        if let Ok(paths) = std::env::var("PATH") {
            for path in paths.split(std::path::MAIN_SEPARATOR) {
                let code_path = PathBuf::from(path).join(if cfg!(windows) { "code.cmd" } else { "code" });
                if code_path.exists() {
                    return Some(code_path);
                }
            }
        }

        None
    }

    /// 获取 VSCode 版本
    async fn get_version(&self) -> Result<Option<String>> {
        if let Some(vs_code_path) = &self.vs_code_path {
            let output = std::process::Command::new(vs_code_path)
                .arg("--version")
                .output();

            match output {
                Ok(output) if output.status.success() => {
                    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    Ok(Some(version))
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    /// 获取安装的扩展
    async fn load_extensions(&mut self) -> Result<()> {
        if let Some(vs_code_path) = &self.vs_code_path {
            let output = std::process::Command::new(vs_code_path)
                .arg("--list-extensions")
                .arg("--show-versions")
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout);
                    for line in text.lines() {
                        let parts: Vec<&str> = line.split('@').collect();
                        if parts.len() >= 2 {
                            let id = parts[0].trim().to_string();
                            let version = parts[1].trim().to_string();

                            self.extensions.insert(id.clone(), ExtensionState {
                                id: id.clone(),
                                name: id.clone(), // 简化：使用ID作为名称
                                enabled: true,
                                version,
                                status: ExtensionStatus::Active,
                            });
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl EditorIntegration for VSCodeIntegration {
    async fn init(&mut self, config: &EditorConfig) -> Result<()> {
        self.config = config.clone();
        self.load_extensions().await?;
        tracing::info!("VSCode integration initialized");
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
            EditorFeature::TerminalIntegration,
            EditorFeature::StateQuery,
            EditorFeature::Configuration,
            EditorFeature::PluginSystem,
        ]
    }

    fn supports_command(&self, command: &str) -> bool {
        matches!(command,
            "openFile" | "openFolder" | "runCommand" |
            "getState" | "getExtensions" | "installExtension"
        )
    }

    async fn execute_command(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        match command {
            "openFile" => {
                let file_path = args["filePath"].as_str()
                    .ok_or_else(|| crate::error::ClaudeError::Editor("filePath is required".to_string()))?;

                if let Some(vs_code_path) = &self.vs_code_path {
                    let status = std::process::Command::new(vs_code_path)
                        .arg(file_path)
                        .status();

                    match status {
                        Ok(_) => Ok(serde_json::json!({ "success": true, "file": file_path })),
                        Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                    }
                } else {
                    Ok(serde_json::json!({ "success": false, "error": "VSCode not found" }))
                }
            }
            "openFolder" => {
                let folder_path = args["folderPath"].as_str()
                    .ok_or_else(|| crate::error::ClaudeError::Editor("folderPath is required".to_string()))?;

                if let Some(vs_code_path) = &self.vs_code_path {
                    let status = std::process::Command::new(vs_code_path)
                        .arg(folder_path)
                        .status();

                    match status {
                        Ok(_) => Ok(serde_json::json!({ "success": true, "folder": folder_path })),
                        Err(e) => Ok(serde_json::json!({ "success": false, "error": e.to_string() })),
                    }
                } else {
                    Ok(serde_json::json!({ "success": false, "error": "VSCode not found" }))
                }
            }
            "getExtensions" => {
                let extensions: Vec<_> = self.extensions.values()
                    .map(|ext| serde_json::json!({
                        "id": ext.id,
                        "name": ext.name,
                        "version": ext.version,
                        "enabled": ext.enabled,
                    }))
                    .collect();

                Ok(serde_json::json!({
                    "success": true,
                    "extensions": extensions,
                    "count": extensions.len()
                }))
            }
            "getState" => {
                let version = self.get_version().await?;
                let extensions: Vec<_> = self.extensions.values()
                    .map(|ext| serde_json::json!({
                        "id": ext.id,
                        "name": ext.name,
                        "version": ext.version,
                        "enabled": ext.enabled,
                    }))
                    .collect();

                Ok(serde_json::json!({
                    "success": true,
                    "editor": "VSCode",
                    "version": version,
                    "extensions": extensions,
                    "path": self.vs_code_path.as_ref().map(|p| p.display().to_string())
                }))
            }
            _ => {
                Err(crate::error::ClaudeError::Editor(format!("Unknown command: {}", command)))
            }
        }
    }

    async fn get_state(&self) -> Result<EditorState> {
        let version = self.get_version().await?;

        Ok(EditorState {
            is_running: true,
            open_files: Vec::new(), // 需要实际实现
            active_file: None, // 需要实际实现
            cursor_position: None,
            selection_range: None,
            project_info: None,
            editor_version: version,
            extensions: self.extensions.clone(),
        })
    }

    async fn update_config(&mut self, config: &EditorConfig) -> Result<()> {
        self.config = config.clone();
        Ok(())
    }

    fn name(&self) -> &str {
        "VSCode"
    }
}

impl Default for VSCodeIntegration {
    fn default() -> Self {
        Self::new()
    }
}