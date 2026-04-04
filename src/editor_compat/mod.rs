//! 编辑器兼容层模块
//!
//! 这个模块实现了上游编辑器集成兼容层，支持：
//! - VSCode 集成
//! - JetBrains IDE 集成
//! - Vim/Neovim 集成
//! - Emacs 集成
//! - 通用 LSP 协议支持

pub mod vscode;
pub mod jetbrains;
pub mod vim;
pub mod emacs;
pub mod lsp;
pub mod common;

// 重新导出主要类型
pub use vscode::VSCodeIntegration;
pub use jetbrains::JetBrainsIntegration;
pub use vim::VimIntegration;
pub use emacs::EmacsIntegration;
pub use lsp::LspIntegration;
pub use common::{EditorIntegration, EditorFeature, EditorConfig, EditorState};

use crate::error::Result;

/// 支持的编辑器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditorType {
    /// Visual Studio Code
    VSCode,
    /// JetBrains IDE (IntelliJ, PyCharm, WebStorm等)
    JetBrains,
    /// Vim / Neovim
    Vim,
    /// Emacs
    Emacs,
    /// 通用 LSP 客户端
    LspClient,
    /// 未知编辑器
    Unknown,
}

impl EditorType {
    /// 从环境变量检测编辑器类型
    pub fn detect() -> Self {
        // 检查环境变量
        if std::env::var("VSCODE_PID").is_ok() || std::env::var("VSCODE_CWD").is_ok() {
            return Self::VSCode;
        }

        if std::env::var("INTELLIJ_IDEA").is_ok() || std::env::var("PYCHARM_IDE").is_ok() {
            return Self::JetBrains;
        }

        if std::env::var("VIMRUNTIME").is_ok() || std::env::var("NVIM").is_ok() {
            return Self::Vim;
        }

        if std::env::var("EMACS").is_ok() || std::env::var("INSIDE_EMACS").is_ok() {
            return Self::Emacs;
        }

        // 检查进程
        #[cfg(target_os = "linux")]
        {
            if let Ok(output) = std::process::Command::new("ps")
                .args(&["-p", &std::process::id().to_string(), "-o", "comm="])
                .output()
            {
                let process_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if process_name.contains("code") {
                    return Self::VSCode;
                } else if process_name.contains("idea") || process_name.contains("pycharm") {
                    return Self::JetBrains;
                } else if process_name.contains("vim") || process_name.contains("nvim") {
                    return Self::Vim;
                } else if process_name.contains("emacs") {
                    return Self::Emacs;
                }
            }
        }

        Self::Unknown
    }

    /// 获取编辑器显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::VSCode => "Visual Studio Code",
            Self::JetBrains => "JetBrains IDE",
            Self::Vim => "Vim/Neovim",
            Self::Emacs => "Emacs",
            Self::LspClient => "LSP Client",
            Self::Unknown => "Unknown Editor",
        }
    }

    /// 获取编辑器配置文件扩展名
    pub fn config_extension(&self) -> &'static str {
        match self {
            Self::VSCode => "json",
            Self::JetBrains => "xml",
            Self::Vim => "vim",
            Self::Emacs => "el",
            Self::LspClient => "json",
            Self::Unknown => "txt",
        }
    }
}

/// 编辑器集成管理器
pub struct EditorIntegrationManager {
    editor_type: EditorType,
    integrations: Vec<Box<dyn EditorIntegration>>,
    config: EditorConfig,
}

impl EditorIntegrationManager {
    /// 创建新的编辑器集成管理器
    pub fn new() -> Self {
        let editor_type = EditorType::detect();
        let mut manager = Self {
            editor_type,
            integrations: Vec::new(),
            config: EditorConfig::default(),
        };

        // 根据编辑器类型添加集成
        match editor_type {
            EditorType::VSCode => manager.integrations.push(Box::new(VSCodeIntegration::new())),
            EditorType::JetBrains => manager.integrations.push(Box::new(JetBrainsIntegration::new())),
            EditorType::Vim => manager.integrations.push(Box::new(VimIntegration::new())),
            EditorType::Emacs => manager.integrations.push(Box::new(EmacsIntegration::new())),
            EditorType::LspClient => manager.integrations.push(Box::new(LspIntegration::new())),
            EditorType::Unknown => {},
        }

        manager
    }

    /// 创建带自定义配置的编辑器集成管理器
    pub fn with_config(config: EditorConfig) -> Self {
        let mut manager = Self::new();
        manager.config = config;
        manager
    }

    /// 获取编辑器类型
    pub fn editor_type(&self) -> EditorType {
        self.editor_type
    }

    /// 初始化所有集成
    pub async fn init(&mut self) -> Result<()> {
        for integration in &mut self.integrations {
            integration.init(&self.config).await?;
        }
        tracing::info!("Initialized editor integrations for {:?}", self.editor_type);
        Ok(())
    }

    /// 获取支持的编辑器功能
    pub fn supported_features(&self) -> Vec<EditorFeature> {
        self.integrations.iter()
            .flat_map(|integration| integration.supported_features())
            .collect()
    }

    /// 检查是否支持特定功能
    pub fn supports_feature(&self, feature: EditorFeature) -> bool {
        self.integrations.iter()
            .any(|integration| integration.supports_feature(feature))
    }

    /// 执行编辑器命令
    pub async fn execute_command(&self, command: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        for integration in &self.integrations {
            if integration.supports_command(command) {
                return integration.execute_command(command, args).await;
            }
        }
        Err(crate::error::ClaudeError::Editor(format!("Command not supported: {}", command)))
    }

    /// 获取编辑器状态
    pub async fn get_state(&self) -> Result<EditorState> {
        for integration in &self.integrations {
            if integration.supports_feature(EditorFeature::StateQuery) {
                return integration.get_state().await;
            }
        }
        Ok(EditorState::default())
    }

    /// 更新编辑器配置
    pub async fn update_config(&mut self, config: EditorConfig) -> Result<()> {
        self.config = config;
        for integration in &mut self.integrations {
            integration.update_config(&self.config).await?;
        }
        Ok(())
    }
}

impl Default for EditorIntegrationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 初始化编辑器集成系统
pub async fn init_editor_integration() -> Result<EditorIntegrationManager> {
    let mut manager = EditorIntegrationManager::new();
    manager.init().await?;
    Ok(manager)
}