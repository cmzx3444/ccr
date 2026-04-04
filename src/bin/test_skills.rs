//! 技能系统测试程序

use claude_code_rs::skills;
use claude_code_rs::editor_compat;
use claude_code_rs::skills::executor::SkillContextBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== Claude Code Rust 技能系统测试 ===\n");

    // 测试1: 初始化技能系统
    println!("1. 初始化技能系统...");
    let manager = skills::init().await?;
    println!("   技能系统初始化成功，共加载 {} 个技能", manager.registry().len().await);

    // 测试2: 列出所有技能
    println!("\n2. 列出所有可用技能...");
    let all_skills = manager.registry().names().await;
    for skill_name in &all_skills {
        println!("   - {}", skill_name);
    }
    println!("   共发现 {} 个技能", all_skills.len());

    // 测试3: 执行帮助技能
    println!("\n3. 测试帮助技能...");
    let context = SkillContextBuilder::new().build();
    let result = skills::execute_skill("help", None, context.clone()).await?;
    println!("   技能执行结果: {}", result.success);
    if let Some(output) = result.output.get("text") {
        println!("   输出: {}", output);
    }

    // 测试4: 执行配置检查技能
    println!("\n4. 测试配置检查技能...");
    let result = skills::execute_skill("config-check", None, context.clone()).await?;
    println!("   配置检查结果: {}", result.success);
    if let Some(summary) = result.output.get("summary") {
        println!("   检查总结: {}", summary);
    }

    // 测试5: 测试编辑器集成
    println!("\n5. 测试编辑器集成...");
    match editor_compat::init_editor_integration().await {
        Ok(editor_manager) => {
            let editor_type = editor_manager.editor_type();
            println!("   检测到编辑器: {} ({:?})", editor_type.display_name(), editor_type);

            let features = editor_manager.supported_features();
            println!("   支持的编辑器功能 ({} 个):", features.len());
            for feature in features {
                println!("     - {:?}: {}", feature, feature.description());
            }

            // 测试获取编辑器状态
            if editor_manager.supports_feature(editor_compat::EditorFeature::StateQuery) {
                match editor_manager.get_state().await {
                    Ok(state) => {
                        println!("   编辑器状态:");
                        println!("     - 运行中: {}", state.is_running);
                        println!("     - 版本: {:?}", state.editor_version);
                        println!("     - 扩展数: {}", state.extensions.len());
                    }
                    Err(e) => {
                        println!("   获取编辑器状态失败: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("   编辑器集成初始化失败: {}", e);
            println!("   注意: 这可能是正常的，如果没有检测到支持的编辑器");
        }
    }

    // 测试6: 测试技能搜索
    println!("\n6. 测试技能搜索功能...");
    let search_results = manager.registry().search("help").await;
    println!("   搜索 'help' 找到 {} 个技能:", search_results.len());
    for skill in search_results {
        println!("   - {}: {}", skill.name, skill.description);
    }

    println!("\n=== 测试完成 ===");
    Ok(())
}