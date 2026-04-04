//! 插件系统
//!
//! 设计并实现基于Rust的安全插件架构，支持动态加载/卸载ELF格式插件

pub mod manager;
pub mod plugin;
pub mod message_bus;
pub mod lifecycle;
pub mod dependency;
pub mod traits;
pub mod isolation;
pub mod security;

pub use manager::PluginManager;
pub use plugin::DynamicPlugin;
pub use traits::{Plugin, PluginApi, PluginState, PluginMetadata, PluginConfig, PluginEntryPoint, capabilities};
pub use message_bus::MessageBus;
pub use lifecycle::PluginLifecycle;
pub use dependency::PluginDependency;
pub use isolation::{PluginSandbox, IsolationConfig, SandboxViolation};
pub use security::{PluginSignatureVerifier, create_default_verifier, CLAUDE_CODE_OFFICIAL_PUBLIC_KEY};
