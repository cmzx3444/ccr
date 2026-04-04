//! 插件定义
//!
//! 定义插件的基本结构和特质

use std::fmt::Debug;
use std::path::PathBuf;
use crate::error::Result;
use super::traits::{Plugin, PluginMetadata, PluginState, PluginApi, PluginEntryPoint};

/// 动态加载插件
#[derive(Debug)]
pub struct DynamicPlugin {
    /// 插件路径
    path: PathBuf,
    /// 插件元数据
    metadata: PluginMetadata,
    /// 插件状态
    state: PluginState,
    /// 插件句柄（已加载的库）
    handle: Option<libloading::Library>,
    /// 插件实例（由插件库创建）
    plugin_instance: Option<Box<dyn Plugin>>,
}

impl DynamicPlugin {
    /// 创建新的动态插件
    pub fn new(path: PathBuf, metadata: PluginMetadata) -> Self {
        Self {
            path,
            metadata,
            state: PluginState::Unloaded,
            handle: None,
            plugin_instance: None,
        }
    }
}

#[async_trait::async_trait]
impl Plugin for DynamicPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
    
    fn state(&self) -> PluginState {
        self.state
    }
    
    async fn initialize(&mut self, api: PluginApi) -> Result<()> {
        self.state = PluginState::Loading;

        // 加载插件库
        let handle = unsafe {
            libloading::Library::new(&self.path)?
        };

        // 查找并调用入口点函数
        let entry_point_name = std::ffi::CString::new(self.metadata.entry_point.as_bytes())?;
        let plugin_entry: libloading::Symbol<PluginEntryPoint> = unsafe {
            handle.get(entry_point_name.as_bytes())?
        };

        // 调用入口点获取插件实例
        let plugin_ptr = plugin_entry();
        if plugin_ptr.is_null() {
            return Err(crate::error::ClaudeError::Other("Plugin entry point returned null".to_string()));
        }

        let mut plugin_instance = unsafe { Box::from_raw(plugin_ptr) };

        // 初始化插件实例
        plugin_instance.initialize(api).await?;

        // 存储句柄和实例
        self.handle = Some(handle);
        self.plugin_instance = Some(plugin_instance);
        self.state = PluginState::Loaded;

        Ok(())
    }
    
    async fn start(&mut self) -> Result<()> {
        if let Some(instance) = &mut self.plugin_instance {
            instance.start().await?;
            self.state = PluginState::Running;
        }
        Ok(())
    }
    
    async fn stop(&mut self) -> Result<()> {
        if let Some(instance) = &mut self.plugin_instance {
            instance.stop().await?;
            self.state = PluginState::Loaded;
        }
        Ok(())
    }
    
    async fn unload(&mut self) -> Result<()> {
        self.state = PluginState::Unloading;

        // 先停止插件
        if let Some(instance) = &mut self.plugin_instance {
            let _ = instance.stop().await;
            let _ = instance.unload().await;
        }

        // 释放插件实例（将所有权返回给插件库以便清理）
        self.plugin_instance = None;

        // 卸载库
        self.handle = None;

        self.state = PluginState::Unloaded;
        Ok(())
    }
}
