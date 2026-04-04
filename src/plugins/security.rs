//! 插件安全模块
//!
//! 实现插件签名验证和安全检查

use std::path::PathBuf;
use crate::error::Result;

/// 插件签名验证器
#[derive(Debug, Clone)]
pub struct PluginSignatureVerifier {
    /// 验证公钥（base64编码）
    public_keys: Vec<String>,
}

impl PluginSignatureVerifier {
    /// 创建新的签名验证器
    pub fn new() -> Self {
        Self {
            public_keys: Vec::new(),
        }
    }

    /// 添加公钥
    pub fn add_public_key(&mut self, public_key: String) {
        self.public_keys.push(public_key);
    }

    /// 验证插件签名
    pub async fn verify_plugin(&self, plugin_path: &PathBuf, metadata: &super::PluginMetadata) -> Result<bool> {
        // 如果没有签名，检查是否允许未签名插件
        let signature = match &metadata.signature {
            Some(sig) => sig,
            None => {
                // TODO: 从配置中检查是否允许未签名插件
                return Ok(true); // 暂时允许未签名插件
            }
        };

        // 解码base64签名
        let signature_bytes = match base64::decode(signature) {
            Ok(bytes) => bytes,
            Err(e) => {
                return Err(crate::error::ClaudeError::Other(
                    format!("Failed to decode signature: {}", e)
                ));
            }
        };

        // 计算插件文件的哈希
        let plugin_content = match tokio::fs::read(plugin_path).await {
            Ok(content) => content,
            Err(e) => {
                return Err(crate::error::ClaudeError::Io(e));
            }
        };

        // 验证元数据
        let metadata_json = serde_json::to_string(metadata)
            .map_err(|e| crate::error::ClaudeError::Other(e.to_string()))?;

        // 合并插件内容和元数据作为验证数据
        let mut verification_data = Vec::new();
        verification_data.extend(plugin_content);
        verification_data.extend(metadata_json.as_bytes());

        // 尝试验证每个公钥
        for public_key in &self.public_keys {
            let public_key_bytes = match base64::decode(public_key) {
                Ok(bytes) => bytes,
                Err(_) => continue, // 跳过无效的公钥
            };

            // TODO: 实现ed25519验证
            // 暂时跳过实际的签名验证，只检查签名格式
            if signature_bytes.len() == 64 { // ed25519签名通常是64字节
                return Ok(true);
            }
        }

        Err(crate::error::ClaudeError::Permission(
            "Plugin signature verification failed".to_string()
        ))
    }

    /// 验证插件元数据签名
    pub fn verify_metadata(&self, metadata: &super::PluginMetadata) -> Result<bool> {
        // 简化版本：只检查签名是否存在和格式是否正确
        match &metadata.signature {
            Some(signature) => {
                if signature.len() >= 64 { // 粗略检查base64编码的ed25519签名长度
                    Ok(true)
                } else {
                    Err(crate::error::ClaudeError::Permission(
                        "Invalid signature format".to_string()
                    ))
                }
            }
            None => {
                // TODO: 检查是否允许未签名插件
                Ok(true)
            }
        }
    }

    /// 生成测试密钥对
    #[cfg(feature = "test-keys")]
    pub fn generate_test_keypair() -> (String, String) {
        use ed25519_dalek::{Keypair, Signer};
        use rand::rngs::OsRng;

        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        let public_key = base64::encode(keypair.public.to_bytes());
        let private_key = base64::encode(keypair.secret.to_bytes());

        (public_key, private_key)
    }
}

/// 默认的Claude Code官方公钥（示例）
pub const CLAUDE_CODE_OFFICIAL_PUBLIC_KEY: &str = "AAAAA1234567890EXAMPLEKEY==";

/// 创建带有默认公钥的验证器
pub fn create_default_verifier() -> PluginSignatureVerifier {
    let mut verifier = PluginSignatureVerifier::new();
    verifier.add_public_key(CLAUDE_CODE_OFFICIAL_PUBLIC_KEY.to_string());
    verifier
}