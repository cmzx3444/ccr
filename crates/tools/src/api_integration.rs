//! API工具调用集成
//!
//! 提供本地工具系统与Claude API工具调用之间的桥梁。
//! 仅在启用`api-tool-use`特性时可用。

#[cfg(feature = "api-tool-use")]
use crate::registry::{ToolRegistry, ToolManager};
#[cfg(feature = "api-tool-use")]
use crate::types::{ApiToolDefinition, ApiToolCall, ApiToolResult, ToolUseContext};
#[cfg(feature = "api-tool-use")]
use anyhow::Result;
#[cfg(feature = "api-tool-use")]
use api_client::error::ApiError;
#[cfg(feature = "api-tool-use")]
use std::sync::Arc;

/// 工具注册表适配器
///
/// 将`ToolRegistry`适配为API客户端的`ToolCallHandler`
#[cfg(feature = "api-tool-use")]
pub struct ToolRegistryAdapter {
    /// 工具注册表
    registry: Arc<ToolRegistry>,
    /// 缓存的工具定义
    tool_definitions: Vec<api_client::tool_use::ToolDefinition>,
}

#[cfg(feature = "api-tool-use")]
impl ToolRegistryAdapter {
    /// 创建新的工具注册表适配器（同步）
    ///
    /// 注意：工具定义将为空，需要调用`refresh_tool_definitions`异步加载
    pub fn new(registry: Arc<ToolRegistry>) -> Self {
        Self {
            registry,
            tool_definitions: Vec::new(),
        }
    }

    /// 异步创建新的工具注册表适配器，并加载工具定义
    pub async fn new_async(registry: Arc<ToolRegistry>) -> Result<Self> {
        let tool_definitions = Self::load_tool_definitions(&registry).await?;
        Ok(Self {
            registry,
            tool_definitions,
        })
    }

    /// 加载工具定义
    async fn load_tool_definitions(
        registry: &ToolRegistry,
    ) -> Result<Vec<api_client::tool_use::ToolDefinition>> {
        let local_defs = registry.api_tool_definitions().await;
        Ok(local_defs.into_iter().map(|def| {
            api_client::tool_use::ToolDefinition {
                name: def.name,
                description: def.description,
                input_schema: def.input_schema,
            }
        }).collect())
    }

    /// 刷新缓存的工具定义
    pub async fn refresh_tool_definitions(&mut self) -> Result<()> {
        self.tool_definitions = Self::load_tool_definitions(&self.registry).await?;
        Ok(())
    }

    /// 获取工具注册表
    pub fn registry(&self) -> &Arc<ToolRegistry> {
        &self.registry
    }

    /// 获取缓存的工具定义
    pub fn tool_definitions(&self) -> &[api_client::tool_use::ToolDefinition] {
        &self.tool_definitions
    }

    /// 将API客户端的工具调用转换为本地工具调用
    fn convert_tool_call(
        &self,
        tool_call: api_client::tool_use::ToolCall,
    ) -> ApiToolCall {
        ApiToolCall::new(
            tool_call.id,
            tool_call.name,
            tool_call.input,
        )
    }

    /// 将本地工具结果转换为API客户端工具结果
    fn convert_tool_result(
        &self,
        result: ApiToolResult,
    ) -> api_client::tool_use::ToolResult {
        api_client::tool_use::ToolResult {
            tool_use_id: result.tool_use_id,
            content: result.content,
            is_error: result.is_error.unwrap_or(false),
        }
    }
}

#[cfg(feature = "api-tool-use")]
#[async_trait::async_trait]
impl api_client::tool_use::ToolCallHandler for ToolRegistryAdapter {
    async fn handle_tool_call(
        &self,
        tool_call: api_client::tool_use::ToolCall,
    ) -> std::result::Result<api_client::tool_use::ToolResult, ApiError> {
        // 转换为本地工具调用
        let api_tool_call = self.convert_tool_call(tool_call);
        let context = ToolUseContext::new(std::path::PathBuf::from("."));

        // 通过工具注册表执行
        let result = self.registry.handle_api_tool_call(api_tool_call, context).await
            .map_err(|e| ApiError::tool_call(format!("{}", e)))?;

        // 转换为API客户端工具结果
        Ok(self.convert_tool_result(result))
    }

    fn get_tools(&self) -> Vec<api_client::tool_use::ToolDefinition> {
        self.tool_definitions.clone()
    }
}

/// 工具管理器适配器
///
/// 将`ToolManager`适配为API客户端的`ToolCallHandler`
#[cfg(feature = "api-tool-use")]
pub struct ToolManagerAdapter {
    /// 工具管理器
    manager: Arc<ToolManager>,
    /// 缓存的工具定义
    tool_definitions: Vec<api_client::tool_use::ToolDefinition>,
}

#[cfg(feature = "api-tool-use")]
impl ToolManagerAdapter {
    /// 创建新的工具管理器适配器（同步）
    ///
    /// 注意：工具定义将为空，需要调用`refresh_tool_definitions`异步加载
    pub fn new(manager: Arc<ToolManager>) -> Self {
        Self {
            manager,
            tool_definitions: Vec::new(),
        }
    }

    /// 异步创建新的工具管理器适配器，并加载工具定义
    pub async fn new_async(manager: Arc<ToolManager>) -> Result<Self> {
        let tool_definitions = Self::load_tool_definitions(&manager).await?;
        Ok(Self {
            manager,
            tool_definitions,
        })
    }

    /// 加载工具定义
    async fn load_tool_definitions(
        manager: &ToolManager,
    ) -> Result<Vec<api_client::tool_use::ToolDefinition>> {
        let local_defs = manager.api_tool_definitions().await;
        Ok(local_defs.into_iter().map(|def| {
            api_client::tool_use::ToolDefinition {
                name: def.name,
                description: def.description,
                input_schema: def.input_schema,
            }
        }).collect())
    }

    /// 刷新缓存的工具定义
    pub async fn refresh_tool_definitions(&mut self) -> Result<()> {
        self.tool_definitions = Self::load_tool_definitions(&self.manager).await?;
        Ok(())
    }

    /// 获取工具管理器
    pub fn manager(&self) -> &Arc<ToolManager> {
        &self.manager
    }

    /// 获取缓存的工具定义
    pub fn tool_definitions(&self) -> &[api_client::tool_use::ToolDefinition] {
        &self.tool_definitions
    }
}

#[cfg(feature = "api-tool-use")]
#[async_trait::async_trait]
impl api_client::tool_use::ToolCallHandler for ToolManagerAdapter {
    async fn handle_tool_call(
        &self,
        tool_call: api_client::tool_use::ToolCall,
    ) -> std::result::Result<api_client::tool_use::ToolResult, ApiError> {
        // 转换为本地工具调用
        let api_tool_call = ApiToolCall::new(
            tool_call.id,
            tool_call.name,
            tool_call.input,
        );
        let context = ToolUseContext::new(std::path::PathBuf::from("."));

        // 通过工具管理器执行
        let result = self.manager.handle_api_tool_call(api_tool_call, context).await
            .map_err(|e| ApiError::tool_call(format!("{}", e)))?;

        // 转换为API客户端工具结果
        Ok(api_client::tool_use::ToolResult {
            tool_use_id: result.tool_use_id,
            content: result.content,
            is_error: result.is_error.unwrap_or(false),
        })
    }

    fn get_tools(&self) -> Vec<api_client::tool_use::ToolDefinition> {
        self.tool_definitions.clone()
    }
}

/// 为API客户端创建工具调用处理器的工厂函数（异步）
#[cfg(feature = "api-tool-use")]
pub async fn create_tool_registry_handler_async(
    registry: Arc<ToolRegistry>,
) -> Result<Arc<dyn api_client::tool_use::ToolCallHandler>> {
    let adapter = ToolRegistryAdapter::new_async(registry).await?;
    Ok(Arc::new(adapter))
}

/// 为API客户端创建工具管理器处理器的工厂函数（异步）
#[cfg(feature = "api-tool-use")]
pub async fn create_tool_manager_handler_async(
    manager: Arc<ToolManager>,
) -> Result<Arc<dyn api_client::tool_use::ToolCallHandler>> {
    let adapter = ToolManagerAdapter::new_async(manager).await?;
    Ok(Arc::new(adapter))
}

/// 为API客户端创建工具调用处理器的工厂函数（同步，工具定义为空）
#[cfg(feature = "api-tool-use")]
pub fn create_tool_registry_handler(
    registry: Arc<ToolRegistry>,
) -> Arc<dyn api_client::tool_use::ToolCallHandler> {
    Arc::new(ToolRegistryAdapter::new(registry))
}

/// 为API客户端创建工具管理器处理器的工厂函数（同步，工具定义为空）
#[cfg(feature = "api-tool-use")]
pub fn create_tool_manager_handler(
    manager: Arc<ToolManager>,
) -> Arc<dyn api_client::tool_use::ToolCallHandler> {
    Arc::new(ToolManagerAdapter::new(manager))
}